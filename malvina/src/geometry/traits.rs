
use std::ops::{Add, Mul};

pub trait Linear: Add<Self, Output = Self> + Mul<f32, Output = Self> + Sized + Clone {}

impl Linear for f32 {}
impl Linear for glam::Vec2 {}
impl Linear for glam::Vec3 {}
impl Linear for glam::Vec4 {}

pub trait HasMagnitude: Linear {

    fn magnitude(&self) -> f32;

    fn distance(self, other: Self) -> f32 {
        (self + other * -1.0).magnitude()
    }

}

impl HasMagnitude for glam::Vec2 {

    fn magnitude(&self) -> f32 {
        self.length()
    }

}

impl HasMagnitude for glam::Vec3 {

    fn magnitude(&self) -> f32 {
        self.length()
    }

}

impl HasMagnitude for glam::Vec4 {

    fn magnitude(&self) -> f32 {
        self.length()
    }

}

pub trait Turnable: Linear {

    fn turn_cw(&self) -> Self;

    fn turn_ccw(&self) -> Self {
        self.turn_cw() * -1.0
    }

}

impl Turnable for glam::Vec2 {

    fn turn_cw(&self) -> Self {
        glam::vec2(self.y, -self.x)
    }
    
}
