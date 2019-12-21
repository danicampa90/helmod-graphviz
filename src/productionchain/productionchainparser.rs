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

impl TryFrom<&LuaObject> for ProductionChain {
    type Error = ConversionError;

    fn try_from(luaobj: &LuaObject) -> Result<Self, Self::Error> {
        // products
        let mut products = vec![];
        let prod_it = luaobj
            .get_with_str("products".to_string())?
            .as_object()?
            .itervalues();
        for prod_info in prod_it {
            let prod_info_obj = prod_info.as_object()?;
            let count = prod_info_obj.get_with_str("count".into())?.as_float()?;
            let name = prod_info_obj
                .get_with_str("name".into())?
                .as_string()?
                .clone();
            products.push((name, count));
        }

        // ingredients
        let mut ingredients = vec![];
        let ingr_it = luaobj
            .get_with_str("ingredients".to_string())?
            .as_object()?
            .itervalues();
        for ingr_info in ingr_it {
            let ingr_info_obj = ingr_info.as_object()?;
            let count = ingr_info_obj.get_with_str("count".into())?.as_float()?;
            let name = ingr_info_obj
                .get_with_str("name".into())?
                .as_string()?
                .clone();
            ingredients.push((name, count));
        }

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
        let name = luaobj
            .get_with_str("name".to_string())?
            .as_string()?
            .clone();
        let count = luaobj
            .get_with_str("count".to_string())?
            .as_float()?
            .clone();

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
            ingredients: vec![],
            recipes: recipes,
        })
    }
}
impl TryFrom<&LuaObject> for ProductionRecipe {
    type Error = ConversionError;

    fn try_from(luaobj: &LuaObject) -> Result<Self, Self::Error> {
        todo!()
        /*
        Ok(ProductionRecipe {
            id: id,
            name: name,
            type_: type_,
            output_count: output_count,
            speed: speed,
            factory_name: factory_name,
            factory_count: factory_count,
            factory_speed: factory_speed,
            modules: vec![],
            beacons: vec![],
        })*/
    }
}
