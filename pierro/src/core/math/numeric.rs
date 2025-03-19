
use std::{ops::{Add, Sub}, str::FromStr};

pub trait Numeric: Sized + Add<Self, Output = Self> + Sub<Self, Output = Self> + PartialOrd + FromStr + Copy {

    const MIN: Self;
    const MAX: Self;
    const INTEGRAL: bool;

    fn from_f64(x: f64) -> Self;
    fn to_str(&self) -> String;

}

impl Numeric for i32 {

    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    const INTEGRAL: bool = true;

    fn from_f64(x: f64) -> Self {
        x.round() as i32 
    }

    fn to_str(&self) -> String {
        self.to_string()
    }

}

impl Numeric for u32 {

    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    const INTEGRAL: bool = true;

    fn from_f64(x: f64) -> Self {
        x.round() as u32 
    }

    fn to_str(&self) -> String {
        self.to_string() 
    }

}

impl Numeric for f32 {

    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    const INTEGRAL: bool = false;

    fn from_f64(x: f64) -> Self {
        x as f32
    }

    fn to_str(&self) -> String {
        format!("{:.3}", *self)
    }

}
