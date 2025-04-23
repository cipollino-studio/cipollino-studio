
use crate::ABFValue;

use super::Encoder;
use super::Result;

impl<W: std::io::Write> Encoder<'_, W> {

    pub fn value(&mut self, value: &ABFValue) -> Result {
        match value {
            ABFValue::Bool(bool) => self.boolean(*bool),
            ABFValue::PositiveInt(val) => self.u8(*val),
            ABFValue::U8(u8) => self.u8(*u8),
            ABFValue::U16(u16) => self.u16(*u16),
            ABFValue::U32(u32) => self.u32(*u32),
            ABFValue::U64(u64) => self.u64(*u64),
            ABFValue::I8(i8) => self.i8(*i8),
            ABFValue::I16(i16) => self.i16(*i16),
            ABFValue::I32(i32) => self.i32(*i32),
            ABFValue::I64(i64) => self.i64(*i64),
            ABFValue::F32(f32) => self.f32(*f32),
            ABFValue::F64(f64) => self.f64(*f64),
            ABFValue::Str(str) => self.string(&str),
            ABFValue::Binary(data) => self.binary(&data),
            ABFValue::ObjPtr(obj_type, key) => self.obj_ptr(*obj_type, *key),
            ABFValue::Array(items) => {
                self.array(items.len() as u32)?;
                for item in items {
                    self.value(item)?;
                }
                Ok(())
            },
            ABFValue::Map(items) => {
                self.map(items.len() as u32)?;
                for (name, value) in items {
                    self.map_field(&name, |encoder| {
                        encoder.value(value)
                    })?;
                }
                Ok(())
            },
            ABFValue::IndexedUnitEnum(idx) => self.indexed_unit_enum(*idx),
            ABFValue::IndexedEnum(idx, data) => self.indexed_enum(*idx, |encoder| encoder.value(&data)),
            ABFValue::NamedUnitEnum(name) => self.named_unit_enum(name),
            ABFValue::NamedEnum(name, data) => self.named_enum(name, |encoder| encoder.value(&data)),
        }
    }

}

pub fn encode_abf(value: &ABFValue) -> Vec<u8> {
    let mut data = Vec::new();
    let _ = Encoder::new(&mut data).value(value);
    data
}
