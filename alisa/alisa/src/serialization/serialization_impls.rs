
use std::{collections::HashSet, hash::Hash};

use crate::{Ptr, Object, Project};
use super::{Serializable, DeserializationContext, SerializationContext};

macro_rules! number_serializable_impl {
    ($T: ty, $N: ty) => {
        paste::paste! {
            impl<P: Project> Serializable<P> for $T {

                fn deserialize(data: &rmpv::Value, _context: &mut DeserializationContext<P>) -> Option<Self> {
                    data.[< as_ $N >]()?.try_into().ok()
                }

                fn serialize(&self, _context: &SerializationContext<P>) -> rmpv::Value {
                    (*self as $N).into()
                }

            } 
        }
    };
}

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

impl<P: Project> Serializable<P> for f32 {

    fn deserialize(data: &rmpv::Value, _context: &mut DeserializationContext<P>) -> Option<Self> {
        Some(data.as_f64()? as f32)
    }

    fn serialize(&self, _context: &SerializationContext<P>) -> rmpv::Value {
        (*self as f64).into()
    }

}

number_serializable_impl!(f64, f64);

impl<P: Project> Serializable<P> for String {

    fn deserialize(data: &rmpv::Value, _context: &mut DeserializationContext<P>) -> Option<Self> {
        Some(data.as_str()?.to_owned())
    }

    fn serialize(&self, _context: &SerializationContext<P>) -> rmpv::Value {
        self.as_str().into()
    }

}

impl<P: Project, T: Serializable<P>> Serializable<P> for Vec<T> {

    fn deserialize(data: &rmpv::Value, context: &mut DeserializationContext<P>) -> Option<Self> {
        let Some(arr) = data.as_array() else { return Some(Vec::new()); };
        Some(arr.iter().filter_map(|element| T::deserialize(element, context)).collect())
    }

    fn serialize(&self, context: &SerializationContext<P>) -> rmpv::Value {
        rmpv::Value::Array(self.iter().map(|val| val.serialize(context)).collect())
    }

}

impl<P: Project, T: Serializable<P> + Eq + Hash> Serializable<P> for HashSet<T> {

    fn deserialize(data: &rmpv::Value, context: &mut DeserializationContext<P>) -> Option<Self> {
        let Some(arr) = data.as_array() else { return Some(HashSet::new()); };
        Some(arr.iter().filter_map(|element| T::deserialize(element, context)).collect())
    }

    fn serialize(&self, context: &SerializationContext<P>) -> rmpv::Value {
        rmpv::Value::Array(self.iter().map(|val| val.serialize(context)).collect())
    }

}

impl<P: Project> Serializable<P> for () {

    fn serialize(&self, _context: &SerializationContext<P>) -> rmpv::Value {
        rmpv::Value::Nil
    }

    fn deserialize(_data: &rmpv::Value, _context: &mut DeserializationContext<P>) -> Option<Self> {
        Some(())
    }

}

impl<O: Object> Serializable<O::Project> for Ptr<O> {

    fn deserialize(data: &rmpv::Value, _context: &mut DeserializationContext<O::Project>) -> Option<Self> {
        data.as_u64().map(Self::from_key)
    }

    fn serialize(&self, _context: &SerializationContext<O::Project>) -> rmpv::Value {
        self.key.into()
    }

}
