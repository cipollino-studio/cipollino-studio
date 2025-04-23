
mod numbers;

#[derive(PartialEq, Debug, Clone)]
pub enum ABFValue {
    Bool(bool),
    PositiveInt(u8),
    U8(u8), 
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8), 
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Str(String),
    Binary(Box<[u8]>),
    ObjPtr(u16, u64),
    Array(Box<[ABFValue]>),
    Map(Box<[(String, ABFValue)]>),
    IndexedUnitEnum(u8),
    IndexedEnum(u8, Box<ABFValue>),
    NamedUnitEnum(String),
    NamedEnum(String, Box<ABFValue>),
}

impl ABFValue {
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ABFValue::Bool(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            ABFValue::Str(string) => Some(&string),
            _ => None
        }
    }

    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            ABFValue::Binary(binary) => Some(&binary),
            _ => None
        }
    }

    pub fn as_obj_ptr(&self) -> Option<(u16, u64)> {
        match self {
            ABFValue::ObjPtr(obj_type, key) => Some((*obj_type, *key)),
            _ => None
        }
    }

    pub fn as_array(&self) -> Option<&[ABFValue]> {
        match self {
            ABFValue::Array(vals) => Some(&vals),
            _ => None
        }
    }

    pub fn as_map(&self) -> Option<&[(String, ABFValue)]> {
        match self {
            ABFValue::Map(fields) => Some(&fields),
            _ => None
        }
    }

    pub fn as_indexed_unit_enum(&self) -> Option<u8> {
        match self {
            ABFValue::IndexedUnitEnum(idx) => Some(*idx),
            _ => None
        }
    }

    pub fn as_indexed_enum(&self) -> Option<(u8, &ABFValue)> {
        match self {
            ABFValue::IndexedEnum(idx, data) => Some((*idx, &data)),
            _ => None
        }
    }

    pub fn as_named_unit_enum(&self) -> Option<&str> {
        match self {
            ABFValue::NamedUnitEnum(name) => Some(&name),
            _ => None
        }
    }

    pub fn as_named_enum(&self) -> Option<(&str, &ABFValue)> {
        match self {
            ABFValue::NamedEnum(name, data) => Some((&name, data)),
            _ => None
        }
    }

    pub fn get(&self, key: &str) -> Option<&ABFValue> {
        let map = self.as_map()?;
        for (name, val) in map {
            if name == key {
                return Some(val);
            }
        }
        None
    }

}

impl From<bool> for ABFValue {

    fn from(value: bool) -> Self {
        Self::Bool(value)
    }

}

impl From<&str> for ABFValue {

    fn from(value: &str) -> Self {
        Self::Str(value.to_owned())
    }

}
