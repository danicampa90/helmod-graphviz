use serde_json::Value;
use std::convert::TryFrom;
use std::convert::TryInto;

struct IngredientProductInfo {
    type_: String,
    name: String,
    amount: Option<f64>,
    amount_min: Option<f64>,
    amount_max: Option<f64>,
    probability: Option<f64>,
}

struct Recipe {
    id: String,
    category: String,
    energy: f64,
    ingredients: Vec<IngredientProductInfo>,
    products: Vec<IngredientProductInfo>,
}

pub struct RecipeDatabase {
    recipes: Vec<Recipe>,
}
#[derive(Debug)]
pub enum JSONParsingError {
    FieldNotExistsError(&'static str),
    InvalidType(&'static str),
}

// : Value = serde_json::from_str

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
        let mut result = vec![];
        for recipe_node in recipes_map.values() {
            result.push(recipe_node.try_into()?);
        }
        return Ok(RecipeDatabase { recipes: result });
    }
}
