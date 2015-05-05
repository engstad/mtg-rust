#![crate_type = "lib"]

#![allow(dead_code)]
#![feature(core)]
#![feature(collections)]

#![feature(convert)]
#![feature(unicode)]

extern crate regex;
extern crate curl;
extern crate rustc_serialize;
extern crate num;

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

