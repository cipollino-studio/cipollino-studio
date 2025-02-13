
use crate::Color;

#[derive(Clone, Copy)]
pub struct Stroke {
    pub color: Color,
    pub width: f32
}

impl Stroke {

    pub fn new(color: Color, width: f32) -> Self {
        Self {
            color,
            width,
        }
    }

    pub const NONE: Self = Self {
        color: Color::TRANSPARENT,
        width: 0.0,
    };

}
