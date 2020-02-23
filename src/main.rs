// first thing include this
#[macro_use]
extern crate lalrpop_util;

// modules
lalrpop_mod!(pub grammar);
mod ast;
mod dot_writer;
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
    let cmdline_args = get_cmdline_args();

    let recipes = parse_recipes(cmdline_args.value_of("recipes").unwrap());
    let prod_chain = parse_helmod(cmdline_args.value_of("helmod").unwrap());

    if !recipes.is_ok() || !prod_chain.is_ok() {
        eprintln!("Encountered an error. Exiting.");
        return;
    }

    print!("Generating dot files...");
    let recipes = recipes.unwrap();
    let prod_chain = prod_chain.unwrap();
    dot_writer::write_dot_files(prod_chain, recipes);
    println!("Done");
}

fn get_cmdline_args() -> clap::ArgMatches<'static> {
    clap::App::new("Helmod to graphviz dot exporter")
                          .version("1.0")
                          .author("Daniele C. <danicampa90 AT gmail.com>")
                          .about("Parses an exported production block, the list of recipes from recipes.json and generates a graph in graphviz dot format of the flow of materials.")
                          .arg(clap::Arg::with_name("recipes")
                               .short("r")
                               .long("recipes")
                               .value_name("FILE")
                               .help("Indicates where to find the recipes.json file containing all the recipes")
                               .takes_value(true)
                               .required(true))
                          .arg(clap::Arg::with_name("helmod")
                               .help("h")
                               .long("helmod")
                               .value_name("FILE")
                               .help("Indicates where to find the exported helmod production chain")
                               .required(true)).get_matches()
}

/// Parses recipes.json and returns the recipe representation
fn parse_recipes(filename: &str) -> Result<RecipeDatabase, ()> {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let recipes_json: Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str(&contents);

    match recipes_json {
        Ok(recipes_json) => {
            let recipes_result = (&recipes_json).try_into();
            match (recipes_result) {
                Ok(recipes) => return Ok(recipes),
                Err(err) => {
                    eprintln!("ERROR: {:?}", err);
                    return Err(());
                }
            }
        }

        Err(err) => {
            eprintln!("ERROR: {:?}", err);
            return Err(());
        }
    }
}

/// Parses an string exported from helmod mod, and produces a production chain representation.
fn parse_helmod(filename: &str) -> Result<ProductionChain, ()> {
    // helmod
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parsed = grammar::ObjectParser::new().parse(&contents);
    match parsed {
        Err(err) => {
            eprintln!("ERROR: {:?}", err);
            return Err(());
        }
        _ => (),
    }

    let parsed = parsed.unwrap();

    let prod_chain: Result<ProductionChain, ConversionError> = (&parsed).try_into();

    match prod_chain {
        Err(err) => {
            eprintln!("ERROR: {:?}", err);
            return Err(());
        }
        Ok(prodchain) => return Ok(prodchain),
    }
}

/// Prints some quick representation from the production chain and recipe database in string format.
#[cfg(Debug)]
fn debug_print(prod_chain: ProductionChain, recipes: RecipeDatabase) {
    // debug print
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

            match (recipes.get_recipe(&prod_recipe.name)) {
                Some(recipe) => {
                    print!("    Found a recipe: ");
                    for ingr in &recipe.ingredients {
                        print!("<{}>", ingr.name);
                    }
                    print!("-->");
                    for ingr in &recipe.products {
                        print!("<{}>", ingr.name);
                    }
                    println!("")
                }
                None => println!("    No recipe found"),
            }
        }
    }
}
