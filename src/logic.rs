use std::iter::repeat;
use crate::pile::{GenPile, GenPileKeys, DualPile, LandPile, ColoredPile};
use crate::table::Table;
use crate::table::TableElem::{LStr, RStr, I32, U32, Empty};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

//
// Mulligan Rule:
//
//  - 7 cards: Mulligan 0, 1, 6 or 7 lands (2, 3, 4 or 5 spells)
//  - 6 cards: Mulligan 0, 1, 5 or 6 lands (2, 3 or 4 spells)
//  - 5 cards: Mulligan 0 or 5 lands (1, 2, 3 or 4 spells)
//  - 4 cards: Always kept
//

fn mull_rule(hand_size: usize) -> (usize, usize) {
    match hand_size {
        7 => (2, 5),
        6 => (2, 4),
        5 => (1, 4),
        4 => (0, 4),
        _ => panic!("Eh")
    }
}

mod single {
    use crate::pile::{Pile, LandPile, ColoredPile};
    use crate::prob;

    fn draw<G>(hand: ColoredPile, draws: usize, deck: ColoredPile,
            goal: G) -> f64
        where G : Fn(ColoredPile)->bool
    {
        if draws > 0 {
            ColoredPile::iter(draws)
                .filter(|draw| deck.has(draw) && goal(hand + *draw))
                .map(|draw| deck.prob_draw(&draw))
                .sum()
        } else {
            prob::cond(goal(hand))
        }
    }

    fn intern<G>(hand_size: usize,
                 deck: ColoredPile, draws: usize, goal: G)
                 -> (f64, f64)
        where G : Fn(ColoredPile)->bool
    {
        let (lands_min, lands_max) = super::mull_rule(hand_size);

        // Probability of keeping
        let keep = (lands_min ..= lands_max)
            .map(|lands| deck.prob_land(lands, hand_size - lands))
            .sum::<f64>();

        // Probability of casting (where we auto-fail if we don't have the lands)
        let cast:f64 = ColoredPile::iter(hand_size)
                           .filter(|hand| hand.lands() >= lands_min && hand.lands() <= lands_max)
                           .filter(|hand| deck.has(hand))
                           .map(|hand| (deck.prob_draw(&hand) *
                                        draw(hand, draws, deck - hand, |g| goal(g))))
                           .sum();

        // So cast * keep = chance of reaching goals, *given* no mulligan.
        (keep, cast)
    }

    pub fn turn0<G>(deck: ColoredPile, draws: usize, goal: G) -> f64
        where G : Fn(ColoredPile)->bool
    {
        let mut mull = 1.0; // the chance we mulled before
        let mut succ = 0.0;

        for hand_size in (4 ..= 7).rev() {
            let (keep, cast) = intern(hand_size, deck, draws, |g| goal(g));
            succ += mull * (cast * keep);
            mull *= 1.0 - keep;
        }

        succ
    }

    pub fn cards<G>(lands: usize, deck: usize, draws: usize, perc: f64,
                 goal: G) -> i32
        where G : Fn(ColoredPile)->bool
    {
        let deck1 = ColoredPile::new(lands, 0, deck-lands);
        let r1 = turn0(deck1, draws, |g| goal(g));

        for k in 0..=lands {
            let deck0 = ColoredPile::new(k, lands-k, deck-lands);
            let r0 = turn0(deck0, draws, |g| goal(g));
            if r0 >= perc * r1 {
                return k as i32
            }
        }
        return 0
    }

    pub fn prob_color_screwed(lands: usize, colored: usize, deck: usize, cmc: usize, colored_mana: usize) -> String {
        let deck0 = ColoredPile::new(lands, 0, deck-lands);
        let res0 = turn0(deck0, cmc-1, |hand: ColoredPile| { hand.colored() >= colored_mana && hand.lands() >= cmc });
        let deck1 = ColoredPile::new(colored, lands-colored, deck-lands);
        let res1 = turn0(deck1, cmc-1, |hand: ColoredPile| { hand.colored() >= colored_mana && hand.lands() >= cmc });
        format!("{:.1}%", res1/res0*100.0)
    }
}

// ================================================================================

pub mod dual {
    use crate::pile::{Pile, LandPile, DualPile};
    use crate::prob;

