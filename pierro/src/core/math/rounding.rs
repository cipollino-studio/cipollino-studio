use std::ops::{Mul, MulAssign};


#[derive(Clone, Copy)]
pub struct Rounding {
    tl: f32,
    tr: f32,
    bl: f32,
    br: f32
}

impl Rounding {

    pub const ZERO: Self = Self::same(0.0);

    pub const fn new(tl: f32, tr: f32, bl: f32, br: f32) -> Self {
        Self {
            tl,
            tr,
            bl,
            br,
        }
    }

    pub const fn same(rounding: f32) -> Self {
        Self::new(rounding, rounding, rounding, rounding)
    }

    pub const fn top(rounding: f32) -> Self {
        Self::new(rounding, rounding, 0.0, 0.0)
    }

    pub const fn bottom(rounding: f32) -> Self {
        Self::new(0.0, 0.0, rounding, rounding)
    }

    pub const fn left(rounding: f32) -> Self {
        Self::new(rounding, 0.0, rounding, 0.0)
    }

    pub const fn right(rounding: f32) -> Self {
        Self::new(0.0, rounding, 0.0, rounding)
    }

    pub fn tl(&self) -> f32 {
        self.tl
    }

    pub fn tr(&self) -> f32 {
        self.tr
    }

    pub fn bl(&self) -> f32 {
        self.bl
    }

    pub fn br(&self) -> f32 { 
        self.br
    }

    pub fn min(&self, rounding: Self) -> Self {
        Self {
            tl: self.tl.min(rounding.tl),
            tr: self.tr.min(rounding.tr),
            bl: self.bl.min(rounding.bl),
            br: self.br.min(rounding.br)
        }
    }

}

impl Mul<f32> for Rounding {
    type Output = Rounding;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.tl * rhs, self.tr * rhs, self.bl * rhs, self.br * rhs)
    }
}

impl Mul<Rounding> for f32 {
    type Output = Rounding;

    fn mul(self, rhs: Rounding) -> Rounding {
        rhs * self
    }
}

impl MulAssign<f32> for Rounding {

    fn mul_assign(&mut self, rhs: f32) {
        self.tl *= rhs;
        self.tr *= rhs;
        self.bl *= rhs;
        self.br *= rhs;
    }

}
