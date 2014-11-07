#![allow(dead_code)]
#![feature(tuple_indexing)]
#![feature(slicing_syntax)]
#![feature(unboxed_closures)]
#![feature(unboxed_closure_sugar)]

extern crate collections;
extern crate regex;
extern crate serialize;
extern crate hyper;

use pile::{GenPile, GenPileKeys, DualPile, LandPile, ColoredPile};
use std::os;
use table::{Table, LStr, RStr, Int, UInt, Empty};
use mtgjson::test_read;
//use interval::closed;

mod prob;
mod pile;
mod standard;
mod table;
mod perm;
mod interval;
mod mana;
mod colors;
mod mtgjson;

//
// Mulligan Rule: 
//
//  - 7 cards: Mulligan 0, 1, 6 or 7 lands (2, 3, 4 or 5 spells)
//  - 6 cards: Mulligan 0, 1, 5 or 6 lands (2, 3 or 4 spells)
//  - 5 cards: Mulligan 0 or 5 lands (1, 2, 3 or 4 spells)
//  - 4 cards: Always kept
//

fn mull_rule(hand_size: uint) -> (uint, uint) {
    match hand_size {
        7 => (2u, 5u),
        6 => (2u, 4u),
        5 => (1u, 4u),
        4 => (0u, 4u),
        _ => panic!("Eh")
    }
}

mod single {
    use pile::{KvMap, LandPile, ColoredPile};
    use std::iter::{range_inclusive, AdditiveIterator};
    use prob;
    use interval::closed;

    fn draw(hand: &ColoredPile, draws: uint, deck: &ColoredPile, 
            goal: |&ColoredPile|->bool) -> f64 
    {
        if draws > 0 {
            ColoredPile::iter(draws)
                .filter(|draw| deck.has(draw) && goal(&(hand.add(draw))))
                .map(|draw| deck.prob_draw(&draw))
                .sum()
        } else {
            prob::cond(goal(hand))
        }
    }        

    fn intern(hand_size: uint, 
              deck: &ColoredPile, draws: uint, goal: |&ColoredPile|->bool)
              -> (f64, f64) {
        let (lands_min, lands_max) = ::mull_rule(hand_size);
        
        // Probability of keeping 
        let keep = range_inclusive(lands_min, lands_max)
            .map(|lands| deck.prob_land(lands, hand_size - lands))
            .sum();
        
        // Probability of casting (where we auto-fail if we don't have the lands)
        let cast = ColoredPile::iter(hand_size)
            .filter(|hand| hand.lands() >= lands_min && hand.lands() <= lands_max)
            .filter(|hand| deck.has(hand))
            .map(|hand| (deck.prob_draw(&hand) * 
                         draw(&hand, draws, &deck.sub(&hand), |g| goal(g))))
            .sum();

        // So cast * keep = chance of reaching goals, *given* no mulligan.
        (keep, cast)
    }

    pub fn turn0(deck: &ColoredPile, draws: uint, goal: |&ColoredPile|->bool) -> f64 {
        let mut mull = 1.0; // the chance we mulled before
        let mut succ = 0.0;
        
        for hand_size in range_inclusive(4u, 7u).rev() {
            let (keep, cast) = intern(hand_size, deck, draws, |g| goal(g));
            succ += mull * (cast * keep); 
            mull *= 1.0 - keep;
        }
        
        succ
    }
    
    pub fn cards(lands: uint, deck: uint, draws: uint, perc: f64, 
                 goal: |&ColoredPile|->bool) -> 
        int 
    {
        let deck1 = ColoredPile::new(lands, 0, deck-lands);
        let r1 = turn0(&deck1, draws, |g| goal(g));
        
        for k in closed(0, lands).iter() {
            let deck0 = ColoredPile::new(k, lands-k, deck-lands);
            let r0 = turn0(&deck0, draws, |g| goal(g));
            if r0 >= perc * r1 {
                return k as int
            }
        }
        return 0
    }        
}

// ================================================================================

mod dual {
    use pile::{KvMap, LandPile, DualPile};
    use std::iter::{range_inclusive, AdditiveIterator};
    use prob;
    use interval::closed;

