
use std::collections::HashMap;

use cosmic_text::SubpixelBin;

use crate::{text::FontId, vec2, Color, Rect, Vec2};

use super::{PaintRect, Painter, Texture};

#[derive(Clone, Copy)]
pub struct TextStyle {
    pub font: FontId,
    pub color: Color,
    pub font_size: f32,
    pub line_height: f32,
}

impl Default for TextStyle {

    fn default() -> Self {
        Self {
            font: FontId::default(),
            color: Color::BLACK,
            font_size: 16.0,
            line_height: 1.0
        }
    }

}  

pub struct PaintText {
    text: String,
    style: TextStyle,
    rect: Rect
}

impl PaintText {

    pub fn new(text: String, style: TextStyle, rect: Rect) -> Self {
        Self {
            text,
            style,
            rect,
        }
    }

}

#[derive(Clone, Hash, PartialEq, Eq)]
struct TextRenderCacheKey {
    text: String,
    font_size: i64,
    line_height: i64,
    width: Option<i64>,
    height: Option<i64>
}

#[derive(Clone)]
struct TextRenderCacheGlyph {
    texture: Texture,
    pos: Vec2,
    size: Vec2,
    uv_min: Vec2,
    uv_max: Vec2,
    color: bool
}

#[derive(Clone)]
struct TextRenderLayout {
    glyphs: Vec<TextRenderCacheGlyph> 
}

pub(crate) struct TextRenderCache {
    cache: HashMap<TextRenderCacheKey, TextRenderLayout>
}

impl TextRenderCache {

    pub(crate) fn new() -> Self {
        Self {
            cache: HashMap::new()
        }
    }

}

fn size_to_bounds(size: f32) -> Option<f32> {
    if size.is_infinite() {
        None
    } else {
        Some(size)
    }
}

fn f32_to_i64_key(val: f32) -> i64 {
    (val * 1024.0).round() as i64
}

impl Painter<'_> {

    fn render_text(&mut self, layout: &TextRenderLayout, pos: Vec2, color: Color) {
        for glyph in &layout.glyphs {
            let rect = Rect::min_size(
                glyph.pos + pos,
                glyph.size 
            );

            self.rect(PaintRect::new(rect, if glyph.color { Color::white_alpha(color.a) } else { color })
                .with_texture(glyph.texture.clone())
                .with_uv(glyph.uv_min, glyph.uv_max));
        }
    }

    pub fn text(&mut self, text: PaintText) {

        let clip_rect = self.curr_clip_rect();
        if (self.curr_transform() * text.rect).intersect(clip_rect).area() < 0.01 {
            return;
        }

        let Some(font) = &mut self.text_resources.fonts.get_mut(&text.style.font) else { return; };
        let mut font_system = &mut font.font_system;
        let font_size = text.style.font_size * self.dpi_scale;
        let line_height = font_size * text.style.line_height;
        let width = size_to_bounds(text.rect.width() * self.dpi_scale);
        let height = size_to_bounds(text.rect.height() * self.dpi_scale);

        let cache_key = TextRenderCacheKey {
            text: text.text.clone(),
            font_size: f32_to_i64_key(font_size),
            line_height: f32_to_i64_key(line_height),
            width: width.map(f32_to_i64_key),
            height: height.map(f32_to_i64_key),
        };

        if let Some(layout) = self.text_render_cache.cache.remove(&cache_key) {
            self.render_text(&layout, text.rect.tl(), text.style.color);
            self.next_text_render_cache.cache.insert(cache_key, layout); 
            return;
        } else if let Some(layout) = self.next_text_render_cache.cache.get(&cache_key) {
            let layout = (*layout).clone();
            self.render_text(&layout, text.rect.tl(), text.style.color);
            return;
        }

        let mut buffer = cosmic_text::Buffer::new(&mut font_system, cosmic_text::Metrics { font_size, line_height });
        buffer.set_text(&mut font_system, &text.text, cosmic_text::Attrs::new().family(cosmic_text::Family::SansSerif), cosmic_text::Shaping::Advanced);
        buffer.set_size(font_system, width, height);
        buffer.shape_until_scroll(&mut font_system, false);

        let mut glyphs = Vec::new();

        for run in buffer.layout_runs() {
            for glyph in run.glyphs {
                let mut physical_glyph = glyph.physical((0.0, 0.0), 1.0);
                physical_glyph.cache_key.x_bin = SubpixelBin::Zero;
                physical_glyph.cache_key.y_bin = SubpixelBin::Zero;
                if let Some(glyph_info) = self.text_resources.get_glyph(text.style.font, physical_glyph.cache_key, self.device, self.queue) {
                    let pos = (vec2(physical_glyph.x as f32, physical_glyph.y as f32 + run.line_y) + glyph_info.data.pos) / self.dpi_scale;
                    let size = glyph_info.data.size / self.dpi_scale;
                    let texture = glyph_info.texture.clone();
                    let uv_min = glyph_info.data.uv_min;
                    let uv_max = glyph_info.data.uv_max;
                    let color = glyph_info.data.color;

                    glyphs.push(TextRenderCacheGlyph {
                        texture,
                        pos,
                        size,
                        uv_min,
                        uv_max,
                        color
                    });
                }
            }
        }

        let layout = TextRenderLayout { glyphs };
        self.render_text(&layout, text.rect.tl(), text.style.color);

        self.next_text_render_cache.cache.insert(cache_key, layout);

    }

}
