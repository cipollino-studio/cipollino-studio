
use std::{collections::HashSet, hash::Hash};

use crate::{Ptr, Object};
use super::{Serializable, DeserializationContext, SerializationContext};

macro_rules! number_serializable_impl {
    ($T: ty, $N: ty) => {
        paste::paste! {
            impl Serializable for $T {

                fn deserialize(data: &rmpv::Value, _context: &mut DeserializationContext) -> Option<Self> {
                    data.[< as_ $N >]()?.try_into().ok()
                }

                fn serialize(&self, _context: &SerializationContext) -> rmpv::Value {
                    (*self as $N).into()
                }

            } 
        }
    };
}

number_serializable_impl!(bool, bool);
number_serializable_impl!(i8,  i64);
number_serializable_impl!(i16, i64);
number_serializable_impl!(i32, i64);
number_serializable_impl!(i64, i64);
number_serializable_impl!(isize, i64);
number_serializable_impl!(u8,  u64);
number_serializable_impl!(u16, u64);
number_serializable_impl!(u32, u64);
number_serializable_impl!(u64, u64);
number_serializable_impl!(usize, u64);

impl Serializable for f32 {

    fn deserialize(data: &rmpv::Value, _context: &mut DeserializationContext) -> Option<Self> {
        Some(data.as_f64()? as f32)
    }

    fn serialize(&self, _context: &SerializationContext) -> rmpv::Value {
        (*self as f64).into()
    }

}

number_serializable_impl!(f64, f64);

impl Serializable for String {

    fn deserialize(data: &rmpv::Value, _context: &mut DeserializationContext) -> Option<Self> {
        Some(data.as_str()?.to_owned())
    }

    fn serialize(&self, _context: &SerializationContext) -> rmpv::Value {
        self.as_str().into()
    }

}

impl<T: Serializable> Serializable for Option<T> {

    fn serialize(&self, context: &SerializationContext) -> rmpv::Value {
        match self {
            Some(value) => rmpv::Value::Array(vec![value.serialize(context)]),
            None => rmpv::Value::Nil,
        }
    }

    fn deserialize(data: &rmpv::Value, context: &mut DeserializationContext) -> Option<Self> {
        if data.is_nil() {
            return Some(None);
        }

        let data = data.as_array()?;
        let data = data.get(0)?;
        Some(T::deserialize(data, context))
    }

}

impl<T: Serializable, const N: usize> Serializable for [T; N] {

    fn serialize(&self, context: &SerializationContext) -> rmpv::Value {
        rmpv::Value::Array(self.iter().map(|val| val.serialize(context)).collect())
    }

    fn deserialize(data: &rmpv::Value, context: &mut DeserializationContext) -> Option<Self> {
        let Some(arr) = data.as_array() else { return None; };
        if arr.len() != N {
            return None;
        } 
        let mut deserialized = Vec::new(); 
        for i in 0..arr.len() {
            deserialized.push(T::deserialize(&arr[i], context)?);
        }
        let mut deserialized = deserialized.into_iter();
        Some(std::array::from_fn(|_| deserialized.next().unwrap()))
    }

}

impl<T: Serializable> Serializable for Vec<T> {

    fn deserialize(data: &rmpv::Value, context: &mut DeserializationContext) -> Option<Self> {
        let Some(arr) = data.as_array() else { return Some(Vec::new()); };
        Some(arr.iter().filter_map(|element| T::deserialize(element, context)).collect())
    }

    fn serialize(&self, context: &SerializationContext) -> rmpv::Value {
        rmpv::Value::Array(self.iter().map(|val| val.serialize(context)).collect())
    }

}

impl<T: Serializable + Eq + Hash> Serializable for HashSet<T> {

    fn deserialize(data: &rmpv::Value, context: &mut DeserializationContext) -> Option<Self> {
        let Some(arr) = data.as_array() else { return Some(HashSet::new()); };
        Some(arr.iter().filter_map(|element| T::deserialize(element, context)).collect())
    }

    fn serialize(&self, context: &SerializationContext) -> rmpv::Value {
        rmpv::Value::Array(self.iter().map(|val| val.serialize(context)).collect())
    }

}

impl Serializable for () {

    fn serialize(&self, _context: &SerializationContext) -> rmpv::Value {
        rmpv::Value::Nil
    }

    fn deserialize(_data: &rmpv::Value, _context: &mut DeserializationContext) -> Option<Self> {
        Some(())
    }

}

impl<O: Object> Serializable for Ptr<O> {

    fn deserialize(data: &rmpv::Value, _context: &mut DeserializationContext) -> Option<Self> {
        data.as_u64().map(Self::from_key)
    }

    fn serialize(&self, _context: &SerializationContext) -> rmpv::Value {
        self.key.into()
    }

}
