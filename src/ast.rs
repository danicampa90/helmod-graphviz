use std::str::FromStr;

//// This is rest of the stuff
///
///
#[derive(Debug, Clone)]
pub struct LuaObject {
    props: Vec<LuaProperty>,
}
#[derive(Debug, Clone)]
pub struct LuaArray {
    vals: Vec<LuaValue>,
}

#[derive(Debug, Clone)]
pub struct LuaProperty {
    name: LuaValue,
    value: LuaValue,
}

#[derive(Debug, Clone)]
pub enum LuaValue {
    Number(String),
    String(String),
    Identifier(String),
    Object(LuaObject),
    Array(LuaArray),
}

pub enum CastError {
    NotAString(LuaValue),
    NotAnInteger(LuaValue),
}

//// Impls
impl LuaObject {
    pub fn new(properties: Vec<LuaProperty>) -> LuaObject {
        LuaObject { props: properties }
    }
}

//impl Index for LuaObject // support [luaValue] -> luaValue syntax

impl LuaValue {
    pub fn as_integer(&self) -> Result<i64, CastError> {
        match self {
            LuaValue::Number(x) => {
                FromStr::from_str(&x).map_err(|_| CastError::NotAnInteger(self.clone()))
            }
            _ => Err(CastError::NotAnInteger(self.clone())),
        }
    }

    pub fn as_string(&self) -> Result<String, CastError> {
        todo!()
    }

    pub fn as_float(&self) -> Result<f64, CastError> {
        todo!()
    }

    pub fn as_string_array(&self) -> Result<Vec<String>, CastError> {
        match self {
            LuaValue::Array(input) => {
                let mut out = vec![];
                for item in &input.vals {
                    match item {
                        LuaValue::String(s) => out.push(s.clone()),
                        _ => return Err(CastError::NotAString(item.clone())),
                    }
                }
                return Ok(out);
            }
            _ => return Err(CastError::NotAString(self.clone())),
        }
    }
}

impl LuaArray {
    pub fn new(values: Vec<LuaValue>) -> LuaArray {
        LuaArray { vals: values }
    }
}

impl LuaProperty {
    pub fn new(name: LuaValue, value: LuaValue) -> LuaProperty {
        LuaProperty {
            name: name,
            value: value,
        }
    }
}
