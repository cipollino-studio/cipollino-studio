
use std::sync::Arc;

#[derive(Clone)]
pub struct Texture {
    tex: Arc<(wgpu::Texture, wgpu::TextureView)> 
}

impl PartialEq for Texture {

    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.tex, &other.tex)
    }

}

impl Eq for Texture {}

impl Texture {

    pub fn new(texture: wgpu::Texture, texture_view: wgpu::TextureView) -> Self {
        Self {
            tex: Arc::new((texture, texture_view)),
        }
    }

    pub fn texture(&self) -> &wgpu::Texture {
        &self.tex.0
    }
    
    pub fn texture_view(&self) -> &wgpu::TextureView {
        &self.tex.1
    }

    pub fn create_empty(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        
        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                label: None,
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            }
        );

        let texture_view = texture.create_view(&Default::default());
        
        Self::new(texture, texture_view)
    }

    pub fn create(device: &wgpu::Device, queue: &wgpu::Queue, width: u32, height: u32, data: &[u8]) -> Self {
        assert_eq!((width * height * 4) as usize, data.len());

        let texture = Self::create_empty(device, width, height);

        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture.texture(),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height) 
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 } 
        );

        texture
    }

    pub fn width(&self) -> u32 {
        self.texture().width()
    }

    pub fn height(&self) -> u32 {
        self.texture().height()
    }

}
