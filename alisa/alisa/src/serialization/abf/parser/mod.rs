
use super::Decoder;

#[derive(PartialEq, Debug)]
pub enum ABFNode {
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
    Array(Box<[ABFNode]>),
    Map(Box<[(String, ABFNode)]>),
    IndexedUnitEnum(u8),
    IndexedEnum(u8, Box<ABFNode>),
    NamedUnitEnum(String),
    NamedEnum(String, Box<ABFNode>),
}

impl Decoder<'_> {

    pub fn parse(&mut self) -> Option<ABFNode> {
        if let Some(x) = self.read_bool() {
            return Some(ABFNode::Bool(x));
        } else if let Some(x) = self.read_positive_int() {
            return Some(ABFNode::PositiveInt(x));
        } else if let Some(x) = self.read_u8() {
            return Some(ABFNode::U8(x));
        } else if let Some(x) = self.read_u16() {
            return Some(ABFNode::U16(x));
        } else if let Some(x) = self.read_u32() {
            return Some(ABFNode::U32(x));
        } else if let Some(x) = self.read_u64() {
            return Some(ABFNode::U64(x));
        } else if let Some(x) = self.read_i8() {
            return Some(ABFNode::I8(x));
        } else if let Some(x) = self.read_i16() {
            return Some(ABFNode::I16(x));
        } else if let Some(x) = self.read_i32() {
            return Some(ABFNode::I32(x));
        } else if let Some(x) = self.read_i64() {
            return Some(ABFNode::I64(x));
        } else if let Some(x) = self.read_f32() {
            return Some(ABFNode::F32(x));
        } else if let Some(x) = self.read_f64() {
            return Some(ABFNode::F64(x));
        } else if let Some(str) = self.read_string() {
            return Some(ABFNode::Str(str.into()));
        } else if let Some(binary) = self.read_binary() {
            return Some(ABFNode::Binary(binary.into()));
        } else if let Some((obj_type, key)) = self.read_obj_ptr() {
            return Some(ABFNode::ObjPtr(obj_type, key));
        } else if let Some(arr_len) = self.read_array_length() {
            let items = (0..arr_len).filter_map(|_| {
                self.parse()
            });
            return Some(ABFNode::Array(items.collect()));
        } else if let Some(map_len) = self.read_map_length() {
            let items = (0..map_len).filter_map(|_| {
                Some((self.read_map_field_name()?.to_owned(), self.parse()?))
            });
            return Some(ABFNode::Map(items.collect()));
        } else if let Some(idx) = self.read_indexed_unit_enum() {
            return Some(ABFNode::IndexedUnitEnum(idx));
        } else if let Some(idx) = self.read_indexed_enum() {
            return Some(ABFNode::IndexedEnum(idx, Box::new(self.parse()?)));
        } else if let Some(name) = self.read_named_unit_enum() {
            return Some(ABFNode::NamedUnitEnum(name.into()));
        } else if let Some(name) = self.read_named_enum() {
            return Some(ABFNode::NamedEnum(name.into(), Box::new(self.parse()?)));
        }
        None
    }

}

pub fn parse_abf(bytes: &[u8]) -> Option<ABFNode> {
    Decoder::new(bytes).parse()
}
