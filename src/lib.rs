#![crate_type = "lib"]

#![allow(dead_code)]
#![feature(slicing_syntax)]
#![feature(unboxed_closures)]

extern crate collections;
extern crate regex;
extern crate curl;
extern crate core;
extern crate libc;
//extern crate serialize;
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

