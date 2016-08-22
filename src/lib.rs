#![crate_type = "lib"]

#![allow(dead_code)]
#![feature(inclusive_range_syntax)]
#![feature(question_mark)]

extern crate regex;
//extern crate curl;
extern crate hyper;
extern crate rustc_serialize;
extern crate num;
extern crate unicode_width;
extern crate unicode_segmentation;

pub mod prob;
pub mod pile;
pub mod standard;
pub mod table;
pub mod perm;
pub mod interval;
pub mod mana;
pub mod colors;
pub mod mtgjson;
pub mod logic;

