
mod texture;
use std::f32;

pub use texture::*;

pub struct BrushSettings {
    pub stamp_spacing: f32,
    pub base_angle: f32,
    pub angle_range: f32,
    pub shift_range: f32
}

impl BrushSettings {

    pub const fn new() -> Self {
        Self {
            stamp_spacing: 1.0,
            base_angle: 0.0,
            angle_range: 0.0,
            shift_range: 0.0
        }
    }

    pub const fn with_stamp_spacing(mut self, spacing: f32) -> Self {
        self.stamp_spacing = spacing;
        self
    }

    pub const fn with_base_angle(mut self, angle: f32) -> Self {
        self.base_angle = angle;
        self
    }

    pub const fn with_angle_range(mut self, angle: f32) -> Self {
        self.angle_range = angle;
        self
    }

    pub const fn random_angle(self) -> Self {
        self.with_angle_range(180.0)
    }

    pub const fn with_shift_range(mut self, range: f32) -> Self {
        self.shift_range = range;
        self
    }

}

impl Default for BrushSettings {

    fn default() -> Self {
        Self::new()
    }

}