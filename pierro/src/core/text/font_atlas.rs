
use std::collections::HashMap;
use cosmic_text::{CacheKey, FontSystem, SwashCache, SwashContent};
use etagere::{BucketedAtlasAllocator, size2};

use crate::{vec2, Texture, Vec2};

struct AtlasTexture {
    texture: Texture,
    packer: BucketedAtlasAllocator,
    size: u32
}

impl AtlasTexture {

    const SIZE: u32 = 1024;

    pub fn new(device: &wgpu::Device) -> Self {

        let size = Self::SIZE.min(device.limits().max_texture_dimension_2d);

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("pierro_font_atlas_texture"),
            size: wgpu::Extent3d { width: size, height: size, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let packer = BucketedAtlasAllocator::new(size2(size as i32, size as i32));

        let texture = Texture::new(texture, texture_view);

        Self {
            texture,
            packer,
            size
        }
    }

}

#[derive(Clone, Copy)]
pub(crate) struct GlyphData {
    pub(crate) pos: Vec2,
    pub(crate) size: Vec2,
    pub(crate) uv_min: Vec2,
    pub(crate) uv_max: Vec2,
    pub(crate) color: bool
}

struct GlyphLookup {
    texture_id: usize,
    data: GlyphData
}

pub(crate) struct FontAtlas {
    textures: Vec<AtlasTexture>,
    glyph_lookup: HashMap<CacheKey, GlyphLookup>
}

pub(crate) struct Glyph<'a> {
    pub(crate) texture: &'a Texture,
    pub(crate) data: GlyphData
}

impl FontAtlas {

    pub(crate) fn new(device: &wgpu::Device) -> Self {
        
        Self {
            textures: vec![AtlasTexture::new(device)],
            glyph_lookup: HashMap::new() 
        }
    }

    pub(crate) fn get_glyph(
        &mut self,
        glyph: CacheKey,

        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,

        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Option<Glyph> {
        if let Some(lookup) = self.glyph_lookup.get(&glyph) {
            return Some(Glyph {
                texture: &self.textures[lookup.texture_id].texture,
                data: lookup.data
            });
        }

        let image = swash_cache.get_image_uncached(font_system, glyph)?;
        let x = image.placement.left;
        let y = image.placement.top;
        let w = image.placement.width;
        let h = image.placement.height;
        let (color, data) = match image.content {
            SwashContent::Mask => {
                let mut data = Vec::new();
                data.reserve_exact((w * h * 4) as usize);
                for val in image.data {
                    data.push(255); // r
                    data.push(255); // g
                    data.push(255); // b
                    data.push(val); // a
                }
                (false, data)
            },
            SwashContent::Color => (true, image.data),
            SwashContent::SubpixelMask => panic!("subpixel text antialiasing not be supported."),
        };

        let alloc = if let Some(alloc) = self.textures.last_mut()?.packer.allocate(size2(w as i32, h as i32)) {
            alloc
        } else {
            let mut new_texture = AtlasTexture::new(device);
            // if we can't allocate the glyph in a new texture, we might as well give up.
            let alloc = new_texture.packer.allocate(size2(w as i32, h as i32))?;
            self.textures.push(new_texture);
            alloc
        };

        let alloc = alloc.rectangle;

        let texture = self.textures.last()?;
        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture.texture.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d { x: alloc.min.x as u32, y: alloc.min.y as u32, z: 0 },
                aspect: wgpu::TextureAspect::All 
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * w),
                rows_per_image: None
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1 
            } 
        );

        let pos = vec2(x as f32, -y as f32);
        let size = vec2(w as f32, h as f32);
        let uv_min = vec2(alloc.min.x as f32, alloc.min.y as f32) / Vec2::splat(texture.size as f32);
        let uv_max = vec2((alloc.min.x as u32 + w) as f32, (alloc.min.y as u32 + h) as f32) / Vec2::splat(texture.size as f32);

        let data = GlyphData {
            pos,
            size,
            uv_min,
            uv_max,
            color,
        };

        self.glyph_lookup.insert(glyph, GlyphLookup {
            texture_id: self.textures.len() - 1,
            data
        });

        Some(Glyph {
            texture: &texture.texture,
            data
        }) 
    }

}
