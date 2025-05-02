
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

use super::{vec2, Axis, Vec2};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IVec2 {
    pub x: i32,
    pub y: i32
}

pub const fn ivec2(x: i32, y: i32) -> IVec2 {
    IVec2 { x, y }
}

impl IVec2 {

    pub const X: Self = ivec2(1, 0);
    pub const Y: Self = ivec2(0, 1);
    pub const NEG_X: Self = ivec2(-1, 0);
    pub const NEG_Y: Self = ivec2(0, -1);

    pub const AXES: [Self; 2] = [Self::X, Self::Y];
    pub const NEG_AXES: [Self; 2] = [Self::NEG_X, Self::NEG_Y];
    pub const CARDINAL_DIRECTIONS: [Self; 4] = [Self::X, Self::NEG_Y, Self::NEG_X, Self::Y];

    pub const ZERO: Self = Self::splat(0);
    pub const ONE: Self = Self::splat(1);
    pub const NEG_ONE: Self = Self::splat(-1);
    pub const MAX: Self = Self::splat(i32::MAX);
    pub const MIN: Self = Self::splat(i32::MIN);

    pub const fn splat(val: i32) -> Self {
        ivec2(val, val)
    }

    pub const fn per_axis(axis: Axis, along: i32, across: i32) -> Self {
        match axis {
            Axis::X => ivec2(along, across),
            Axis::Y => ivec2(across, along),
        }
    } 

    pub const fn on_axis(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }
    
    pub fn on_axis_mut(&mut self, axis: Axis) -> &mut i32 {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
        }
    }

    pub fn min(&self, other: IVec2) -> Self {
        ivec2(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(&self, other: IVec2) -> Self {
        ivec2(self.x.max(other.x), self.y.max(other.y))
    }

    pub fn abs(&self) -> Self {
        ivec2(self.x.abs(), self.y.abs())
    }

    pub fn min_component(&self) -> i32 {
        self.x.min(self.y)
    }

    pub fn max_component(&self) -> i32 {
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

    pub fn turn_cw(&self) -> IVec2 {
        ivec2(self.y, -self.x)
    }

    pub fn turn_ccw(&self) -> IVec2 {
        ivec2(-self.y, self.x)
    }

    pub fn div_euclid(&self, rhs: IVec2) -> IVec2 {
        ivec2(self.x.div_euclid(rhs.x), self.y.div_euclid(rhs.y))
    }

    pub fn to_vec(&self) -> Vec2 {
        (*self).into()
    }

}

impl Add<IVec2> for IVec2 {
    type Output = IVec2;

    fn add(self, rhs: IVec2) -> Self::Output {
        ivec2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<IVec2> for IVec2 {

    fn add_assign(&mut self, rhs: IVec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

}

impl Sub<IVec2> for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: IVec2) -> Self::Output {
        ivec2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<IVec2> for IVec2 {

    fn sub_assign(&mut self, rhs: IVec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }

}

impl Neg for IVec2 {
    type Output = IVec2;

    fn neg(self) -> Self::Output {
        ivec2(-self.x, -self.y)
    }
}

impl Mul<i32> for IVec2 {
    type Output = IVec2;

    fn mul(self, rhs: i32) -> Self::Output {
        ivec2(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<i32> for IVec2 {

    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
    
}

impl Mul<IVec2> for i32 {
    type Output = IVec2;

    fn mul(self, rhs: IVec2) -> Self::Output {
        ivec2(rhs.x * self, rhs.y * self)
    }
}

impl Mul<IVec2> for IVec2 {
    type Output = IVec2;

    fn mul(self, rhs: IVec2) -> Self::Output {
        ivec2(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign<IVec2> for IVec2 {

    fn mul_assign(&mut self, rhs: IVec2) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }

}

impl Div<i32> for IVec2 {
    type Output = IVec2;

    fn div(self, rhs: i32) -> Self::Output {
        ivec2(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<i32> for IVec2 {

    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }

}

impl Div<IVec2> for IVec2 {
    type Output = IVec2;

    fn div(self, rhs: IVec2) -> Self::Output {
        ivec2(self.x / rhs.x, self.y / rhs.y)
    }
}

impl From<IVec2> for [i32; 2] {

    fn from(v: IVec2) -> Self {
        [v.x, v.y]
    }
    
}

impl Rem<i32> for IVec2 {
    type Output = IVec2;

    fn rem(self, rhs: i32) -> Self::Output {
        ivec2(self.x % rhs, self.y % rhs)
    }
}

impl RemAssign<i32> for IVec2 {

    fn rem_assign(&mut self, rhs: i32) {
        self.x %= rhs;
        self.y %= rhs;
    }

}

impl std::fmt::Display for IVec2 {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        self.x.fmt(f)?;
        f.write_str(", ")?;
        self.y.fmt(f)?;
        f.write_str("]")?;
        Ok(())
    }

}

impl std::fmt::Debug for IVec2 {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as std::fmt::Display>::fmt(self, f)
    }

}

impl Default for IVec2 {

    fn default() -> Self {
        Self::ZERO
    }

}

impl From<IVec2> for Vec2 {

    fn from(vec: IVec2) -> Self {
        vec2(vec.x as f32, vec.y as f32)
    }

}
