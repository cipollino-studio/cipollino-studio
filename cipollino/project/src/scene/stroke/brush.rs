
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StrokeBrush {
    Builtin(usize)
}

impl Default for StrokeBrush {

    fn default() -> Self {
        Self::Builtin(0)
    }

} 

impl alisa::Serializable for StrokeBrush {

    fn serialize(&self, _context: &alisa::SerializationContext) -> alisa::ABFValue {
        match self {
            StrokeBrush::Builtin(idx) => alisa::ABFValue::IndexedEnum(0, Box::new(alisa::ABFValue::U64(*idx as u64))),
        }
    }

    fn deserialize(data: &alisa::ABFValue, _context: &mut alisa::DeserializationContext) -> Option<Self> {
        let (variant, data) = data.as_indexed_enum()?;
        match variant {
            0 => {
                let idx = data.as_u64()? as usize;
                Some(Self::Builtin(idx))
            },
            _ => None
        }
    }

}