    fn draw(hand: &DualPile, draws: uint, deck: &DualPile, 
            goal: |&DualPile|->bool) -> f64 
    {
        if draws > 0 {
            DualPile::iter(draws)
                .filter(|draw| deck.has(draw) && goal(&hand.add(draw)))
                .map(|draw| deck.prob_draw(&draw))
                .sum()
        } else {
            prob::cond(goal(hand))
        }
    }        

    pub fn turn0(deck: &DualPile, draws: uint, goal: |&DualPile|->bool) -> f64 {
        let mut mull = 1.0;
        let mut succ = 0.0;
        //let mut tally = 0.0;

        for hand_size in range_inclusive(4u, 7u).rev() {
            let (lands_min, lands_max) = ::mull_rule(hand_size);
            
            // Probability of keeping 
            let keep = range_inclusive(lands_min, lands_max)
                .map(|lands| deck.prob_land(lands, hand_size - lands))
                .sum();
            
            // Probability of casting
            let cast = DualPile::iter(hand_size)
                .filter(|hand| hand.lands() >= lands_min && hand.lands() <= lands_max)
                .filter(|hand| deck.has(hand))
                .map(|hand| deck.prob_draw(&hand) * draw(&hand, draws, &deck.sub(&hand), 
                                                 |g| goal(g)))
                .sum();

            succ += mull * (cast * keep); 
            mull *= 1.0 - keep;
        }
        
        succ
    }
    
    pub fn cards(lands: uint, deck: uint, uncolored: uint,
                 a_rate: f64, draws: uint, perc: f64, goal: |&DualPile|->bool) -> int {
        
        let deck0 = DualPile::new(0, 0, lands, 0, deck-lands);
        let deck1 = DualPile::new(0, 0, lands-uncolored, uncolored, deck-lands);
        
        let r0 = turn0(&deck0, draws, |g| goal(g));
        let r1 = turn0(&deck1, draws, |g| goal(g));
        
        if r1 < perc * r0 {
            return -1
        }
        
        for ab in closed(0u, lands).iter() {
            let mono = lands - ab - uncolored;
            let a = ((mono as f64) * a_rate + 0.5).round() as uint;
            let b = mono - a;
            
            assert!(a+b+ab+uncolored+(deck-lands) == deck);
            
            let deck0 = DualPile::new(a, b, ab, uncolored, deck-lands);
            let r = turn0(&deck0, draws, |g| goal(g));
            if r >= perc * r0 {
                return ab as int
            }
        }
        return -1
    }        
}

// ================================================================================

mod gen {
    use pile::{KvMap, GenPileKeys, LandPile, GenPile};
    use std::iter::{range_inclusive, AdditiveIterator};
    use prob;

    fn draw(hand: &GenPile<GenPileKeys>, draws: uint, 
            deck: &GenPile<GenPileKeys>, 
            goal: |&GenPile<GenPileKeys>|->bool) -> f64 
    {
        if draws > 0 {
            deck.subsets(draws).iter()
                .filter(|&draw| goal(&hand.add(draw)))
                .map(|draw| deck.prob_draw(draw))
                .sum()
        } else {
            prob::cond(goal(hand)) 
        }
    }        

    pub fn turn0(deck: &GenPile<GenPileKeys>, draws: uint, 
                 goal: |&GenPile<GenPileKeys>|->bool) -> f64 {
        let mut mull = 1.0;
        let mut succ = 0.0;
        
        for hand_size in range_inclusive(4u, 7u).rev() {
            let (lands_min, lands_max) = ::mull_rule(hand_size);
            
            // Probability of keeping 
            let keep = range_inclusive(lands_min, lands_max)
                .map(|lands| deck.prob_land(lands, hand_size - lands))
                .sum();
            
            // Probability of casting
            let cast = deck.subsets(hand_size).iter()
                .filter(|hand| hand.lands() >= lands_min && hand.lands() <= lands_max)
                .map(|hand| {
                    let d0 = deck.prob_draw(hand);
                    let p0 = {
                        let r = deck.sub(hand);
                        draw(hand, draws, &r, |g| goal(g))
                    };
                    d0 * p0
                })
                .sum();
            
            succ += mull * cast;
            mull *= 1.0 - keep;
        }
        
        succ
    }
}

