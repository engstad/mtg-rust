#![feature(slicing_syntax)]
#![feature(globs)]

extern crate collections;
extern crate regex;
extern crate serialize;
extern crate curl;
extern crate core;
extern crate libc;
extern crate mtg;

//use mtg::logic::*;
use mtg::logic::{investigate, frank_table, summary, summary_c, dual};

use mtg::pile::{DualPile};
use mtg::table::Table;
use mtg::table::TableElem::{LStr, RStr, UInt /*, Int, Empty */};
//use mtg::mtgjson::{fetch_set, fetch_img};
use mtg::interval::*;
//use std::io::File;

#[main]
fn main() {
    //use interval::closed;

    let args = std::os::args();

    /*
    if args.len() == 1 || (args.len() == 2 && (args[1] == "dump" || args[1] == "fetch")) {
        let mut cs = vec![];
        cs.push_all(fetch_set("KTK")[]);
        cs.push_all(fetch_set("M15")[]);
        cs.push_all(fetch_set("JOU")[]);
        cs.push_all(fetch_set("BNG")[]);
        cs.push_all(fetch_set("THS")[]);
        //cs.sort_by(|a, b| a.mana_cost.cmc().cmp(&b.mana_cost.cmc()));
        cs.sort_by(|a, b| b.card_text.len().cmp(&a.card_text.len()));

        println!("{} cards", cs.len());

        let fetch_images = args.len() == 2 && args[1] == "fetch";
        let width = 60;
        for c in cs.iter() {
            if // c.sub_types.iter().any(|s| s[] == "God") &&
                !c.colors.iter().any(|s| *s == mtg::colors::Color::W) &&
                // !c.colors.iter().any(|s| *s == mtg::colors::U) &&
                // !c.colors.iter().any(|s| *s == mtg::colors::B) &&
                !c.colors.iter().any(|s| *s == mtg::colors::Color::G) &&
                // !c.colors.iter().any(|s| *s == mtg::colors::R) &&                
                c.colors.iter().any(|s| (*s == mtg::colors::Color::R ||
                                         *s == mtg::colors::Color::B ||
                                         *s == mtg::colors::Color::U)) &&
                true
            {
                for _ in range(0, width).iter() { print!("=") } println!("");
                println!("[{:3}] {:40} {:6}\n({})   {:40} {}", 
                         c.expansion, c.card_name, c.mana_cost.pretty(), 
                         c.rarity.graphemes(false).next().unwrap_or("?"),
                         c.card_type, 
                         if c.power.len() > 0 { format!("{}/{}", c.power, c.toughness) } 
                         else { "".into_string() });
                if c.card_text.len() > 0 {
                    println!("{}", "-".repeat(width));
                    show_card_text(c.card_text.as_slice(), width);
                }
                println!("{}\n", "=".repeat(width));

                if fetch_images {
                    let jpg = fetch_img(c.expansion.as_slice(), 
                                        format!("{}", c.image_name).as_slice());
                    
                    let mut f = File::create(&Path::new(format!("pics/{}-{}.jpg", 
                                                                c.expansion, c.image_name)));
                    match f.write(jpg.as_slice()) {
                        Ok(()) => (), 
                        Err(s) => { println!("Couldn't write JPG file: {}", s); return () }
                    }
                }
            }
        }
    }
    */
    //else if args.len() == 3 && args[1].as_slice() == "pic" {
    //    sdl_main(args[2].as_slice())
    //}
    //else 
    if args.len() == 2 && args[1].as_slice() == "land"	{
		investigate()
    }
    else if args.len() == 2 && args[1].as_slice() == "duals" {
		let mut dp = Table::new (18, 2);
		for a in closed(0u, 17).iter() {
			let goal = |&:&hand: &DualPile | {(hand.a >= 1) || hand.ab >= 1 };
			let td = DualPile::new (a, 17 - a, 0, 0, 23);
			let rt = dual::turn0(&td, 1, goal);
			dp.set(a, 0, LStr(format !("{}", td)));
			dp.set(a, 1, RStr(format !("{:6.2}%", rt * 100.0)));
		}
		dp.print("Duals");
    }
    else if args.len() >= 2 && args[1].as_slice() == "table" {
        let l = if args.len() == 3 { args[2].parse::<uint>().unwrap_or(0) } else { 0 };
        if l == 0 {
	    for i in closed(16, 18).iter() { summary_c(i, 40); }
	    for i in closed(22, 28).iter() { summary_c(i, 60); }
        }
        else if l <= 19 {
            summary_c(l, 40);
        } else { 
            summary_c(l, 60)
        }
    }
    else if args.len() == 2 && args[1].as_slice() == "frank" {
        frank_table()
    }
    else if args.len() == 3 && args[1].as_slice() == "pow" {
        let a = args[2].parse::<int>().unwrap_or(0);
        for k in closed(0i, 10).iter() {
            println!("{}^{} = {}", a, k, mtg::prob::pow_acc(a, k, 1));
        }
    }
    else if args.len() == 4 && args[1].as_slice() == "C" {
        let a:u64 = args[2].parse().unwrap_or(0);
        let b:u64 = args[3].parse().unwrap_or(1);
        
        println!("c({:3}, {:2}) = {:60.0}", a, b, mtg::prob::c(a, b));
    }
    else if args.len() == 2 && args[1].as_slice() == "dice" {
        /*
        println!("{:3} {:6}  {:6}  {:6}  {:6}  {:6}  {:6}   | {:6}  {:6}  {:6}  {:6}  {:6}",
                 "#D", 0u, 1u, 2u, 3u, 4u, 5u, "  >= 1", "  >= 2", "  >= 3", "  >= 4", "  >= 5");
        for n in closed(2u, 10u).iter() {
            let mut count = [0i, 0, 0, 0, 0, 0];
            
            let num_diff = mtg::prob::pow(6, n as int, 1) as uint;
            
            for p in range(0u, num_diff).iter() {
                let res = Vec::from_fn(n, |i| (p / mtg::prob::pow(6, i as int, 1) as uint) % 6); // 0-5
                let mut freq = [0u, 0, 0, 0, 0, 0];
                for r in res.iter() { freq[*r] += 1 };
                if freq[4] >= 2 { count[5] += 1 }
                else if freq[3] >= 2 { count[4] += 1 }
                else if freq[2] >= 2 { count[3] += 1 }
                else if freq[1] >= 2 { count[2] += 1 }
                else if freq[0] >= 2 { count[1] += 1 }
                else { count[0] += 1 }
            }

            dt.set(n - 1, 0, UInt(n));
            for i in closed(0u, 5).iter() {
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
        fn doubles(d: u64, n: u64, b: u64) -> u64 {            
            //println!("d({},{},{})=?", d, n, b);
            let r = if n == 0 { 0 } 
            else if d == 0 {
                mtg::prob::pow_acc(6, n as int, 1) as u64 - 
                    closed(1u64, 5).iter().fold(0u64, |acc, od| {
                        acc + doubles(od, n, b)
                    })
            }
            else {
                closed(2u64, n).iter().fold(0u64, |acc, k| {
                    let c:u64 = mtg::prob::ch(n, k);
                    let p:u64 = mtg::prob::pow_acc(b as int, (n-k) as int, 1) as u64;
                    //println!("k: {} -- (c, p) = ({}, {})", k, c, p);
                    let q:u64 = if n >= k + 2 && d < 5 && b > 1 {
                        closed(d+1, 5).iter()
                            .fold(0u64, |acc, s| acc + doubles(s, n - k, b - 1)) as u64
                    } 
                    else { 
                        0 
                    };
                    //println!(" : C({},{}) = {} : {} * ({} - {}) = {}", n, k, c, c, p, q, c * (p - q));
                    acc + c * (p - q)
                })
            };
            r
        }

        //println!("res={}", doubles(4, 5, 5));

        let mut dt = Table::new(10, 13);
        dt.set(0, 0, LStr("Dice".to_string()));
        for i in closed(0u, 5u).iter() { dt.set(0, i+1, RStr(format!("{}  ", i))) }
        for n in closed(0u, 9u).iter() { dt.set(n, 7, RStr("|".to_string())) }
        for i in closed(1u, 5u).iter() { dt.set(0, 7+i, RStr(format!(">= {}  ", i))) }
        for n in closed(2u, 10).iter() {
            dt.set(n - 1, 0, UInt(n));
            for i in closed(0u, 5).iter() {
                let r = doubles(i as u64, n as u64, 5) as f64 / mtg::prob::pow_acc(6, n as int, 1) as f64;
                dt.set(n - 1, 1 + i, LStr(format!("{:4.0}% ", r * 100.0)));
            }
            for i in closed(1u, 5).iter() {
                let r = closed(i, 5u).iter().fold(0.0, |acc, c| acc + doubles(c as u64, n as u64, 5) as f64 / mtg::prob::pow_acc(6, n as int, 1) as f64);
                dt.set(n - 1, 7 + i, LStr(format!("{:4.0}% ", r * 100.0)));
            }            
        }
        dt.print("Action Dice")
    }
    else if args.len() == 2 {
        let lands = mtg::standard::analyze(args[1].as_slice());
        summary_c(lands, 60);
    }
    else if false {
        let l = 26;
        let d = 60;
        for u in closed(0u, 4u).iter() {
            mtg::logic::summary(l, d, u)
        }
    }
}

