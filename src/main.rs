mod ast;

#[macro_use]
extern crate lalrpop_util;

use std::fs::File;
use std::io::prelude::*;
use std::string::String;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

fn main() {
    let mut file = File::open("test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parsed = grammar::ObjectParser::new().parse(&contents);
    println!("{:?}", parsed);
}

#[test]
fn test_numbers() {
    assert!(grammar::ObjectParser::new().parse("{x=42}").is_ok());
    assert!(grammar::ObjectParser::new().parse("{x=-42}").is_ok());
    assert!(grammar::ObjectParser::new().parse("{x=0}").is_ok());
    assert!(grammar::ObjectParser::new().parse("{x=-1}").is_ok());
    assert!(grammar::ObjectParser::new()
        .parse("{x=-122.532232}")
        .is_ok());
}
#[test]
fn test_strings() {
    assert!(grammar::ObjectParser::new()
        .parse(r#"{x="testString"}"#)
        .is_ok());
}

#[test]
fn test_objects() {
    assert!(grammar::ObjectParser::new().parse(r#"{}"#).is_ok());
    assert!(grammar::ObjectParser::new()
        .parse("{speed=0,productivity=0,consumption=0}")
        .is_ok());
}
