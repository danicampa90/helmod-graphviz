use std::ops::Index;
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
    key: LuaValue,
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

#[derive(Debug)]
pub enum CastError {
    NotAString(LuaValue),
    NotAnInteger(LuaValue),
    NotAFloat(LuaValue),
    NotAnArray(LuaValue),
}

#[derive(Debug)]
pub enum IndexingError {
    FieldNotExists(LuaValue),
    FieldNotExistsStr(String),
    IndexOutOfRange(usize, usize), // (accessed, total)
}

//// Impls
impl LuaObject {
    pub fn new(properties: Vec<LuaProperty>) -> LuaObject {
        LuaObject { props: properties }
    }

    pub fn get<'a>(&'a self, val: &LuaValue) -> Result<&LuaValue, IndexingError> {
        for prop in &self.props {
            if prop.key == *val {
                return Ok(&prop.value);
            }
        }
        return Err(IndexingError::FieldNotExists(val.clone()));
    }

    pub fn get_with_str(&self, val: String) -> Result<&LuaValue, IndexingError> {
        match self.get(&LuaValue::Identifier(val.clone())) {
            Ok(x) => return Ok(x),
            _ => (),
        }
        match self.get(&LuaValue::String(val.clone())) {
            Ok(x) => return Ok(x),
            _ => (),
        }
        match self.get(&LuaValue::Array(LuaArray::from_single_string(val.clone()))) {
            Ok(x) => return Ok(x),
            _ => (),
        }
        return Err(IndexingError::FieldNotExistsStr(val));
    }

    pub fn iterkeys(&self) -> impl Iterator<Item = &LuaValue> {
        self.props.iter().map(|v| &v.key)
    }

    pub fn itervalues(&self) -> impl Iterator<Item = &LuaValue> {
        self.props.iter().map(|v| &v.value)
    }

    pub fn iteritems(&self) -> impl Iterator<Item = &LuaProperty> {
        self.props.iter()
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

    pub fn as_float(&self) -> Result<f64, CastError> {
        match self {
            LuaValue::Number(x) => {
                FromStr::from_str(&x).map_err(|_| CastError::NotAFloat(self.clone()))
            }
            _ => Err(CastError::NotAFloat(self.clone())),
        }
    }

    pub fn as_string(&self) -> Result<&String, CastError> {
        match self {
            LuaValue::String(x) => Ok(x),
            _ => Err(CastError::NotAString(self.clone())),
        }
    }

    pub fn as_string_array(&self) -> Result<Vec<&String>, CastError> {
        match self {
            LuaValue::Array(input) => {
                let mut out = vec![];
                for item in &input.vals {
                    match item {
                        LuaValue::String(s) => out.push(s),
                        _ => return Err(CastError::NotAString(item.clone())),
                    }
                }
                return Ok(out);
            }
            _ => return Err(CastError::NotAnArray(self.clone())),
        }
    }

    pub fn as_array(&self) -> Result<&LuaArray, CastError> {
        match self {
            LuaValue::Array(x) => Ok(&x),
            _ => Err(CastError::NotAnArray(self.clone())),
        }
    }

    pub fn as_object(&self) -> Result<&LuaObject, CastError> {
        match self {
            LuaValue::Object(x) => Ok(&x),
            _ => Err(CastError::NotAnArray(self.clone())),
        }
    }
}

impl LuaArray {
    pub fn new(values: Vec<LuaValue>) -> LuaArray {
        LuaArray { vals: values }
    }

    pub fn get<'a>(&'a self, idx: usize) -> Result<&'a LuaValue, IndexingError> {
        self.vals
            .get(idx)
            .ok_or(IndexingError::IndexOutOfRange(idx, self.vals.len()))
    }

    pub fn from_single_string(s: String) -> LuaArray {
        Self::new(vec![LuaValue::String(s)])
    }
}

impl LuaProperty {
    pub fn new(key: LuaValue, value: LuaValue) -> LuaProperty {
        LuaProperty {
            key: key,
            value: value,
        }
    }
}

// partialeqs

impl PartialEq for LuaValue {
    fn eq(&self, other: &LuaValue) -> bool {
        match (self, other) {
            (LuaValue::Number(me), LuaValue::Number(other)) => me == other,
            (LuaValue::String(me), LuaValue::String(other)) => me == other,
            (LuaValue::Identifier(me), LuaValue::Identifier(other)) => me == other,
            (LuaValue::Object(me), LuaValue::Object(other)) => me == other,
            (LuaValue::Array(me), LuaValue::Array(other)) => me == other,
            _ => false,
        }
    }
}

impl PartialEq for LuaObject {
    fn eq(&self, other: &LuaObject) -> bool {
        if self.props.len() != other.props.len() {
            return false;
        }

        let mut both_iterators = self.props.iter().zip(other.props.iter());
        return both_iterators.all(|(me, other)| me.key == other.key && me.value == other.value);
    }
}

impl PartialEq for LuaArray {
    fn eq(&self, other: &LuaArray) -> bool {
        if self.vals.len() != other.vals.len() {
            return false;
        }

        let mut both_iterators = self.vals.iter().zip(other.vals.iter());
        return both_iterators.all(|(me, other)| me == other);
    }
}

// index

impl<'a> Index<&'a LuaValue> for LuaObject {
    type Output = LuaValue;
    fn index(&self, idx: &LuaValue) -> &<Self as std::ops::Index<&'a LuaValue>>::Output {
        self.get(idx).expect("Value not found in Lua Dict")
    }
}

impl<'a> Index<usize> for LuaArray {
    type Output = LuaValue;
    fn index(&self, idx: usize) -> &<Self as std::ops::Index<usize>>::Output {
        self.get(idx).expect("Value not found in in Lua Array")
    }
}
