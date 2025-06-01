
use project::StrokeBrush;
use crate::{BuiltinBrushTextures, BUILTIN_BRUSHES};

pub fn get_brush_texture<'a>(brush: StrokeBrush, brushes: &'a BuiltinBrushTextures) -> &'a malvina::BrushTexture {
    match brush {
        StrokeBrush::Builtin(idx) => &brushes.textures[idx],
    }
}

pub fn get_brush_settings(brush: StrokeBrush) -> &'static malvina::BrushSettings {
    match brush {
        StrokeBrush::Builtin(idx) => &BUILTIN_BRUSHES[idx].brush,
    }
}
