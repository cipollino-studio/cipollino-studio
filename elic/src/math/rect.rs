
use core::f32;
use std::fmt::Display;

use crate::Segment;

use super::{vec2, Axis, Range, Vec2};

#[derive(Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Rect {
    min: Vec2,
    max: Vec2
}

impl Rect {

    pub const fn min_max(min: Vec2, max: Vec2) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn min_size(min: Vec2, size: Vec2) -> Self {
        Self {
            min,
            max: min + size
        }
    }

    pub fn center_size(center: Vec2, size: Vec2) -> Self {
        Self {
            min: center - size / 2.0,
            max: center + size / 2.0
        }
    }

    pub const fn from_ranges(x_range: Range, y_range: Range) -> Self {
        Self {
            min: vec2(x_range.min, y_range.min),
            max: vec2(x_range.max, y_range.max)
        }
    }

    pub const fn to_infinity(from: Vec2) -> Self {
        Self {
            min: from,
            max: Vec2::INFINITY 
        }
    }

    pub const ZERO: Self = Self::min_max(Vec2::ZERO, Vec2::ZERO);

    pub const fn tl(&self) -> Vec2 {
        self.min
    }

    pub const fn tr(&self) -> Vec2 {
        vec2(self.max.x, self.min.y)
    }

    pub const fn bl(&self) -> Vec2 {
        vec2(self.min.x, self.max.y)
    }

    pub const fn br(&self) -> Vec2 {
        self.max
    }
    
    pub const fn left(&self) -> f32 {
        self.min.x
    }

    pub const fn right(&self) -> f32 {
        self.max.x
    }

    pub const fn top(&self) -> f32 {
        self.min.y
    }

    pub const fn bottom(&self) -> f32 {
        self.max.y
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn width(&self) -> f32 {
        self.size().x
    }
    
    pub fn height(&self) -> f32 {
        self.size().y
    }

    pub const fn x_range(&self) -> Range {
        Range::new(self.min.x, self.max.x)
    }

    pub fn set_x_range(&mut self, range: Range) {
        self.min.x = range.min;
        self.max.x = range.max;
    } 

    pub const fn y_range(&self) -> Range {
        Range::new(self.min.y, self.max.y)
    }

    pub fn set_y_range(&mut self, range: Range) {
        self.min.y = range.min;
        self.max.y = range.max;
    } 

    pub const fn axis_range(&self, axis: Axis) -> Range {
        Range::new(self.min.on_axis(axis), self.max.on_axis(axis))
    }

    pub fn set_axis_range(&mut self, axis: Axis, range: Range) {
        *self.min.on_axis_mut(axis) = range.min; 
        *self.max.on_axis_mut(axis) = range.max; 
    }

    pub fn contains(&self, pt: Vec2) -> bool {
        self.x_range().contains(pt.x) && self.y_range().contains(pt.y)
    }

    pub fn intersect(&self, other: Rect) -> Rect {
        Self::from_ranges(
            self.x_range().intersect(other.x_range()),
            self.y_range().intersect(other.y_range()) 
        )
    }

    pub fn intersects(&self, other: Rect) -> bool {
        self.x_range().intersects(other.x_range()) && self.y_range().intersects(other.y_range())
    }

    pub fn shift(&self, offset: Vec2) -> Rect {
        Self::min_max(self.min + offset, self.max + offset)
    }

    pub fn map(&self, from: Rect, to: Rect) -> Self {
        Self::min_max(
            self.min.map(from, to),
           self.max.map(from, to) 
        )
    }

    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }

    pub fn left_frac(&self, frac: f32) -> Rect {
        Self::min_max(self.tl(), self.bl().lerp(self.br(), frac))
    }
    
    pub fn right_frac(&self, frac: f32) -> Rect {
        Self::min_max(self.tl().lerp(self.tr(), frac), self.br())
    }

    pub fn top_frac(&self, frac: f32) -> Rect {
        Self::min_max(self.tl(), self.tr().lerp(self.br(), frac))
    }

    pub fn bottom_frac(&self, frac: f32) -> Rect {
        Self::min_max(self.bl().lerp(self.tl(), frac), self.br())
    }

    pub fn left_half(&self) -> Rect {
        self.left_frac(0.5)
    }

    pub fn right_half(&self) -> Rect {
        self.right_frac(0.5)
    }

    pub fn top_half(&self) -> Rect {
        self.top_frac(0.5)
    }

    pub fn bottom_half(&self) -> Rect {
        self.bottom_frac(0.5)
    }

    pub fn merge(&self, other: Rect) -> Rect {
        Self::min_max(self.min.min(other.min), self.max.max(other.max))
    }

    pub fn bounds_all<I: Iterator<Item = Vec2>>(pts: I) -> Self {
        let mut min = Vec2::INFINITY;
        let mut max = -Vec2::INFINITY;
        for pt in pts {
            min = min.min(pt);
            max = max.max(pt);
        }
        Self::min_max(min, max)
    }

    pub fn left_side(&self) -> Segment {
        Segment::new(self.tl(), self.bl())
    }

    pub fn top_side(&self) -> Segment {
        Segment::new(self.tl(), self.tr())
    }

    pub fn right_side(&self) -> Segment {
        Segment::new(self.tr(), self.br())
    }

    pub fn bottom_side(&self) -> Segment {
        Segment::new(self.bl(), self.br())
    }

    pub fn h_split_line(&self) -> Segment {
        Segment::new(
            0.5 * (self.tl() + self.bl()),
            0.5 * (self.tr() + self.br())
        )
    }
    
    pub fn v_split_line(&self) -> Segment {
        Segment::new(
            0.5 * (self.tl() + self.tr()),
            0.5 * (self.bl() + self.br())
        )
    }

}

impl Display for Rect {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tl().fmt(f)?;
        f.write_str(" -> ")?;
        self.br().fmt(f)
    }

}
