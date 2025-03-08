
use std::hash::Hash;

use crate::{hash, Axis, Color, Margin, Painter, PerAxis, Rect, Rounding, Stroke, TSTransform, TextStyle, Texture};

use super::{Layout, Size};

bitflags::bitflags! {

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct Sense: u8 {
        const MOUSE = 1 << 0;
        const SCROLL = 1 << 1;
        const DND_HOVER = 1 << 2;
        const KEYBOARD = 1 << 3;

        const INTERACTION_PRIORITY = 1 << 7;
    }

}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextureMapMode {
    /// Scale the texture to the needed size, without preserving aspect ratio
    Scale,
    /// Scale the texture to the needed size, preserving the aspect ratio
    Fit,
    /// Don't scale the texture down, and always preserve the aspect ratio
    Cover
}

pub struct UINodeParams {
    // Layout
    pub(crate) size: PerAxis<Size>,
    pub(crate) layout: Layout,
    pub(crate) margin: Margin,
    pub(crate) interaction_margin: Margin,
    pub(crate) transform: TSTransform,

    // Styling
    pub(crate) fill: Color,
    pub(crate) rounding: Rounding,
    pub(crate) stroke: Stroke,
    pub(crate) clip: bool,
    pub(crate) texture: Option<Texture>,
    pub(crate) texture_map: TextureMapMode,

    // Text
    pub(crate) text: Option<String>,
    pub(crate) text_style: TextStyle,

    // Id
    pub(crate) id_source: Option<u64>,

    pub(crate) sense: Sense,

    // Custom Behaviour 
    pub(crate) on_paint: Option<Box<dyn FnOnce(&mut Painter, Rect)>>
}

impl UINodeParams {

    pub fn new(w: Size, h: Size) -> Self {
        Self {
            size: PerAxis::new(w, h),
            layout: Layout::new(Axis::Y),
            margin: Margin::ZERO,
            interaction_margin: Margin::ZERO,
            transform: TSTransform::IDENTITY,
            fill: Color::TRANSPARENT,
            rounding: Rounding::ZERO,
            stroke: Stroke::NONE,
            clip: true,
            text: None,
            text_style: TextStyle::default(),
            texture: None,
            texture_map: TextureMapMode::Fit,
            id_source: None,
            sense: Sense::empty(),
            on_paint: None
        }
    }
    
    pub fn new_per_axis(size: PerAxis<Size>) -> Self {
        Self::new(size.x, size.y)
    }

    pub fn with_size(mut self, w: Size, h: Size) -> Self {
        self.size.x = w;
        self.size.y = h;
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn with_margin(mut self, margin: Margin) -> Self {
        self.margin = margin;
        self
    }

    pub fn with_interaction_margin(mut self, margin: Margin) -> Self {
        self.interaction_margin = margin;
        self
    }

    pub fn with_transform(mut self, transform: TSTransform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_fill(mut self, color: Color) -> Self {
        self.fill = color;
        self
    }

    pub fn with_rounding(mut self, rounding: Rounding) -> Self {
        self.rounding = rounding;
        self
    }

    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = stroke;
        self
    }

    pub fn with_texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn with_texture_map(mut self, texture_map: TextureMapMode) -> Self {
        self.texture_map = texture_map;
        self
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    pub fn no_clip(self) -> Self {
        self.with_clip(false)
    }

    pub fn with_text<S: Into<String>>(mut self, text: S) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_text_style(mut self, style: TextStyle) -> Self {
        self.text_style = style;
        self
    }

    pub fn with_id<H: Hash>(mut self, source: &H) -> Self {
        self.id_source = Some(hash(source));
        self
    }

    pub fn sense_mouse(mut self) -> Self {
        self.sense |= Sense::MOUSE;
        self
    }
    
    pub fn sense_scroll(mut self) -> Self {
        self.sense |= Sense::SCROLL;
        self
    }

    pub fn sense_dnd_hover(mut self) -> Self {
        self.sense |= Sense::DND_HOVER;
        self
    }

    pub fn sense_keyboard(mut self) -> Self {
        self.sense |= Sense::KEYBOARD;
        self
    }

    pub fn with_interaction_priority(mut self) -> Self {
        self.sense |= Sense::INTERACTION_PRIORITY;
        self
    }

    pub fn on_paint<F: FnOnce(&mut Painter, Rect) + 'static>(mut self, on_paint: F) -> Self {
        self.on_paint = Some(Box::new(on_paint));
        self
    }

}
