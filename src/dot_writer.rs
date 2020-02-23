use crate::productionchain::ProductionChain;
use crate::recipes::{Recipe, RecipeDatabase};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::prelude::*;

struct IngredientNodeDeclarator(HashSet<String>);

impl IngredientNodeDeclarator {
    fn new() -> Self {
        Self(HashSet::new())
    }
    fn declare_ingr(&mut self, file: &mut impl Write, name: &String) {
        if !self.0.contains(name) {
            writeln!(file, r#"  "{0}" [label = "{0}" shape="rectangle"];"#, name);
            self.0.insert(name.clone());
        }
    }
}

pub fn write_dot_files(prod_chain: ProductionChain, recipes: RecipeDatabase) {
    let mut count = 0;

    for prod_block in prod_chain.blocks {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(count.to_string() + ".dot")
            .unwrap();

        count += 1;

        writeln!(
            file,
            r#"digraph "{0} - {1}" {{"#,
            prod_block.name, prod_block.count
        );

        writeln!(
            file,
            r#"  input [label = "<input>" shape=circle color=grey];"#
        );

        let mut item_declarator = IngredientNodeDeclarator::new();

        for (name, count) in prod_block.ingredients {
            writeln!(
                file,
                r#"  input -> "{0}" [color="grey", label="{1}"];"#,
                name, count
            );
            item_declarator.declare_ingr(&mut file, &name);
        }

        writeln!(file, r"");
        for prod_recipe in prod_block.recipes {
            match (recipes.get_recipe(&prod_recipe.name)) {
                Some(recipe) => {
                    writeln!(
                        file,
                        r#"  "Rec_{0}" [label = "{0}*{1}" shape=circle color=grey]"#,
                        prod_recipe.name, prod_recipe.output_count
                    );

                    for ingr in &recipe.ingredients {
                        item_declarator.declare_ingr(&mut file, &ingr.name);
                        writeln!(
                            file,
                            r#"  "{0}" -> "Rec_{1}" [label = "{2:.2}" color=red]"#,
                            ingr.name,
                            recipe.id,
                            (ingr.amount.unwrap_or_else(|| { ingr.amount_min.unwrap() }))
                                / recipe.energy
                        );
                    }

                    for ingr in &recipe.products {
                        item_declarator.declare_ingr(&mut file, &ingr.name);
                        writeln!(
                            file,
                            r#"  "Rec_{1}" -> "{0}" [label = "{2:.2}" color=green]"#,
                            ingr.name,
                            recipe.id,
                            (ingr.amount.unwrap_or_else(|| { ingr.amount_min.unwrap() }))
                                / recipe.energy
                        );
                    }
                }
                None => {
                    writeln!(
                        file,
                        r#"  "Rec_{0}" [label="RECIPE NOT FOUND: {0}" color=red shape=star]"#,
                        prod_recipe.name
                    );
                }
            }
        }
        writeln!(file, r#"}}"#);
    }
    generate_makefile(count);
}

/// Generates Makefile for the generated DOT files
fn generate_makefile(count: i32) {
    let mut makefile = OpenOptions::new()
        .write(true)
        .create(true)
        .open("Makefile")
        .unwrap();

    write!(makefile, "all:");

    for i in 0..count {
        write!(makefile, " {0}.png", i);
    }
    writeln!(makefile, "");
    writeln!(makefile, ".PHONY: all");

    for i in 0..count {
        writeln!(makefile, r#"{0}.png: {0}.dot"#, i);
        writeln!(makefile, r#"	dot -Tpng {0}.dot -o {0}.png"#, i);
    }
}
