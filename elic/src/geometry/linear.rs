
use crate::Vec2;

pub trait Linear: Clone {

    fn add(&self, other: Self) -> Self;
    fn scale(&self, scl: f32) -> Self; 

} 

impl Linear for f32 {

    fn add(&self, other: Self) -> Self {
        self + other
    }

    fn scale(&self, scl: f32) -> Self {
        self * scl
    }
    
}

impl Linear for Vec2 {

    fn add(&self, other: Self) -> Self {
        *self + other
    }

    fn scale(&self, scl: f32) -> Self {
        *self * scl
    }

}
