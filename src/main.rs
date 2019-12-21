// first thing include this
#[macro_use]
extern crate lalrpop_util;

// modules
lalrpop_mod!(pub grammar);
mod ast;
mod grammar_tests;
mod productionchain;

use productionchain::{ConversionError, ProductionChain};
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

fn main() {
    let mut file = File::open("test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parsed = grammar::ObjectParser::new().parse(&contents);
    let prodChain: Result<ProductionChain, ConversionError> =
        (&parsed.expect("Parse error")).try_into();
    println!("{:?}", prodChain);
}