    fn draw<G>(hand: DualPile, draws: usize, deck: DualPile, goal: G) -> f64
        where G : Fn(DualPile)->bool
    {
        if draws > 0 {
            DualPile::iter(draws)
                .filter(|draw| deck.has(draw) && goal(hand + *draw))
                .map(|draw| deck.prob_draw(&draw))
                .sum()
        } else {
            prob::cond(goal(hand))
        }
    }

    pub fn turn0<G>(deck: DualPile, draws: usize, goal: G) -> f64
        where G: Fn(DualPile)->bool
    {
        let mut mull = 1.0;
        let mut succ = 0.0;
        //let mut tally = 0.0;

        for hand_size in (4 ..= 7).rev() {
            let (lands_min, lands_max) = super::mull_rule(hand_size);

            // Probability of keeping
            let keep = (lands_min ..= lands_max)
                .map(|lands| deck.prob_land(lands, hand_size - lands))
                .sum::<f64>();

            // Probability of casting
            let cast:f64 = DualPile::iter(hand_size)
                .filter(|hand| hand.lands() >= lands_min && hand.lands() <= lands_max)
                .filter(|hand| deck.has(hand))
                .map(|hand| deck.prob_draw(&hand) * draw(hand, draws, deck - hand,
                                                         |g| goal(g)))
                .sum();

            succ += mull * (cast * keep);
            mull *= 1.0 - keep;
        }

        succ
    }

    pub fn cards<G>(lands: usize, deck: usize, uncolored: usize,
                    a_rate: f64, draws: usize, perc: f64, goal: G) -> i32
        where G : Fn(DualPile)->bool
    {

        let deck0 = DualPile::new(0, 0, lands, 0, deck-lands);
        let deck1 = DualPile::new(0, 0, lands-uncolored, uncolored, deck-lands);

        let r0 = turn0(deck0, draws, |g| goal(g));
        let r1 = turn0(deck1, draws, |g| goal(g));

        if r1 < perc * r0 {
            return -1
        }

        for ab in 0..=lands {
            let mono = lands - ab - uncolored;
            let a = ((mono as f64) * a_rate + 0.5).round() as usize;
            let b = mono - a;

            assert!(a+b+ab+uncolored+(deck-lands) == deck);

            let deck0 = DualPile::new(a, b, ab, uncolored, deck-lands);
            let r = turn0(deck0, draws, |g| goal(g));
            if r >= perc * r0 {
                return ab as i32
            }
        }
        return -1
    }
}

// ================================================================================

mod gen {
    use crate::pile::{Pile, LandPile, GenPile};
    use crate::prob;

    fn draw<G>(hand: GenPile, draws: usize, deck: GenPile, goal: G) -> f64
        where G : Fn(GenPile)->bool
    {
        if draws > 0 {
            deck.subsets(draws).iter()
                .filter(|&draw| goal(hand.clone() + draw.clone()))
                .map(|draw| deck.prob_draw(draw))
                .sum()
        } else {
            prob::cond(goal(hand))
        }
    }

