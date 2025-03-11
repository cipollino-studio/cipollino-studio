
pub struct PickingBuffer {
    width: u32,
    height: u32,
    pub(crate) texture: Option<wgpu::Texture>,
    pixel_copy_buffer: Option<wgpu::Buffer>
}

impl PickingBuffer {

    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            texture: None,
            pixel_copy_buffer: None
        }
    }

    pub fn update_texture(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        let recreate_texture = self.texture.as_ref().map(|texture| texture.width() != width || texture.height() != height).unwrap_or(true);
        if recreate_texture {
            self.width = width;
            self.height = height;
            self.texture = Some(device.create_texture(&wgpu::TextureDescriptor {
                label: Some("malvina_picking_buffer_texture"),
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            }));
        }
    }

    pub fn read_pixel(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, x: u32, y: u32) -> u32 {
        let Some(texture) = self.texture.as_ref() else { return 0; };

        if self.pixel_copy_buffer.is_none() {
            self.pixel_copy_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("malvina_picking_buffer_copy_pixels_buffer"),
                size: 4,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }));
        }
        let pixel_copy_buffer = self.pixel_copy_buffer.as_ref().unwrap();
        
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("malvina_picking_buffer_copy_pixels_encoder"),
        });
        let texture_copy_source = wgpu::ImageCopyTextureBase {
            texture,
            mip_level: 0,
            origin: wgpu::Origin3d { x, y, z: 0 },
            aspect: wgpu::TextureAspect::All,
        };
        let texture_copy_dest = wgpu::ImageCopyBufferBase {
            buffer: pixel_copy_buffer,
            layout: wgpu::ImageDataLayout { offset: 0, bytes_per_row: None, rows_per_image: None },
        };
        encoder.copy_texture_to_buffer(texture_copy_source, texture_copy_dest, wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 });

        queue.submit([encoder.finish()]);

        pixel_copy_buffer.slice(..).map_async(wgpu::MapMode::Read, |_| {});

        device.poll(wgpu::MaintainBase::Wait);

        let pixel_data = pixel_copy_buffer.slice(..).get_mapped_range();
        let r = pixel_data[0];
        let g = pixel_data[1];
        let b = pixel_data[2];
        drop(pixel_data);
        pixel_copy_buffer.unmap();

        ((r as u32) << 0)  |
        ((g as u32) << 8)  |
        ((b as u32) << 16)
    }

}
