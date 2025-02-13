
mod font_atlas;

use std::collections::HashMap;

use cosmic_text::{fontdb, CacheKey, FontSystem, SwashCache};
use font_atlas::{FontAtlas, Glyph};

pub(crate) struct Font {
    pub(crate) font_system: FontSystem,
}

impl Font {

    fn system_fonts() -> Self {
        Self {
            font_system: FontSystem::new()
        }
    }

    fn icons() -> Self {
        let mut icon_font_db = fontdb::Database::new(); 
        icon_font_db.load_font_data(include_bytes!("../../../res/icons/Phosphor.ttf").to_vec());
        Self {
            font_system: FontSystem::new_with_locale_and_db("en_US".to_owned(), icon_font_db)
        }
    }

}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontId(usize);

impl Default for FontId {

    fn default() -> Self {
        Self(0)
    }

}

pub(crate) struct TextResources {
    pub(crate) fonts: HashMap<FontId, Font>,
    pub(crate) swash_cache: SwashCache,
    pub(crate) text_font: FontId,
    pub(crate) icon_font: FontId,
    atlas: FontAtlas
}

impl TextResources {

    pub(crate) fn new(device: &wgpu::Device) -> Self {

        let text_font = FontId(0); 
        let icon_font = FontId(1);

        let mut fonts = HashMap::new();
        fonts.insert(text_font, Font::system_fonts());
        fonts.insert(icon_font, Font::icons());
        Self {
            fonts,
            swash_cache: SwashCache::new(),
            text_font,
            icon_font,
            atlas: FontAtlas::new(device),
        } 
    }

    pub(crate) fn get_glyph(&mut self, font_id: FontId, glyph: CacheKey, device: &wgpu::Device, queue: &wgpu::Queue) -> Option<Glyph> {
        let font = self.fonts.get_mut(&font_id)?;
        self.atlas.get_glyph(glyph, &mut font.font_system, &mut self.swash_cache, device, queue)
    }

}
