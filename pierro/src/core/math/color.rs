
use std::fmt::Display;

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b,
            a: 1.0,
        }
    }
    
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r,
            g,
            b,
            a
        }
    }

    pub fn hex(hex: u32) -> Self {
        let bytes = hex.to_be_bytes();
        Self {
            r: bytes[0] as f32 / 255.0,
            g: bytes[1] as f32 / 255.0,
            b: bytes[2] as f32 / 255.0,
            a: bytes[3] as f32 / 255.0,
        }
    }

    pub const TRANSPARENT: Self = Self::rgba(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const PURPLE: Self = Self::rgb(0.5, 0.0, 1.0);

    pub const fn white_alpha(a: f32) -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a,
        }
    }

    pub fn darken(&self, t: f32) -> Self {
        Self::rgba(self.r * (1.0 - t), self.g * (1.0 - t), self.b * (1.0 - t), self.a)
    }

    pub fn with_alpha(&self, alpha: f32) -> Self {
        Color::rgba(self.r, self.g, self.b, alpha)
    }

}

impl From<Color> for [f32; 4] {

    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
    
}

impl Default for Color {

    fn default() -> Self {
        Self::BLACK
    }

}

impl Display for Color {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        self.r.fmt(f)?;
        f.write_str(", ")?;
        self.g.fmt(f)?;
        f.write_str(", ")?;
        self.b.fmt(f)?;
        f.write_str("]")?;
        Ok(())
    }

}
