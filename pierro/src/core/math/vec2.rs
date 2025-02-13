use core::f32;
use std::{fmt::Display, ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign}};

use super::{map, Axis, Rect};

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

impl Vec2 {

    pub const X: Self = vec2(1.0, 0.0);
    pub const Y: Self = vec2(0.0, 1.0);
    pub const NEG_X: Self = vec2(-1.0, 0.0);
    pub const NEG_Y: Self = vec2(0.0, -1.0);

    pub const AXES: [Self; 2] = [Self::X, Self::Y];
    pub const NEG_AXES: [Self; 2] = [Self::NEG_X, Self::NEG_Y];

    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const INFINITY: Self = Self::splat(f32::INFINITY);

    pub const fn splat(val: f32) -> Self {
        vec2(val, val)
    }

    pub const fn on_axis(&self, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }
    
    pub fn on_axis_mut(&mut self, axis: Axis) -> &mut f32 {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn distance(&self, other: Vec2) -> f32 {
        (*self - other).length()
    }

    pub fn map(&self, from: Rect, to: Rect) -> Self {
        vec2(
            map(self.x,from.x_range(), to.x_range()),
            map(self.y,from.y_range(), to.y_range()),
        )
    }

    pub fn min(&self, other: Vec2) -> Self {
        vec2(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(&self, other: Vec2) -> Self {
        vec2(self.x.max(other.x), self.y.max(other.y))
    }

    pub fn abs(&self) -> Self {
        vec2(self.x.abs(), self.y.abs())
    }

    pub fn min_component(&self) -> f32 {
        self.x.min(self.y)
    }

    pub fn max_component(&self) -> f32 {
        self.x.max(self.y)
    }

    pub fn min_axis(&self) -> Axis {
        if self.x < self.y {
            Axis::X
        } else {
            Axis::Y
        }
    }

    pub fn max_axis(&self) -> Axis {
        self.min_axis().other()
    }

}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        vec2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Vec2> for Vec2 {

    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        vec2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Vec2> for Vec2 {

    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }

}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        vec2(-self.x, -self.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        vec2(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        vec2(rhs.x * self, rhs.y * self)
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        vec2(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        vec2(self.x / rhs, self.y / rhs)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        vec2(self.x / rhs.x, self.y / rhs.y)
    }
}

impl From<Vec2> for [f32; 2] {

    fn from(v: Vec2) -> Self {
        [v.x, v.y]
    }
    
}

impl Display for Vec2 {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)?;
        f.write_str("]")?;
        Ok(())
    }

}

impl Default for Vec2 {

    fn default() -> Self {
        Self::ZERO
    }

}
