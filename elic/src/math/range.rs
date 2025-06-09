use std::ops::Mul;


#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Range {
    pub min: f32,
    pub max: f32
}

impl Range {

    pub const fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn min_size(min: f32, size: f32) -> Self {
        Self::new(min, min + size)
    }

    pub fn max_size(max: f32, size: f32) -> Self {
        Self::new(max - size, max)
    }

    pub fn center_size(center: f32, size: f32) -> Self {
        Self::new(center - size / 2.0, center + size / 2.0)
    }

    pub fn point(pt: f32) -> Self {
        Self::min_size(pt, 0.0)
    }

    pub fn size(&self) -> f32 {
        self.max - self.min 
    }

    pub fn contains(&self, x: f32) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn intersect(&self, other: Range) -> Self {
        Self::new(
            self.min.max(other.min),
            self.max.min(other.max)
        )
    }

    pub fn intersects(&self, other: Range) -> bool {
        self.max > other.min && self.min < other.max
    }

    pub fn center(&self) -> f32 {
        (self.min + self.max) / 2.0
    }

    pub fn shift(&self, offset: f32) -> Self {
        Self {
            min: self.min + offset,
            max: self.max + offset,
        }
    }

}

impl Mul<f32> for Range {
    type Output = Range;

    fn mul(self, rhs: f32) -> Self::Output {
        Range {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }

}

impl Mul<Range> for f32 {
    type Output = Range;

    fn mul(self, rhs: Range) -> Self::Output {
        rhs * self
    }

}
