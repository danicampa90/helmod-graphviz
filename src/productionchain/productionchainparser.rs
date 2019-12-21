use crate::ast::{CastError, IndexingError, LuaArray, LuaObject, LuaValue};
use crate::productionchain::{ProductionBlock, ProductionChain, ProductionRecipe};
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug)]
pub enum ConversionError {
    CastError(CastError),
    IndexingError(IndexingError),
}

impl From<CastError> for ConversionError {
    fn from(err: CastError) -> ConversionError {
        ConversionError::CastError(err)
    }
}

impl From<IndexingError> for ConversionError {
    fn from(err: IndexingError) -> ConversionError {
        ConversionError::IndexingError(err)
    }
}

fn parse_ingredients_list(
    luaobj: &LuaObject,
    node_name: String,
) -> Result<Vec<(String, f64)>, ConversionError> {
    let mut ingredients = vec![];
    let ingr_it = luaobj.get_with_str(node_name)?.as_object()?.itervalues();
    for ingr_info in ingr_it {
        let ingr_info_obj = ingr_info.as_object()?;
        let count = ingr_info_obj.get_with_str("count".into())?.as_float()?;
        let name = ingr_info_obj
            .get_with_str("name".into())?
            .as_string()?
            .clone();
        ingredients.push((name, count));
    }
    return Ok(ingredients);
}

impl TryFrom<&LuaObject> for ProductionChain {
    type Error = ConversionError;

    fn try_from(luaobj: &LuaObject) -> Result<Self, Self::Error> {
        let products = parse_ingredients_list(luaobj, "products".to_string())?;
        let ingredients = parse_ingredients_list(luaobj, "ingredients".to_string())?;

        // blocks
        let mut blocks = vec![];
        let blocks_it = luaobj
            .get_with_str("blocks".to_string())?
            .as_object()?
            .itervalues();

        for block_info in blocks_it {
            blocks.push(block_info.as_object()?.try_into()?)
        }

        Ok(ProductionChain {
            products: products,
            ingredients: ingredients,
            blocks: blocks,
        })
    }
}

impl TryFrom<&LuaObject> for ProductionBlock {
    type Error = ConversionError;

    fn try_from(luaobj: &LuaObject) -> Result<Self, Self::Error> {
        let id = luaobj.get_with_str("id".to_string())?.as_string()?.clone();
        let count = luaobj.get_with_str("count".to_string())?.as_float()?;
        let name = luaobj
            .get_with_str("name".to_string())?
            .as_string()?
            .clone();

        let ingredients = parse_ingredients_list(luaobj, "ingredients".to_string())?;
        // recipes
        let mut recipes = vec![];
        let recipes_it = luaobj
            .get_with_str("recipes".to_string())?
            .as_object()?
            .itervalues();

        for recipe_info in recipes_it {
            recipes.push(recipe_info.as_object()?.try_into()?)
        }

        Ok(ProductionBlock {
            id: id,
            name: name,
            count: count,
            ingredients: ingredients,
            recipes: recipes,
        })
    }
}
impl TryFrom<&LuaObject> for ProductionRecipe {
    type Error = ConversionError;

    fn try_from(luaobj: &LuaObject) -> Result<Self, Self::Error> {
        let factory_data = luaobj.get_with_str("factory".to_string())?.as_object()?;

        Ok(ProductionRecipe {
            id: luaobj.get_with_str("id".to_string())?.as_string()?.clone(),
            name: luaobj
                .get_with_str("name".to_string())?
                .as_string()?
                .clone(),
            type_: luaobj
                .get_with_str("type".to_string())?
                .as_string()?
                .clone(),
            output_count: luaobj.get_with_str("count".to_string())?.as_float()?,
            factory_name: factory_data
                .get_with_str("name".to_string())?
                .as_string()?
                .clone(),
            factory_count: factory_data.get_with_str("count".to_string())?.as_float()?,
            factory_speed: factory_data.get_with_str("speed".to_string())?.as_float()?,
        })
    }
}
