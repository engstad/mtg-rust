#![crate_type = "lib"]

#![allow(dead_code)]

extern crate regex;
extern crate reqwest;
extern crate rustc_serialize;
extern crate num;
extern crate unicode_width;
extern crate unicode_segmentation;
extern crate url;

pub mod prob;
pub mod pile;
pub mod land;
pub mod table;
pub mod perm;
pub mod interval;
pub mod mana;
pub mod colors;
pub mod mtgjson;
pub mod logic;
