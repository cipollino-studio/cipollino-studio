
use crate::{ABFValue, Decoder};

impl Decoder<'_> {

    pub fn parse(&mut self) -> Option<ABFValue> {
        if let Some(x) = self.read_bool() {
            return Some(ABFValue::Bool(x));
        } else if let Some(x) = self.read_positive_int() {
            return Some(ABFValue::PositiveInt(x));
        } else if let Some(x) = self.read_u8() {
            return Some(ABFValue::U8(x));
        } else if let Some(x) = self.read_u16() {
            return Some(ABFValue::U16(x));
        } else if let Some(x) = self.read_u32() {
            return Some(ABFValue::U32(x));
        } else if let Some(x) = self.read_u64() {
            return Some(ABFValue::U64(x));
        } else if let Some(x) = self.read_i8() {
            return Some(ABFValue::I8(x));
        } else if let Some(x) = self.read_i16() {
            return Some(ABFValue::I16(x));
        } else if let Some(x) = self.read_i32() {
            return Some(ABFValue::I32(x));
        } else if let Some(x) = self.read_i64() {
            return Some(ABFValue::I64(x));
        } else if let Some(x) = self.read_f32() {
            return Some(ABFValue::F32(x));
        } else if let Some(x) = self.read_f64() {
            return Some(ABFValue::F64(x));
        } else if let Some(str) = self.read_string() {
            return Some(ABFValue::Str(str.into()));
        } else if let Some(binary) = self.read_binary() {
            return Some(ABFValue::Binary(binary.into()));
        } else if let Some((obj_type, key)) = self.read_obj_ptr() {
            return Some(ABFValue::ObjPtr(obj_type, key));
        } else if let Some(arr_len) = self.read_array_length() {
            let items = (0..arr_len).filter_map(|_| {
                self.parse()
            });
            return Some(ABFValue::Array(items.collect()));
        } else if let Some(map_len) = self.read_map_length() {
            let items = (0..map_len).filter_map(|_| {
                let name = self.read_map_field_name()?.to_owned();
                let data = self.parse()?;
                Some((name, data))
            });
            return Some(ABFValue::Map(items.collect()));
        } else if let Some(idx) = self.read_indexed_unit_enum() {
            return Some(ABFValue::IndexedUnitEnum(idx));
        } else if let Some(idx) = self.read_indexed_enum() {
            return Some(ABFValue::IndexedEnum(idx, Box::new(self.parse()?)));
        } else if let Some(name) = self.read_named_unit_enum() {
            return Some(ABFValue::NamedUnitEnum(name.into()));
        } else if let Some(name) = self.read_named_enum() {
            return Some(ABFValue::NamedEnum(name.into(), Box::new(self.parse()?)));
        }
        None
    }

}

pub fn parse_abf(bytes: &[u8]) -> Option<ABFValue> {
    Decoder::new(bytes).parse()
}
