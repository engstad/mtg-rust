#![crate_type = "lib"]

#![allow(dead_code)]
#![feature(slicing_syntax)]
#![feature(unboxed_closures)]
#![feature(core)]
#![feature(collections)] 
#![feature(path)]
#![feature(io)]

extern crate regex;
extern crate curl;
extern crate "rustc-serialize" as rustc_serialize;

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