#[bench]
pub fn b_minc(bh: &mut BenchHarness) {
    let l = 28;
    let d = 60;
    let draws = 3;
    let pc = 0.9;
    
    let goal = |hand: &ColoredPile| { 
        hand.colored >= 3 && hand.lands() >= 3 /*&& hand.turn >= 3*/
    };
    bh.iter(|| single::cards(l, d, draws, pc, |h| goal(h)))
}

fn pm2(a:uint, b:uint, c:uint) -> String {
    let mut res = if c > 0 { c.to_string() } else { "".to_string() };
    res.push_str("A".repeat(a).as_slice());
    res.push_str("B".repeat(b).as_slice());
    res
}

// Summary of [lands] lands in a [D] card deck
fn summary(lands: uint, deck: uint, uncolored_lands: uint) {
    use interval::closed;
    
    let mut table = Table::new(5, 9);
    
    {
        table.set(0, 0, LStr(format!("{}/{}({})", lands, deck, uncolored_lands)));
        table.set(0, 1, RStr("--".to_string()));
        for cless in closed(1u, 7).iter() { 
            table.set(0, 1u + cless, UInt(cless)) 
        }
    }
    
    for cmana in closed(2u, 5u).iter() {
        for bmana in closed(1, cmana/2).iter() {
            let amana = cmana - bmana;
            
            let gstr = pm2(amana, bmana, cmana - amana - bmana);
            table.set(cmana-1, 0, RStr(gstr));   
            
            for cless in closed(0u, 7).iter() { 
                
                let arate = (amana as f64) / (amana + bmana) as f64;
                let cmc = cmana + cless;
                let draws = cmc - 1;
                let goal = |hand: &DualPile| { 
                    
                    let a_left = if amana > hand.a { amana - hand.a } else { 0 };
                    let b_left = if bmana > hand.b { bmana - hand.b } else { 0 };
                    
                    let ok = (a_left + b_left) <= hand.ab 
                        && hand.lands() >= cmc; // enough lands for cmc
                    
                    //let gstr = pm2(amana, bmana, cmana - amana - bmana);
                    //if ok { println!("{}/{}: {}\n", gstr, draws, hand) };
                    ok
                };
                let res = dual::cards(lands, deck, uncolored_lands, arate, draws, 0.90, goal);
                
                table.set(cmana-1, cless + 1, 
                          if res == 0 { Empty } 
                          //else if res == (lands - uncolored_lands) as int { RStr("**") }
                          else if res == -1 { RStr("**".to_string()) }
                                  else { Int(res) })
            }
        }
    }
    
    println!("");
    table.print(format!("{} lands, {} colorless", lands, uncolored_lands).as_slice());
}

fn pm(colored_mana:uint, cmc:uint) -> String {
    let nc = cmc - colored_mana;
    let mut res = if nc > 0 { nc.to_string() } else { "".to_string() };
    res.push_str("C".repeat(colored_mana).as_slice());
    res
}  

pub fn summary_c(lands: uint, deck: uint) {
    use interval::closed;

    // Making my adjusted tables
    let mut table = Table::new(5, 9);
    
    {
        table.set(0, 0, LStr(format!("{}/{}", lands, deck)));
        table.set(0, 1, RStr("--".to_string()));
        for cless in closed(1i, 7).iter() { 
            table.set(0, (1 + cless) as uint, Int(cless)) 
        };
    }
    
    for cmana in closed(1u, 4u).iter() {
        let gstr = pm(cmana, cmana);
        table.set(cmana, 0, RStr(gstr));                
        
        for cless in closed(0u, 7).iter() {                     
            let cmc = cmana + cless;
            let draws = cmc - 1;
            let goal = |hand: &ColoredPile| { 
                let ok = hand.colored() >= cmana // colors okay
                    && hand.lands() >= cmc; // enough lands for cmc
                
                ok
            };
            let res = single::cards(lands, deck, draws, 0.90, goal);
            table.set(cmana, 1u+cless,
                      if res == 0 { Empty } 
                      //else if res == (lands - uncolored_lands) as int { RStr("**") }
                      else if res == -1 { RStr("**".to_string()) }
                      else { Int(res) })
        }
    }
    
    println!("");
    table.print(format!("{} lands", lands).as_slice());
}