    pub fn turn0<G>(deck: GenPile, draws: usize, goal: G) -> f64
        where G : Fn(GenPile)->bool
    {
        let mut mull = 1.0;
        let mut succ = 0.0;

        for hand_size in (4 ..= 7).rev() {
            let (lands_min, lands_max) = super::mull_rule(hand_size);

            // Probability of keeping
            let keep = (lands_min ..= lands_max)
                .map(|lands| deck.prob_land(lands, hand_size - lands))
                .sum::<f64>();

            // Probability of casting
            let cast:f64 =
                deck.subsets(hand_size).iter()
                .filter(|hand| hand.lands() >= lands_min && hand.lands() <= lands_max)
                .map(|hand| {
                    let d0 = deck.prob_draw(hand);
                    let p0 = {
                        let r = deck.clone() - hand.clone();
                        draw(hand.clone(), draws, r, |g| goal(g))
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

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use self::test::Bencher;

    #[bench]
    pub fn b_minc(bh: &mut Bencher) {
        let l = 28;
        let d = 60;
        let draws = 3;
        let pc = 0.9;

        let goal = |hand: ColoredPile| {
            hand.colored() >= 3 && hand.lands() >= 3 /*&& hand.turn >= 3*/
        };
        bh.iter(|| single::cards(l, d, draws, pc, |h| goal(h)))
    }
}

fn pm2(a:usize, b:usize, c:usize) -> String {
    let mut res = if c > 0 { c.to_string() } else { "".to_string() };
    res.push_str(&*repeat('A').take(a).collect::<String>());
    res.push_str(&*repeat('B').take(b).collect::<String>());
    res
}

// Summary of [lands] lands in a [D] card deck
pub fn summary(lands: usize, deck: usize, uncolored_lands: usize) {
    let mut table = Table::new(5, 9);

    {
        table.set(0, 0, LStr(format!("{}/{}({})", lands, deck, uncolored_lands)));
        table.set(0, 1, RStr("--".to_string()));
        for cless in 1u32..=7 {
            table.set(0, 1 + cless as usize, U32(cless))
        }
    }

    for cmana in 2..=5 {
        for bmana in 1..=cmana/2 {
            let amana = cmana - bmana;

            let gstr = pm2(amana, bmana, cmana - amana - bmana);
            table.set(cmana-1, 0, RStr(gstr));

            for cless in 0..=7 {

                let arate = (amana as f64) / (amana + bmana) as f64;
                let cmc = cmana + cless;
                let draws = cmc - 1;
                let goal = |hand: DualPile| {

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
                                  else { I32(res) })
            }
        }
    }

    println!("");
    table.print(&format!("{} lands, {} colorless", lands, uncolored_lands));
}

fn pm(colored_mana:usize, cmc:usize) -> String {
    let nc = cmc - colored_mana;
    let mut res = if nc > 0 { nc.to_string() } else { "".to_string() };
    res.push_str(&*repeat('C').take(colored_mana).collect::<String>());
    res
}

pub fn summary_c(lands: usize, deck: usize) {

    // Making my adjusted tables
    let mut table = Table::new(5, 9);

    {
        table.set(0, 0, LStr(format!("{}/{}", lands, deck)));
        table.set(0, 1, RStr("--".to_string()));
        for cless in 1..=7 {
            table.set(0, (1 + cless) as usize, I32(cless))
        };
    }

    for cmana in 1..=4 {
        let gstr = pm(cmana, cmana);
        table.set(cmana, 0, RStr(gstr));

        for cless in 0..=7 {
            let cmc = cmana + cless;
            let draws = cmc - 1;
            let goal = |hand: ColoredPile| {
                let ok = hand.colored() >= cmana // colors okay
                    && hand.lands() >= cmc; // enough lands for cmc

                ok
            };
            let res = single::cards(lands, deck, draws, 0.90, goal);
            table.set(cmana, 1+cless,
                      if res == 0 { Empty }
                      //else if res == (lands - uncolored_lands) as i32 { RStr("**") }
                      else if res == -1 { RStr("**".to_string()) }
                      else { I32(res) })
        }
    }

    println!("");
    table.print(&format!("{} lands", lands));
}

pub fn summary_perc(lands: usize, colored_lands: usize, deck: usize) {
    // Making my adjusted tables
    let mut table = Table::new(5, 9);

    {
        table.set(0, 0, LStr(format!("{}/{}", lands, deck)));
        table.set(0, 1, RStr("--".to_string()));
        for cless in 1..=7 {
            table.set(0, (1 + cless) as usize, I32(cless))
        };
    }

    for cmana in 1..=4 {
        let gstr = pm(cmana, cmana);
        table.set(cmana, 0, RStr(gstr));

        for cless in 0..=7 {
            let cmc = cmana + cless;
            let res = single::prob_color_screwed(lands, colored_lands, deck, cmc, cmana);
            table.set(cmana, 1+cless, RStr(res))
        }
    }

    println!("");
    table.print(&format!("{}/{} lands", colored_lands, lands));
}


pub fn investigate()
{
    #[inline(always)]
    fn min(a:usize, b:usize) -> usize { if a < b { a } else { b } }

    #[inline(always)]
    fn is_land(idx: usize) -> bool { idx == 0 || idx == 1 || idx == 2 }

    fn can_cast(la:usize, lb:usize, lab:usize, lx:usize,
                a: usize, b: usize, x: usize) -> bool {
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
        static A  :usize = 0;
        static B  :usize = 1;
        static C  :usize = 2;
        static AB :usize = 3;
        static BC :usize = 4;
        static AC :usize = 5;
        static S1 :usize = 6;
        static S2 :usize = 7;
        static O  :usize = 8;

        // a,b,c,ab,bc,ac,s,o
        fn is_land(idx: usize) -> bool { idx < 6 }

        let info = GenPileKeys::new(9, is_land);

        fn cc(hand: GenPile, a: usize, b: usize, x: usize) -> bool {
            can_cast(hand[A] + hand[AC], hand[B] + hand[BC], hand[AB], hand[C],
                     a, b, x)
        }

        let turn = 3;

        for cmc2s in 2 ..= 16 {
            for s1 in 1..cmc2s {
                let s2 = cmc2s - s1;

                let mut best_p = 0.0;
                let mut best_a = 0;

                for a in 0 ..= 17 {
                    let b = 17 - a;

                    let deck = GenPile::new(vec![a, b, 0,
                                                 0, 0, 0,
                                                 s1, s2, 23-s1-s2], info);
                    let p_base = gen::turn0(deck.clone(), turn,
                                            |hand : _| {
                                                hand.lands() >= turn && hand[S1] + hand[S2] > 0
                                            });
                    let p_succ = gen::turn0(deck, turn,
                                            |hand| {
                                                (cc(hand.clone(), 2, 0, turn-2) && hand.clone()[S1] > 0) ||
                                                (cc(hand.clone(), 0, 2, turn-2) && hand[S2] > 0)
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
pub fn frank(colored_mana: usize, cmc: usize) -> Table {
    let mut t1 = Table::new(4, 8);

    let ps = pm(colored_mana, cmc);

    t1.set(0, 0, LStr(ps));

    for turn in 1..=7 { t1.set(0, turn as usize, I32(turn)) };

    let manas = vec!(16, 17, 18);
    let lines = manas.iter().map(|l| {
        let f = 10.0f64;
        (40, *l, 0, (f-1.0)/f)
    }).enumerate();

    for (line_no, line) in lines {
        let (d, l, e, pc) = line;
        let sym = if e == 0 {'p'} else {'d'};
        t1.set(1+line_no, 0, LStr(format!("{} lands {}", l as usize, sym)));

        for turn in 1..=7 {
            let draws = turn - 1 + e;
            let goal = |hand: ColoredPile| {
                hand.colored() >= colored_mana // colors okay
                    && hand.lands() >= cmc // enough lands for cmc
                    && turn >= cmc // one land per turn
            };
            let res = single::cards(l, d, draws, pc, goal);
            t1.set(1+line_no, turn, if res == 0 { Empty } else { I32(res) })
        }
    };

    t1
}

pub fn frank_table()
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

pub fn show_card_text(txt : &str, width : usize)
{
    let mut col = 0;
    let mut indent = 0;
    let mut escape = false;
    let mut word : String = String::new();
    for g in UnicodeSegmentation::graphemes(txt, true) {
        if !escape {
            match g {
                "\\" => escape = true,
                " " => {
                    let l = UnicodeWidthStr::width(&*word);
                    let brk = col + l >= width;
                    if brk {
                        print!("\n");
                        if indent > width / 4 { indent = 4 }
                        print!("{}", repeat(' ').take(indent).collect::<String>());
                        col = indent
                    }
                    col += l + 1;
                    print!("{} ", word); word.clear()
                },
                ":" | "•" => {
                    word.push_str(g);
                    indent = col + UnicodeWidthStr::width(&*word) + 1;
                },
                _ => word.push_str(g)
            }
        } else {
            // let chars = "①②";
            match g {
                "n" => {
                    let l = UnicodeWidthStr::width(&*word);
                    if col + l >= width {
                        print!("\n{}{}\n",
                               repeat(' ').take(indent).collect::<String>(), word)
                    }
                    else {
                        print!("{}\n", word)
                    };
                    col = 0;
                    indent = 0;
                    word.clear();
                    escape = false
                },
                _ => { escape = false; word.push_str(g) }
            }
        }
    }
    if col + UnicodeWidthStr::width(&*word) >= width {
        if indent > width / 4 { indent = 4 }
        print!("\n{}", repeat(' ').take(indent).collect::<String>())
    }
    print!("{}\n", word)
}
