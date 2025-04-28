
use std::{collections::HashSet, hash::Hash};

use crate::{ABFValue, AnyPtr, Object, Ptr};
use super::{Serializable, DeserializationContext, SerializationContext};

macro_rules! number_serializable_impl {
    ($T: ty, $N: ty) => {
        paste::paste! {
            impl Serializable for $T {

                fn deserialize(data: &ABFValue, _context: &mut DeserializationContext) -> Option<Self> {
                    data.[< as_ $N >]()?.try_into().ok()
                }

                fn serialize(&self, _context: &SerializationContext) -> ABFValue {
                    (*self as $N).into()
                }

            } 
        }
    };
}

number_serializable_impl!(bool, bool);
number_serializable_impl!(i8,  i8);
number_serializable_impl!(i16, i16);
number_serializable_impl!(i32, i32);
number_serializable_impl!(i64, i64);
number_serializable_impl!(isize, i64);
number_serializable_impl!(u8,  u8);
number_serializable_impl!(u16, u16);
number_serializable_impl!(u32, u32);
number_serializable_impl!(u64, u64);
number_serializable_impl!(usize, u64);
number_serializable_impl!(f32, f32);
number_serializable_impl!(f64, f64);

impl Serializable for String {

    fn deserialize(data: &ABFValue, _context: &mut DeserializationContext) -> Option<Self> {
        Some(data.as_string()?.to_owned())
    }

    fn serialize(&self, _context: &SerializationContext) -> ABFValue {
        ABFValue::Str(self.into())
    }

}

impl<T: Serializable> Serializable for Option<T> {

    fn serialize(&self, context: &SerializationContext) -> ABFValue {
        match self {
            Some(value) => ABFValue::IndexedEnum(0, Box::new(value.serialize(context))),
            None => ABFValue::IndexedUnitEnum(0),
        }
    }

    fn deserialize(data: &ABFValue, context: &mut DeserializationContext) -> Option<Self> {
        match data {
            ABFValue::IndexedUnitEnum(_) => Some(None),
            ABFValue::IndexedEnum(_, data) => Some(Some(T::deserialize(data, context)?)),
            _ => None
        } 
    }

}

impl<T: Serializable, const N: usize> Serializable for [T; N] {

    fn serialize(&self, context: &SerializationContext) -> ABFValue {
        ABFValue::Array(self.iter().map(|val| val.serialize(context)).collect())
    }

    fn deserialize(data: &ABFValue, context: &mut DeserializationContext) -> Option<Self> {
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

    fn deserialize(data: &ABFValue, context: &mut DeserializationContext) -> Option<Self> {
        let Some(arr) = data.as_array() else { return Some(Vec::new()); };
        Some(arr.iter().filter_map(|element| T::deserialize(element, context)).collect())
    }

    fn serialize(&self, context: &SerializationContext) -> ABFValue {
        ABFValue::Array(self.iter().map(|val| val.serialize(context)).collect())
    }

}

impl<T: Serializable + Eq + Hash> Serializable for HashSet<T> {

    fn deserialize(data: &ABFValue, context: &mut DeserializationContext) -> Option<Self> {
        let Some(arr) = data.as_array() else { return Some(HashSet::new()); };
        Some(arr.iter().filter_map(|element| T::deserialize(element, context)).collect())
    }

    fn serialize(&self, context: &SerializationContext) -> ABFValue {
        ABFValue::Array(self.iter().map(|val| val.serialize(context)).collect())
    }

}

impl Serializable for () {

    fn serialize(&self, _context: &SerializationContext) -> ABFValue {
        ABFValue::PositiveInt(0)
    }

    fn deserialize(_data: &ABFValue, _context: &mut DeserializationContext) -> Option<Self> {
        Some(())
    }

}

impl<O: Object> Serializable for Ptr<O> {

    fn deserialize(data: &ABFValue, _context: &mut DeserializationContext) -> Option<Self> {
        let (obj_type, key) = data.as_obj_ptr()?;
        if obj_type != O::TYPE_ID {
            return None;
        }
        Some(Self::from_key(key))
    }

    fn serialize(&self, _context: &SerializationContext) -> ABFValue {
        ABFValue::ObjPtr(O::TYPE_ID, self.key)
    }

}

impl Serializable for AnyPtr {

    fn serialize(&self, _context: &SerializationContext) -> ABFValue {
        ABFValue::ObjPtr(self.obj_type(), self.key())
    }

    fn deserialize(data: &ABFValue, _context: &mut DeserializationContext) -> Option<Self> {
        let (obj_type, key) = data.as_obj_ptr()?;
        Some(Self::new(obj_type, key))
    }

}

impl Serializable for ABFValue {

    fn serialize(&self, _context: &SerializationContext) -> ABFValue {
        self.clone()
    }

    fn deserialize(data: &ABFValue, _context: &mut DeserializationContext) -> Option<Self> {
        Some(data.clone())
    }

}

macro_rules! tuple_serializable {
    ($($t: ident),*) => {

        impl<$($t: Serializable),*> Serializable for ($($t),*) {

            fn serialize(&self, context: &SerializationContext) -> ABFValue {
                #![allow(non_snake_case)]
                let ($($t),*) = self;
                ABFValue::Array(Box::new([
                    $($t.serialize(context)),*
                ]))
            }

            fn deserialize(data: &ABFValue, context: &mut DeserializationContext) -> Option<Self> {
                #![allow(non_snake_case)]
                let mut arr = data.as_array()?;
                $(
                    let $t = arr.get(0)?; 
                    let $t = $t::deserialize($t, context)?;
                    arr = &arr[1..];
                )*
                if !arr.is_empty() {
                    return None;
                }
                Some(($($t),*))
            }

        }

    };
}

tuple_serializable!(A, B);
tuple_serializable!(A, B, C);
tuple_serializable!(A, B, C, D);
tuple_serializable!(A, B, C, D, E);
tuple_serializable!(A, B, C, D, E, F);
tuple_serializable!(A, B, C, D, E, F, G);
tuple_serializable!(A, B, C, D, E, F, G, H);
tuple_serializable!(A, B, C, D, E, F, G, H, I);
tuple_serializable!(A, B, C, D, E, F, G, H, I, J);
tuple_serializable!(A, B, C, D, E, F, G, H, I, J, K);