fn investigate()
{
    #[inline(always)]
    fn min(a:uint, b:uint) -> uint { if a < b { a } else { b } }
    
    #[inline(always)]
    fn is_land(idx: uint) -> bool { idx == 0 || idx == 1 || idx == 2 }
    
    fn can_cast(la:uint, lb:uint, lab:uint, lx:uint,
                a: uint, b: uint, x: uint) -> bool {
        // tap for A
        let ta = min(la, a);
        let (la, a) = (la - ta, a - ta);
        
        // tap for B
        let tb = min(lb, b);
        let (lb, b) = (lb - tb, b - tb);
        
        // tap for A or B
        let ab = a+b;
        let tab = min(lab, ab);
        let (lab, ab) = (lab - tab, ab - tab);
        
        if ab > 0 { false }
        else {
            // tap for X
            x <= lx + la + lb + lab
        }
    }
    
    {                
        assert!( can_cast(1, 0, 1, 0,   1, 0, 0));
        assert!( can_cast(1, 0, 1, 0,   0, 1, 0));
        assert!( can_cast(1, 0, 1, 0,   0, 0, 1));
        assert!( can_cast(1, 0, 1, 0,   2, 0, 0));
        assert!( can_cast(1, 0, 1, 0,   1, 1, 0));
        assert!(!can_cast(1, 0, 1, 0,   0, 2, 0));
        assert!( can_cast(1, 0, 1, 0,   1, 0, 1));
        assert!( can_cast(1, 0, 1, 0,   0, 1, 1));
        assert!( can_cast(1, 0, 1, 0,   0, 0, 2));
        assert!(!can_cast(1, 0, 1, 0,   1, 1, 1));
        assert!(!can_cast(1, 0, 1, 0,   0, 1, 2));
        assert!(!can_cast(1, 0, 1, 0,   1, 0, 2));
        assert!(!can_cast(1, 0, 1, 0,   0, 0, 3));
    }
    
    {
        use std::iter::range_inclusive;
        
        static A  :uint = 0;
        static B  :uint = 1;
        static C  :uint = 2;
        static AB :uint = 3;
        static BC :uint = 4;
        static AC :uint = 5;
        static S1 :uint = 6;
        static S2 :uint = 7;
        static O  :uint = 8;
        
        // a,b,c,ab,bc,ac,s,o
        fn is_land(idx: uint) -> bool { idx < 6 }

        let info = GenPileKeys::new(9, is_land);
        
        fn cc(hand: &GenPile<GenPileKeys>, a: uint, b: uint, x: uint) -> bool {
            can_cast(hand[A] + hand[AC], hand[B] + hand[BC], hand[AB], hand[C],
                     a, b, x)
        }
        
        let turn = 3;

        for cmc2s in range_inclusive(2u, 16) {
            for s1 in range_inclusive(1u, cmc2s-1) {
                let s2 = cmc2s - s1;
                
                let mut best_p = 0.0;
                let mut best_a = 0u;
                
                for a in range_inclusive(0u, 17) {
                    let b = 17 - a;
                    
                    let deck = GenPile::new(vec![a, b, 0,
                                                 0, 0, 0,
                                                 s1, s2, 23-s1-s2], info);
                    let p_base = gen::turn0(&deck, turn, 
                                            |hand : _| { 
                                                hand.lands() >= turn && hand[S1] + hand[S2] > 0
                                            });
                    let p_succ = gen::turn0(&deck, turn, 
                                            |hand| { 
                                                (cc(hand, 2, 0, turn-2) && hand[S1] > 0) ||
                                                    (cc(hand, 0, 2, turn-2) && hand[S2] > 0)
                                            });
                    
                    let p_rel = p_succ / p_base;
                    
                    if p_rel >= best_p {
                        best_p = p_rel;
                        best_a = a;
                    }
                }

                println!("{:2},{:2} : {:2},{:2} {:6.2}%", 
                         s1, s2,
                         best_a, 17 - best_a,
                         best_p * 100.0)
            }
            println!("----------------------");
        }
    }
}

