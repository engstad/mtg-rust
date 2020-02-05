extern crate itertools;
extern crate libmtg;
extern crate unicode_segmentation;

//use mtg::logic::*;
use libmtg::logic::{dual, frank_table, investigate, show_card_text, summary_c, summary_perc};

use libmtg::interval::*;
use libmtg::mtgjson::fetch_set;
use libmtg::pile::DualPile;
use libmtg::table::Table;
use libmtg::table::TableElem::{LStr, RStr, U32 /*, I32, Empty */};
//use std::path::Path;
//use std::fs::File;
//use std::io::Write;
use itertools::*;
use std::convert::TryInto;
use std::iter::repeat;

use libmtg::mtgjson::Rarity;
use unicode_segmentation::UnicodeSegmentation;

#[inline(always)]
fn rep(c: char, s: usize) -> String {
    repeat(c).take(s).collect()
}

// #[main]
fn main() {
    //use interval::closed;

    //let args = std::os::args();

    println!("Starting.");

    let args: Vec<String> = std::env::args().map(|x| x.to_string()).collect();

    if args.len() == 1 || (args.len() == 2 && (args[1] == "dump" || args[1] == "fetch")) {
        let mut cs = vec![];
        let sets = [
            // "BNG", "THS", /* Born of the Gods, Theros */];
            // "M15", "JOU", /* Magic 2015, Journey to Nyx */
            // "KTK", "FRF", /* Khans of Tarkir, Fate Reforged (until Q2 2017)  */
            // "DTK", "ORI", /* Dragons of Tarkir, Magic Origins (until Q4 2017)  */
            // "BFZ", "OGW", /* Battle for Zendikar, Oath of the Gate Watch (until Apr 8, 2018) */
            // "SOI", "EMN", /* Shadows over Innistrad, Eldritch Moon */
            // "KLD",
            "AER", /* Kaladesh, Aether Revolt */
            "AKH", "HOU", /* Amonkhet, Hours of Devastation */
            "XLN", "RIX", /* Ixalan, Rivals of Ixalan */
        ];

        for s in &sets {
            let set = fetch_set(s);
            if set.len() == 0 {
                println!("Didn't receive {}\n", s);
            }
            println!("Received {} of {}", set.len(), s);
            for c in set {
                cs.push(c)
            }
        }

        //cs.sort_by(|a, b| b.card_text.len().cmp(&a.card_text.len()));
        cs.sort_by(|a, b| b.card_text.cmp(&a.card_text));

        println!("{} cards", cs.len());

        //let fetch_images = args.len() == 2 && args[1] == "fetch";
        let width = 60;
        for c in cs
            .iter()
            .sorted_by(|a, b| match a.mana_cost.cmc().cmp(&b.mana_cost.cmc()) {
                std::cmp::Ordering::Equal => a.rarity.cmp(&b.rarity),
                d => d,
            })
        {
            if
            /*
               // c.sub_types.iter().any(|s| *s == "God") &&
               c.mana_cost.b == 0 &&
               c.mana_cost.r == 0 &&
               c.mana_cost.g == 0 &&
               // c.mana_cost.cmc() <= 2 &&
               //c.card_types.iter().any(|s| *s == "Creature") &&
               //(c.card_text.find("Flash").is_some() || c.card_text.find("flash").is_some()) &&
               c.card_types.iter().any(|s| *s == "Land") &&
               !c.super_types.iter().any(|s| *s == "Basic") &&
               {
                   let b = c.card_text.find("{B}").is_some() || c.card_text.find("Swamp").is_some();
                   let r = c.card_text.find("{R}").is_some() || c.card_text.find("Mountain").is_some();
                   let u = c.card_text.find("{U}").is_some() || c.card_text.find("Island").is_some();
                   (b && r) || (r && u) || (u && b)
               } &&
               !(c.card_text.find("{W}").is_some() || c.card_text.find("Plains").is_some()) &&
               !(c.card_text.find("{G}").is_some() || c.card_text.find("Forest").is_some()) &&
               //
               //c.sub_types.iter().any(|s| *s == "") &&
            */
            c.rarity == Rarity::Mythic &&
                //c.expansion == "AER" &&
                true
            {
                for _ in 0..width {
                    print!("=")
                }
                println!("");
                println!(
                    "[{:3}] {:40} {:6}\n({})   {:40} {}",
                    c.expansion,
                    c.card_name,
                    c.mana_cost.pretty(),
                    UnicodeSegmentation::graphemes(c.rarity.short(), false)
                        .next()
                        .unwrap_or("?"),
                    c.card_type,
                    if c.power.len() > 0 {
                        format!("{}/{}", c.power, c.toughness)
                    } else {
                        "".to_string()
                    }
                );
                if c.card_text.len() > 0 {
                    println!("{}", rep('-', width));
                    show_card_text(&*c.card_text, width);
                }
                //println!("super_types = {:?}", c.super_types);
                //println!("sub_types = {:?}", c.sub_types);

                println!("{}\n", rep('=', width));
            }
        }
    }
    //else if args.len() == 3 && args[1].as_slice() == "pic" {
    //    sdl_main(args[2].as_slice())
    //}
    //else
    else if args.len() == 2 && args[1] == "land" {
        investigate()
    } else if args.len() == 2 && args[1] == "duals" {
        let mut dp = Table::new(18, 2);
        for a in 0..=17-4 {
            let goal = |hand: DualPile| (hand.a >= 1) || hand.ab >= 1;
            let td = DualPile::new(a, 17 - 4 - a, 4, 0, 23);
            let rt = dual::turn0(td, 1, goal);
            dp.set(a, 0, LStr(format!("{:?}", td)));
            dp.set(a, 1, RStr(format!("{:6.2}%", rt * 100.0)));
        }
        dp.print("Duals");
    } else if args.len() >= 2 && args[1] == "table" {
        let l = if args.len() == 3 {
            args[2].parse().unwrap_or(0)
        } else {
            0
        };
        if l == 0 {
            for i in closed(16, 18).iter() {
                summary_c(i, 40);
            }
            for i in closed(22, 28).iter() {
                summary_c(i, 60);
            }
        } else if l <= 19 {
            summary_c(l, 40);
        } else {
            summary_c(l, 60)
        }
    } else if args.len() == 2 && args[1] == "frank" {
        frank_table()
    } else if args.len() == 3 && args[1] == "pow" {
        let a = args[2].parse().unwrap_or(0usize);
        for k in closed(0, 10).iter() {
            println!("{}^{} = {}", a, k, libmtg::prob::pow(a, k));
        }
    } else if args.len() == 4 && args[1] == "C" {
        let a: u64 = args[2].parse().unwrap_or(0);
        let b: u64 = args[3].parse().unwrap_or(1);

        println!("c({:3}, {:2}) = {:60.0}", a, b, libmtg::prob::c(a, b));
    } else if args.len() == 2 && args[1] == "dice" {
        /*
        println!("{:3} {:6}  {:6}  {:6}  {:6}  {:6}  {:6}   | {:6}  {:6}  {:6}  {:6}  {:6}",
                 "#D", 0, 1u, 2u, 3u, 4u, 5u, "  >= 1", "  >= 2", "  >= 3", "  >= 4", "  >= 5");
        for n in closed(2u, 10).iter() {
            let mut count = [0i, 0, 0, 0, 0, 0];

            let num_diff = libmtg::prob::pow(6, n as i32, 1) as u32;

            for p in range(0, num_diff).iter() {
                let res = Vec::from_fn(n, |i| (p / libmtg::prob::pow(6, i as i32, 1) as u32) % 6); // 0-5
                let mut freq = [0, 0, 0, 0, 0, 0];
                for r in res.iter() { freq[*r] += 1 };
                if freq[4] >= 2 { count[5] += 1 }
                else if freq[3] >= 2 { count[4] += 1 }
                else if freq[2] >= 2 { count[3] += 1 }
                else if freq[1] >= 2 { count[2] += 1 }
                else if freq[0] >= 2 { count[1] += 1 }
                else { count[0] += 1 }
            }

            dt.set(n - 1, 0, U32(n));
            for i in closed(0, 5).iter() {
                let r = (count[i] as f64) / (num_diff as f64);
                dt.set(n - 1, 1 + i, LStr(format!("{:6.1}% ", r * 100.0)));
            }
            for i in closed(1u, 5).iter() {
                let r = count[i..6u].iter().fold(0.0, |acc, c| acc + (*c as f64) / (num_diff as f64));
                dt.set(n - 1, 6 + i, LStr(format!("{:6.1}% ", r * 100.0)));
            }
        }

        dt.print("Action Dice")
        */

        //
        // 5's are easy: 2 : 1/36
        //               3 : (C(3,2) * 5 + 1)/6^3 [55x, 5x5, x55]
        //
        // 4: 55xx, 5x5x, 5xx5, x5x5, xx55, where x!=5, i.e.: C(4,2)*5^2 = 150
        //    Also, 555x, 55x5, 5x55, x555, where x!=5, i.e.: C(4,3)*5   = 20
        //    Also, 5555                                i.e.: C(4,4)     = 1
        //
        // --
        //
        // 4's is harder, must also account for non 5's.
        //
        // 44xxx, 4x4xx, .., xxx44 : C(5,2) * 5^3 = 10 * 5^3 = 1,250
        // x's: 55x, 5x5, x55, C(3, 1) * 4        = 10 * 13  =   130
        //      555            C(3, 0)
        // 444xx, 44x4x, .., xx444 : C(5,3) * 5^2 = 10 * 5^2 =   250
        // x's: 55                                = 10 * 1   =    10
        // 4444x, 444x4, .., x4444 : C(5,4) * 5^1 =  5 * 5   =    25
        // 44444                   : C(5,5) * 5^0 =  1 * 1   =     1
        //

        // Counting the number of ways to get doubles of `d`, where we
        // thrown `n` dice and we assume that there are `b` x's.
        fn doubles(d: usize, n: usize, b: usize) -> usize {
            //println!("d({},{},{})=?", d, n, b);
            if n == 0 {
                0
            } else if d == 0 {
                let t: usize = libmtg::prob::pow(6, n);
                let s: usize = (1..6usize).map(|od| doubles(od, n, b)).sum();
                t - s
            } else {
                closed(2usize, n).iter().fold(0usize, |acc, k| {
                    let c = libmtg::prob::ch(n, k);
                    let p = libmtg::prob::pow(b, n - k);
                    //println!("k: {} -- (c, p) = ({}, {})", k, c, p);
                    let q = if n >= k + 2 && d < 5 && b > 1 {
                        ((d + 1)..6).map(|s| doubles(s, n - k, b - 1)).sum()
                    } else {
                        0
                    };
                    //println!(" : C({},{}) = {} : {} * ({} - {}) = {}", n, k, c, c, p, q, c * (p - q));
                    acc + c * (p - q)
                })
            }
        }

        //println!("res={}", doubles(4, 5, 5));

        let mut dt = Table::new(10, 13);
        dt.set(0, 0, LStr("Dice".to_string()));
        for i in closed(0, 5).iter() {
            dt.set(0, i + 1, RStr(format!("{}  ", i)))
        }
        for n in closed(0, 9).iter() {
            dt.set(n, 7, RStr("|".to_string()))
        }
        for i in closed(1, 5).iter() {
            dt.set(0, 7 + i, RStr(format!(">= {}  ", i)))
        }
        for n in closed(2, 10).iter() {
            dt.set(n - 1, 0, U32(n as u32));
            for i in closed(0, 5).iter() {
                let r =
                    doubles(i, n, 5) as f64 / libmtg::prob::pow(6, n.try_into().unwrap()) as f64;
                dt.set(n - 1, 1 + i, LStr(format!("{:4.1}% ", r * 100.0)));
            }
            for i in closed(1, 5).iter() {
                let r = closed(i, 5).iter().fold(0.0, |acc, c| {
                    acc + doubles(c, n, 5) as f64 / libmtg::prob::pow(6, n) as f64
                });
                dt.set(n - 1, 7 + i, LStr(format!("{:4.1}% ", r * 100.0)));
            }
        }
        dt.print("Action Dice")
    } else if args.len() == 2 {
        let (lands, colored_lands) = libmtg::land::analyze(&*args[1]);
        summary_c(lands as usize, 60);
        for &clands in &colored_lands {
            summary_perc(lands as usize, clands as usize, 60);
        }
    } else if false {
        let l = 26;
        let d = 60;
        for u in closed(0, 4).iter() {
            libmtg::logic::summary(l, d, u)
        }
    }
}
