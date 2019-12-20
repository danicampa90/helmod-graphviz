use std::str::FromStr;

pub struct ProductionChain {
    pub id: String,
    pub owner: String,
    pub blocks: Vec<ProductionBlock>,
}

pub struct ProductionBlock {
    pub id: String,
    pub name: String,
    pub power: i32,
    pub ingredients: Vec<(String, i32)>,
    pub recipes: Vec<()>,
}

//// This is rest of the stuff
///
///
#[derive(Debug)]
pub struct LuaObject {
    props: Vec<LuaProperty>,
}

#[derive(Debug)]
pub struct LuaProperty {
    name: String,
    value: LuaValue,
}

#[derive(Debug)]
pub enum LuaValue {
    Number(String),
    String(String),
    Object(LuaObject),
}

impl LuaObject {
    pub fn new(properties: Vec<LuaProperty>) -> LuaObject {
        LuaObject { props: properties }
    }
}

impl LuaProperty {
    pub fn new(name: String, value: LuaValue) -> LuaProperty {
        LuaProperty {
            name: name,
            value: value,
        }
    }
}