// Making the Frank 1 colored mana table:
fn frank(colored_mana: uint, cmc: uint) -> Table {
    use interval::closed;

    let mut t1 = Table::new(4, 8);
    
    let ps = pm(colored_mana, cmc);
    
    t1.set(0, 0, LStr(ps));
    
    for turn in closed(1i, 7).iter() { t1.set(0u, turn as uint, Int(turn)) };

    let manas = vec!(16u, 17, 18);
    let mut lines = manas.iter().map(|l| {
        let f = 10.0f64;
        (40u, *l, 0u, (f-1.0)/f) 
    }).enumerate();
    
    for (line_no, line) in lines {
        let (d, l, e, pc) = line;
        let sym = if e == 0u {'p'} else {'d'};
        t1.set(1u+line_no, 0u, LStr(format!("{} lands {}", l as uint, sym)));
        
        for turn in closed(1u, 7u).iter() {
            let draws = turn - 1u + e;
            let goal = |hand: &ColoredPile| { 
                hand.colored() >= colored_mana // colors okay
                    && hand.lands() >= cmc // enough lands for cmc
                    && turn >= cmc // one land per turn
            };
            let res = single::cards(l, d, draws, pc, goal);
            t1.set(1u+line_no, turn, if res == 0i { Empty } else { Int(res) }) 
        }
    };
    
    t1
}

fn frank_table()
{
	frank(1, 1).print("");
	frank(1, 2).print("");
	frank(1, 3).print("");
	frank(1, 4).print("");
	frank(1, 5).print("");
	frank(1, 6).print("");
	frank(1, 7).print("");
	//
	frank(2, 2).print("");
	frank(2, 3).print("");
	frank(2, 4).print("");
	frank(2, 5).print("");
	frank(2, 6).print("");
	frank(2, 7).print("");
	//
	frank(3, 3).print("");
	frank(3, 4).print("");
	frank(3, 5).print("");
	frank(3, 6).print("");
	frank(3, 7).print("");
	//
	frank(7, 7).print("");
}

#[main]
fn main() {
    use interval::closed;

    let args = os::args();

    if args.len() == 1 {
        test_read()
    }
    if args.len() == 2 && args[1].as_slice() == "land"	{
		investigate()
    }
    else if args.len() == 2 && args[1].as_slice() == "duals" {
		let mut dp = Table::new (18, 2);
		for a in closed(0u, 17).iter() {
			let goal = | hand : &DualPile | {(hand.a >= 1) || hand.ab >= 1 };
			let td = DualPile::new (a, 17 - a, 0, 0, 23);
			let rt = dual::turn0(&td, 1, | g | goal(g));
			dp.set(a, 0, LStr(format !("{}", td)));
			dp.set(a, 1, RStr(format !("{:6.2}%", rt * 100.0)));
		}
		dp.print("Duals");
    }
    else if args.len() >= 2 && args[1].as_slice() == "table" {
        let l = if args.len() == 3 { from_str::<uint>(args[2].as_slice()).unwrap_or(0) } else { 0 };
        if l == 0 {
	    for i in closed(16, 18).iter() { summary_c(i, 40); }
	    for i in closed(22, 28).iter() { summary_c(i, 60); }
        }
        else {
            summary_c(l, 60)
        }
    }
    else if args.len() == 2 && args[1].as_slice() == "frank" {
        frank_table()
    }
    else if args.len() == 2 {
        let lands = standard::analyze(args[1].as_slice());
        summary_c(lands, 60);
    }
    else if false {
        let l = 26;
        let d = 60;
        for u in closed(0u, 4u).iter() {
            summary(l, d, u)
        }
    }
    else if args.len() == 2 {
        let lands = standard::analyze(args[1].as_slice());
        summary_c(lands, 60);
    }
    else if args.len() == 3 && args[1].as_slice() == "pow" {
        let a = from_str(args[2].as_slice()).unwrap_or(0);
        for k in closed(0i, 10).iter() {
            println!("{}^{} = {}", a, k, prob::pow(a, k, 1));
        }
    }
    else if args.len() == 4 && args[1].as_slice() == "C" {
        let a:uint = from_str(args[2].as_slice()).unwrap_or(0);
        let b:uint = from_str(args[3].as_slice()).unwrap_or(1);
        
        for n in range(a, b) {
            println!("c({:3u}, {:2u}) = {:60.0f}", 100u, n, prob::c(100, n));
        }
    }
}

