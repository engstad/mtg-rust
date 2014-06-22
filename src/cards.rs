// compile with: rustc --opt-level 3 -Adead_code cards.rs

// #![allow(dead_code)]
#![allow(uppercase_variables)]
// #![allow(unused_imports)]
// #![feature(globs)]
// #![feature(macro_rules)]
// #![feature(simd)]
#![feature(globs)]
#![feature(macro_rules)]

extern crate debug;
extern crate collections;

use pile::{KvMap, GenPile, GenPileKeys, DualPile, LandPile, ColoredPile};
use std::os;
use table::{Table, LStr, RStr, Int, UInt, Empty};

mod prob;
mod pile;
mod standard;
mod table;
mod perm;

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
        _ => fail!("Eh")
    }
}

mod single {
    use pile::{KvMap, LandPile, ColoredPile};
    use std::iter::{range_inclusive, AdditiveIterator};
    use prob;

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
    
    pub fn cards(L: uint, D: uint, draws: uint, perc: f64, goal: |&ColoredPile|->bool) -> 
        int 
    {
        let deck1 = ColoredPile::new(L, 0, D-L);
        let r1 = turn0(&deck1, draws, |g| goal(g));
        
        for k in range(0, L+1) {
            let deck0 = ColoredPile::new(k, L-k, D-L);
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
    
    pub fn cards(L: uint, D: uint, X: uint,
             a_rate: f64, draws: uint, perc: f64, goal: |&DualPile|->bool) -> int {
        
        let deck0 = DualPile::new(0, 0, L, 0, D-L);
        let deck1 = DualPile::new(0, 0, L-X, X, D-L);
        
        let r0 = turn0(&deck0, draws, |g| goal(g));
        let r1 = turn0(&deck1, draws, |g| goal(g));
        
        if r1 < perc * r0 {
            return -1
        }
        
        for ab in range(0u, L+1) {
            let mono = L - ab - X;
            let a = ((mono as f64) * a_rate + 0.5).round() as uint;
            let b = mono - a;
            
            assert!(a+b+ab+X+(D-L) == D);
            
            let deck0 = DualPile::new(a, b, ab, X, D-L);
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


#[main]
fn main() {
    let args = os::args();

    if args.len() < 2 {
        
        if true {

            let cs = ColoredPile::new(2, 0, 0);
            println!("{}", cs.get(pile::C));

            #[inline(always)]
            fn min(a:uint, b:uint) -> uint { if a < b { a } else { b } };
            fn is_land(idx: uint) -> bool { idx == 0 || idx == 1 || idx == 2 };
            fn can_cast(la:uint, lb:uint, lab:uint, 
                        a: uint, b: uint, x: uint) -> bool {
                let ta = min(la, a);
                let (la, a) = (la - ta, a - ta);

                let tb = min(lb, b);
                let (lb, b) = (lb - tb, b - tb);

                let ab = a+b;
                let tab = min(lab, ab);
                let (lab, ab) = (lab - tab, ab - tab);

                if ab > 0 { false }
                else {
                    let lx = la + lb + lab;
                    x <= lx
                }
            }

            {                
                assert!( can_cast(1, 0, 1,   1, 0, 0));
                assert!( can_cast(1, 0, 1,   0, 1, 0));
                assert!( can_cast(1, 0, 1,   0, 0, 1));
                assert!( can_cast(1, 0, 1,   2, 0, 0));
                assert!( can_cast(1, 0, 1,   1, 1, 0));
                assert!(!can_cast(1, 0, 1,   0, 2, 0));
                assert!( can_cast(1, 0, 1,   1, 0, 1));
                assert!( can_cast(1, 0, 1,   0, 1, 1));
                assert!( can_cast(1, 0, 1,   0, 0, 2));
                assert!(!can_cast(1, 0, 1,   1, 1, 1));
                assert!(!can_cast(1, 0, 1,   0, 1, 2));
                assert!(!can_cast(1, 0, 1,   1, 0, 2));
                assert!(!can_cast(1, 0, 1,   0, 0, 3));
            }

            {
                // a,b,c,ab,bc,ac,s,o
                fn is_land(idx: uint) -> bool { idx < 6  };
                let info = GenPileKeys::new(8, is_land);

                let deck = GenPile::new(vec![1,0,0,8,7,8,4,32], info);
                let g0 = |hand:&GenPile<GenPileKeys>| { 
                    can_cast(hand.get(0) + hand.get(3) + hand.get(5), 
                             hand.get(1) + hand.get(2) + hand.get(4),
                             0,
                             2, 0, 1) && hand.get(6) > 0
                };
                let g1 = |hand:&GenPile<GenPileKeys>| { hand.lands() >= 3 };
                let r0 = gen::turn0(&deck, 2, g0);
                let r1 = gen::turn0(&deck, 2, g1);
                println!("P[R]   = {:6.2}% ", r0 * 100.0);
                println!("P[L]   = {:6.2}% ", r1 * 100.0);
                println!("P[R|L] = {:6.2}% ", r0 / r1 * 100.0);
            }
        }

        if false {
            let mut dp = Table::new(18, 2);
            for a in range(0u, 18) {
                let goal = |hand: &DualPile| { (hand.a >= 1) || hand.ab >= 1 };
                let td = DualPile::new(a, 17-a, 0, 0, 23);
                let rt = dual::turn0(&td, 1, |g| goal(g));
                dp.set(a, 0, LStr(format!("{}", td)));
                dp.set(a, 1, RStr(format!("{:6.2}%", rt * 100.0)));
            }
            dp.print("Duals");
        }
            
        fn pl(e:int) -> &str { if e == 0 {"play"} else {"draw"} };

        fn pm(colored_mana:uint, cmc:uint) -> String {
            let nc = cmc - colored_mana;
            (if nc > 0 { nc.to_str() } else { "".to_string() })
                .append("C".repeat(colored_mana).as_slice())
        }  

        fn pm2(a:uint, b:uint, c:uint) -> String {
            (if c > 0 { c.to_str() } else { "".to_string() })
                .append("A".repeat(a).as_slice())
                .append("B".repeat(b).as_slice())
        }

        // Summary of [L] lands in a [D] card deck
        fn summary(L: uint, D: uint, X: uint) {

            let mut table = Table::new(5, 9);

            {
                table.set(0, 0, LStr(format!("{}/{}({})", L, D, X)));
                table.set(0, 1, RStr("--".to_string()));
                for cless in range (1u, 8) { 
                    table.set(0, 1u + cless, UInt(cless)) 
                }
            }

            for cmana in range (2u, 6u) {
                for bmana in range (1, cmana/2+1) {
                    let amana = cmana - bmana;

                    let gstr = pm2(amana, bmana, cmana - amana - bmana);
                    table.set(cmana-1, 0, RStr(gstr));   
                    
                    for cless in range (0u, 8) { 
                        
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
                        let res = dual::cards(L, D, X, arate, draws, 0.90, goal);

                        table.set(cmana-1, cless + 1, 
                                  if res == 0 { Empty } 
                                  //else if res == (L - X) as int { RStr("**") }
                                  else if res == -1 { RStr("**".to_string()) }
                                  else { Int(res) })
                    }
                }
            }

            println!("");
            table.print(format!("{} lands, {} colorless", L, X).as_slice());
        }

        //summary(16, 40);
        //summary(17, 40);
        //summary(18, 40);

        //summary(24, 60);
        if false {
            let l = 26;
            let d = 60;
            for u in range(0u, 4u+1) {
                summary(l, d, u)
            }
        }

        fn summary_c(L: uint, D: uint) {
            // Making my adjusted tables
            let mut table = Table::new(5, 9);

            {
                table.set(0, 0, LStr(format!("{}/{}", L, D)));
                table.set(0, 1, RStr("--".to_string()));
                for cless in range (1, 8) { 
                    table.set(0, (1 + cless) as uint, Int(cless)) 
                };
            }

            for cmana in range (1u, 5u) {
                let gstr = pm(cmana, cmana);
                table.set(cmana, 0, RStr(gstr));                
                    
                for cless in range (0u, 8) {                     
                    let cmc = cmana + cless;
                    let draws = cmc - 1;
                    let goal = |hand: &ColoredPile| { 
                        let ok = hand.colored() >= cmana // colors okay
                            && hand.lands() >= cmc; // enough lands for cmc
                        
                        ok
                    };
                    let res = single::cards(L, D, draws, 0.90, goal);
                    table.set(cmana, 1u+cless,
                              if res == 0 { Empty } 
                              //else if res == (L - X) as int { RStr("**") }
                              else if res == -1 { RStr("**".to_string()) }
                              else { Int(res) })
                }
            }

            println!("");
            table.print(format!("{} lands", L).as_slice());
        }

        if true {
            summary_c(16, 40);
            summary_c(17, 40);
            summary_c(18, 40);

            summary_c(22, 60);
            summary_c(23, 60);
            summary_c(24, 60);
            summary_c(25, 60);
            summary_c(26, 60);
        }

        if false {
            // Making the Frank 1 colored mana table:
            fn frank(colored_mana: uint, cmc: uint) -> Table {

                let manas = vec!(16u, 17, 18);
                let mut lines = manas.iter().map(|l| {
                        let f = 10.0;
                        (40u, *l, 0u, (f-1.0)/f) 
                    }).enumerate();
                
                let mut t1 = Table::new(4, 8);
                
                let ps = pm(colored_mana, cmc);
                
                t1.set(0, 0, LStr(ps));
                
                for turn in range(1, 8) { t1.set(0, turn as uint, Int(turn)) };
                
                for (line_no, line) in lines {
                    let (d, l, e, pc) = line;
                    
                    t1.set(1u+line_no, 0,
                           LStr(format!("{} lands {}", l, 
                                        if e == 0 { 'p' } 
                                        else { 'd' })));
                    
                    for turn in range(1u, 8) {
                        let draws = turn - 1 + e;
                        let goal = |hand: &ColoredPile| { 
                        hand.colored() >= colored_mana // colors okay
                                && hand.lands() >= cmc // enough lands for cmc
                                && turn >= cmc // one land per turn
                        };
                        let res = single::cards(l, d, draws, pc, goal);
                        t1.set(1u+line_no, turn, if res == 0 { Empty } else { Int(res) }) 
                    }
                }
                t1
            };

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

        if false {

		    fn g1(hand: &ColoredPile) -> bool {
			    hand.lands() >= 3 && hand.colored() >= 3
			}
		    fn g2(hand: &ColoredPile) -> bool {
			    hand.lands() >= 3
			}
		    fn g3(hand: &ColoredPile) -> bool {
			    hand.colored() >= 3
			}
		    
		    let draws = 2;
		    
		    let L = 24u;
		    let D = 60u;
		    
		    println!("{:8}: {:8}", "Colored", "Absolute");
		    for k in range(0u, L+1) {
				let r1 = single::turn0(&ColoredPile::new(k, L-k, D-L), draws, g1);
				let r2 = single::turn0(&ColoredPile::new(k, L-k, D-L), draws, g2);
				//let r3 = turn0(ColoredPile::new(k, L-k, D-L), draws, g3);
				
				println!("{:2} of {:2}: {:8.3}% {:8.3}% {:8.3}%", k, L,
				         100.0 * r1, 100.0 * r2, 100.0 * r1 / r2)
			}
	    }
        
        if true {
            standard::test();
        }
    }
    else if args.len() == 2 {
        let a = match from_str(args.get(1).as_slice()) {
            Some(x) => x,
            None    => -1
        };

        for k in range(0, 10) {
            println!("{}^{} = {}", a, k, prob::pow(a, k, 1));
        }
    }
    else {
        let a:uint = match from_str(args.get(1).as_slice()) {
            Some(x) => x,
            None    => -1
        };
        
        let b:uint = match from_str(args.get(2).as_slice()) {
            Some(x) => x,
            None    => -1
        };
        
        for n in range(a, b) {
            println!("c({:3u}, {:2u}) = {:60.0f}", 100u, n, prob::c(100, n));
        }
    }
}

