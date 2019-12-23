use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

pub struct IngredientProductInfo {
    pub type_: String,
    pub name: String,
    pub amount: Option<f64>,
    pub amount_min: Option<f64>,
    pub amount_max: Option<f64>,
    pub probability: Option<f64>,
}

pub struct Recipe {
    pub id: String,
    pub category: String,
    pub energy: f64,
    pub ingredients: Vec<IngredientProductInfo>,
    pub products: Vec<IngredientProductInfo>,
}

pub struct RecipeDatabase {
    pub recipes: HashMap<String, Recipe>,
}
#[derive(Debug)]
pub enum JSONParsingError {
    FieldNotExistsError(&'static str),
    InvalidType(&'static str),
}

impl RecipeDatabase {
    pub fn get_recipe(&self, id: &String) -> Option<&Recipe> {
        self.recipes.get(id)
    }
}

// TryFrom implementations

impl TryFrom<&Value> for Recipe {
    type Error = JSONParsingError;
    fn try_from(val: &Value) -> Result<Recipe, JSONParsingError> {
        fn get_str_field(val: &Value, fname: &'static str) -> Result<String, JSONParsingError> {
            Ok(val
                .get(fname)
                .ok_or(JSONParsingError::FieldNotExistsError(fname))?
                .as_str()
                .ok_or(JSONParsingError::InvalidType(fname))?
                .to_string())
        }

        fn get_f64_field(val: &Value, fname: &'static str) -> Result<f64, JSONParsingError> {
            Ok(val
                .get(fname)
                .ok_or(JSONParsingError::FieldNotExistsError(fname))?
                .as_f64()
                .ok_or(JSONParsingError::InvalidType(fname))?)
        }

        fn get_ingredients_field(
            val: &Value,
            fname: &'static str,
        ) -> Result<Vec<IngredientProductInfo>, JSONParsingError> {
            let nodes = val
                .get(fname)
                .ok_or(JSONParsingError::FieldNotExistsError(fname))?;
            // if no ingredients/results then sometimes an object gets written and not an array :/
            if (nodes.is_object() && nodes.as_object().unwrap().len() == 0) {
                return Ok(vec![]);
            }
            let nodes = nodes
                .as_array()
                .ok_or(JSONParsingError::InvalidType(fname))?;
            let mut result = vec![];

            for node in nodes {
                let to_add = IngredientProductInfo {
                    type_: get_str_field(&node, "type")?,
                    name: get_str_field(&node, "name")?,
                    amount: get_f64_field(&node, "amount").ok(),
                    amount_min: get_f64_field(&node, "amount_min").ok(),
                    amount_max: get_f64_field(&node, "amount_max").ok(),
                    probability: get_f64_field(&node, "probability").ok(),
                };
                result.push(to_add);
            }
            return Ok(result);
        }

        let id: String = get_str_field(val, "name")?;
        let category: String = get_str_field(val, "category")?;
        let energy: f64 = get_f64_field(val, "energy")?;
        let ingredients: Vec<IngredientProductInfo> = get_ingredients_field(val, "ingredients")?;
        let products: Vec<IngredientProductInfo> = get_ingredients_field(val, "products")?;

        let ingredients_node = return Ok(Recipe {
            id: id,
            category: category,
            energy: energy,
            ingredients: ingredients,
            products: products,
        });
    }
}

impl TryFrom<&Value> for RecipeDatabase {
    type Error = JSONParsingError;
    fn try_from(root: &Value) -> Result<RecipeDatabase, JSONParsingError> {
        let recipes_map = root.as_object().ok_or(JSONParsingError::InvalidType(
            "<root object is not an object?>",
        ))?;
        let mut result: HashMap<String, Recipe> = HashMap::new();
        for recipe_node in recipes_map.values() {
            let recipe: Recipe = recipe_node.try_into()?;
            result.insert(recipe.id.clone(), recipe);
        }
        return Ok(RecipeDatabase { recipes: result });
    }
}
