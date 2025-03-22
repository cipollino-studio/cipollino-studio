  
use core::f32;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

pub const fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4 { x, y, z, w }
}

impl Vec4 {

    pub const X: Self = vec4(1.0, 0.0, 0.0, 0.0);
    pub const Y: Self = vec4(0.0, 1.0, 0.0, 0.0);
    pub const Z: Self = vec4(0.0, 0.0, 1.0, 0.0);
    pub const W: Self = vec4(0.0, 0.0, 0.0, 1.0);
    
    pub const NEG_X: Self = vec4(-1.0,  0.0,  0.0,  0.0);
    pub const NEG_Y: Self = vec4( 0.0, -1.0,  0.0,  0.0);
    pub const NEG_Z: Self = vec4( 0.0,  0.0, -1.0,  0.0);
    pub const NEG_W: Self = vec4( 0.0,  0.0,  0.0, -1.0);

    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];
    pub const NEG_AXES: [Self; 4] = [Self::NEG_X, Self::NEG_Y, Self::NEG_Z, Self::NEG_W];

    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const INFINITY: Self = Self::splat(f32::INFINITY);

    pub const fn splat(val: f32) -> Self {
        vec4(val, val, val, val)
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Vec4 {
        *self / self.length()
    }

    pub fn distance(&self, other: Vec4) -> f32 {
        (*self - other).length()
    }

    pub fn min(&self, other: Vec4) -> Self {
        vec4(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w)
        )
    }

    pub fn max(&self, other: Vec4) -> Self {
        vec4(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w)
        )
    }

    pub fn abs(&self) -> Self {
        vec4(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    pub fn min_component(&self) -> f32 {
        self.x.min(self.y).min(self.z).min(self.w)
    }

    pub fn max_component(&self) -> f32 {
        self.x.max(self.y).max(self.z).max(self.w)
    }

    pub fn lerp(&self, other: Vec4, t: f32) -> Vec4 {
        *self * (1.0 - t) + other * t
    }

}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        vec4(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl AddAssign<Vec4> for Vec4 {

    fn add_assign(&mut self, rhs: Vec4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }

}

impl Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        vec4(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
    }
}

impl SubAssign<Vec4> for Vec4 {

    fn sub_assign(&mut self, rhs: Vec4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }

}

impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        vec4(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        vec4(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl MulAssign<f32> for Vec4 {

    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
    
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        vec4(rhs.x * self, rhs.y * self, rhs.z * self, rhs.w * self)
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        vec4(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Self::Output {
        vec4(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl DivAssign<f32> for Vec4 {

    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }

}

impl Div<Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Self::Output {
        vec4(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
    }
}

impl From<Vec4> for [f32; 4] {

    fn from(v: Vec4) -> Self {
        [v.x, v.y, v.z, v.w]
    }
    
}

impl std::fmt::Display for Vec4 {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)?;
        f.write_str(", ")?;
        self.z.fmt(f)?;
        f.write_str(", ")?;
        self.w.fmt(f)?;
        f.write_str("]")?;
        Ok(())
    }

}

impl std::fmt::Debug for Vec4 {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }

}

impl Default for Vec4 {

    fn default() -> Self {
        Self::ZERO
    }

}
