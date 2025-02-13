
use std::ops::Mul;

use super::{Rect, Vec2};

#[derive(Clone, Copy)]
pub struct TSTransform {
    // scale applied first around (0, 0)
    pub scale: f32,
    // translation applied after scaling
    pub translation: Vec2,
}

impl TSTransform {

    pub const IDENTITY: Self = Self::new(Vec2::ZERO, 1.0);

    pub const fn new(trans: Vec2, scale: f32) -> Self {
        Self {
            translation: trans,
            scale
        }
    }

    pub const fn translation(trans: Vec2) -> Self {
        Self::new(trans, 1.0)
    }

    pub const fn scale(scale: f32) -> Self {
        Self::new(Vec2::ZERO, scale)
    }

    pub fn inverse(&self) -> Self {
        Self::new(-self.translation / self.scale, 1.0 / self.scale)
    }

}

impl Mul<Vec2> for TSTransform {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        self.scale * rhs + self.translation
    }
}

impl Mul<Rect> for TSTransform {
    type Output = Rect;

    fn mul(self, rhs: Rect) -> Self::Output {
        Rect::min_max(self * rhs.tl(), self * rhs.br())
    }
}

impl Mul<TSTransform> for TSTransform {
    type Output = TSTransform;

    fn mul(self, rhs: TSTransform) -> Self::Output {
        Self {
            scale: self.scale * rhs.scale,
            translation: self.translation + self.scale * rhs.translation
        }
    }
}
