
use super::ABFValue;

impl ABFValue {

    pub fn as_u8(&self) -> Option<u8> {
        match self {
            ABFValue::PositiveInt(val) => Some(*val),
            ABFValue::U8(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_u16(&self) -> Option<u16> {
        match self {
            ABFValue::PositiveInt(val) => Some(*val as u16),
            ABFValue::U8(val) => Some(*val as u16),
            ABFValue::U16(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_u32(&self) -> Option<u32> {
        match self {
            ABFValue::PositiveInt(val) => Some(*val as u32),
            ABFValue::U8(val) => Some(*val as u32),
            ABFValue::U16(val) => Some(*val as u32),
            ABFValue::U32(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            ABFValue::PositiveInt(val) => Some(*val as u64),
            ABFValue::U8(val) => Some(*val as u64),
            ABFValue::U16(val) => Some(*val as u64),
            ABFValue::U32(val) => Some(*val as u64),
            ABFValue::U64(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_i8(&self) -> Option<i8> {
        match self {
            ABFValue::PositiveInt(val) => Some(*val as i8),
            ABFValue::I8(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_i16(&self) -> Option<i16> {
        match self {
            ABFValue::PositiveInt(val) => Some(*val as i16),
            ABFValue::I8(val) => Some(*val as i16),
            ABFValue::I16(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_i32(&self) -> Option<i32> {
        match self {
            ABFValue::PositiveInt(val) => Some(*val as i32),
            ABFValue::I8(val) => Some(*val as i32),
            ABFValue::I16(val) => Some(*val as i32),
            ABFValue::I32(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            ABFValue::PositiveInt(val) => Some(*val as i64),
            ABFValue::I8(val) => Some(*val as i64),
            ABFValue::I16(val) => Some(*val as i64),
            ABFValue::I32(val) => Some(*val as i64),
            ABFValue::I64(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_f32(&self) -> Option<f32> {
        match self {
            ABFValue::F32(val) => Some(*val),
            _ => None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            ABFValue::F64(val) => Some(*val),
            _ => None
        }
    }

}

impl From<u8> for ABFValue {

    fn from(value: u8) -> Self {
        Self::U8(value)
    }

}

impl From<u16> for ABFValue {

    fn from(value: u16) -> Self {
        Self::U16(value)
    }

}

impl From<u32> for ABFValue {

    fn from(value: u32) -> Self {
        Self::U32(value)
    }

}

impl From<u64> for ABFValue {

    fn from(value: u64) -> Self {
        Self::U64(value)
    }

}

impl From<i8> for ABFValue {

    fn from(value: i8) -> Self {
        Self::I8(value)
    }

}

impl From<i16> for ABFValue {

    fn from(value: i16) -> Self {
        Self::I16(value)
    }

}

impl From<i32> for ABFValue {

    fn from(value: i32) -> Self {
        Self::I32(value)
    }

}

impl From<i64> for ABFValue {

    fn from(value: i64) -> Self {
        Self::I64(value)
    }

}

impl From<f32> for ABFValue {

    fn from(value: f32) -> Self {
        Self::F32(value)
    }

}

impl From<f64> for ABFValue {

    fn from(value: f64) -> Self {
        Self::F64(value)
    }

}
