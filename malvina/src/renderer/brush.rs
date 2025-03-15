
pub struct BrushTextureResources {
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler
}

impl BrushTextureResources {

    pub fn new(device: &wgpu::Device) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("malvina_brush_texture_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false 
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                }
            ] 
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("malvina_texture_brush_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            bind_group_layout,
            sampler,
        }
    }

}

pub struct BrushTexture {
    #[allow(unused)]
    texture: wgpu::Texture,
    pub(crate) bind_group: wgpu::BindGroup
}

impl BrushTexture {

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, resources: &BrushTextureResources, width: u32, height: u32, data: &[u8]) -> Self {
        assert_eq!((width * height) as usize, data.len());

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("malvina_brush_texture"),
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTextureBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All 
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width),
                rows_per_image: Some(height)
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 }
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("malvina_brush_texture_bind_group"),
            layout: &resources.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&resources.sampler),
                }
            ] 
        });

        Self {
            texture,
            bind_group
        }
    }

    pub fn circle(device: &wgpu::Device, queue: &wgpu::Queue, resources: &BrushTextureResources, size: u32) -> Self {
        let mut data = Vec::new(); 

        for y in 0..(size as i32) {
            for x in 0..(size as i32) {
                let r = (size as i32) / 2;
                let dist = (x - r) * (x - r) + (y - r) * (y - r);
                let dist = (dist as f32).sqrt();
                let val = 1.0 - (dist - r as f32).clamp(0.0, 1.0);
                data.push((val * 255.0).floor() as u8);
            }
        }

        Self::new(device, queue, resources, size, size, &data)
    }

}
