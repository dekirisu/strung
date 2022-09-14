#![allow(non_snake_case)]
use std::fmt::Display;
use strung::prelude::*;

// named struct
#[derive(Strung)]   // easy derive
struct Test<A: Display> {
    num: A,
    name: &'static str,
}

fn main(){
    // create structs!
    let NAMED  = Test::<u32> {num: 1, name: "st"};

    let text = NAMED.strung("{num}{name}"); 
    println!("strung: {}",&text);
}