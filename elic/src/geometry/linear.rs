
use crate::Vec2;

pub trait Linear: Clone {

    fn add(&self, other: Self) -> Self;
    fn scale(&self, scl: f32) -> Self; 

    fn lerp(a: Self, b: Self, t: f32) -> Self {
        a.scale(1.0 - t).add(b.scale(t))
    }

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
