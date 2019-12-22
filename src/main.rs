// first thing include this
#[macro_use]
extern crate lalrpop_util;

// modules
lalrpop_mod!(pub grammar);
mod ast;
mod grammar_tests;
mod productionchain;
mod recipes;

use productionchain::{ConversionError, ProductionChain};
use recipes::RecipeDatabase;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

fn main() {
    let mut file = File::open("recipe.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let recipes_json: serde_json::Value =
        serde_json::from_str(&contents).expect("JSON Parsing error in recipe.json");
    let recipes: RecipeDatabase = (&recipes_json).try_into().unwrap();

    let mut file = File::open("test.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parsed = grammar::ObjectParser::new().parse(&contents);
    let prod_chain: Result<ProductionChain, ConversionError> =
        (&parsed.expect("Parse error")).try_into();
    println!("{:?}", prod_chain);
    let prod_chain = prod_chain.expect("Convert error");

    for prod_block in prod_chain.blocks {
        println!("######### {}({:.2})", prod_block.name, prod_block.count);
        print!("From: ");
        for (name, count) in prod_block.ingredients {
            print!("{}({:.2}), ", name, count);
        }
        println!();
        for prod_recipe in prod_block.recipes {
            println!(
                "- {} ({:.2}) - [made in {}x {}]",
                prod_recipe.name,
                prod_recipe.output_count,
                (prod_recipe.factory_count.ceil() as i32),
                prod_recipe.factory_name
            );
        }
    }
}
