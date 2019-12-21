use crate::ast::{CastError, IndexingError, LuaArray, LuaObject, LuaValue};
use crate::productionchain::ProductionChain;
use std::convert::TryFrom;

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
        let mut products = vec![];
        let mut ingredients = vec![];
        let mut blocks = vec![];
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

        Ok(ProductionChain {
            products: products,
            ingredients: ingredients,
            blocks: blocks,
        })
    }
}
