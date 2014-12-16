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
use mtg::logic::{show_card_text, investigate, frank_table, summary, summary_c, dual};

use mtg::pile::{DualPile};
use mtg::table::Table;
use mtg::table::TableElem::{LStr, RStr /*, Int, UInt, Empty */};
use mtg::mtgjson::{fetch_set, fetch_img};
use mtg::interval::*;
use std::io::File;

#[main]
fn main() {
    //use interval::closed;

    let args = std::os::args();

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
                true
            {
                println!("{}", "=".repeat(width));
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
    //else if args.len() == 3 && args[1].as_slice() == "pic" {
    //    sdl_main(args[2].as_slice())
    //}
    else if args.len() == 2 && args[1].as_slice() == "land"	{
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
        else if l <= 19 {
            summary_c(l, 40);
        } else { 
            summary_c(l, 60)
        }
    }
    else if args.len() == 2 && args[1].as_slice() == "frank" {
        frank_table()
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
    else if args.len() == 2 {
        let lands = mtg::standard::analyze(args[1].as_slice());
        summary_c(lands, 60);
    }
    else if args.len() == 3 && args[1].as_slice() == "pow" {
        let a = from_str(args[2].as_slice()).unwrap_or(0);
        for k in closed(0i, 10).iter() {
            println!("{}^{} = {}", a, k, mtg::prob::pow(a, k, 1));
        }
    }
    else if args.len() == 4 && args[1].as_slice() == "C" {
        let a:uint = from_str(args[2].as_slice()).unwrap_or(0);
        let b:uint = from_str(args[3].as_slice()).unwrap_or(1);
        
        for n in mtg::interval::range(a, b).iter() {
            println!("c({:3}, {:2}) = {:60.0}", 100u, n, mtg::prob::c(100, n));
        }
    }
}

