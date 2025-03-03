
use wgpu::util::DeviceExt;

use crate::{Color, Rect, Rounding, Vec2};

use super::{Painter, Stroke, Texture};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct Uniforms {
    screen_size: [f32; 2]
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct RectData {
    min: [f32; 2],
    size: [f32; 2],
    uv_min: [f32; 2],
    uv_size: [f32; 2],
    color: [f32; 4],
    tex_idx: u32,
    clip_min: [f32; 2],
    clip_max: [f32; 2],
    rounding: [f32; 4],
    stroke_color: [f32; 4],
    stroke_width: f32
}

impl RectData { 

    const ATTRIBS: &[wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
        0 => Float32x2,
        1 => Float32x2,
        2 => Float32x2,
        3 => Float32x2,
        4 => Float32x4,
        5 => Uint32,
        6 => Float32x2,
        7 => Float32x2,
        8 => Float32x4,
        9 => Float32x4,
        10 => Float32
    ];
    
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: Self::ATTRIBS 
        }
    }

}

pub(super) struct RectResources {
    pipeline: wgpu::RenderPipeline,
    buffers: Vec<wgpu::Buffer>,
    curr_buffer: usize,
    bind_group_layout: wgpu::BindGroupLayout,
    uniform_buffer: wgpu::Buffer,

    rect_batch: Vec<RectData>,
    textures: Vec<Texture>,

    // needed in case no textures are used in the current rect batch to put *something* in the bind group
    filler_texture: Texture
}

const MAX_RECTS_IN_BATCH: usize = 2048; 
const MAX_TEXTURES_IN_BATCH: usize = 8;

impl RectResources {

    pub(crate) fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {

        let shader = device.create_shader_module(wgpu::include_wgsl!("rect.wgsl"));

        let mut layout_entries = vec![
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None 
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None 
                },
                count: None
            }
        ];
        for i in 0..(MAX_TEXTURES_IN_BATCH as u32) {
            layout_entries.push(wgpu::BindGroupLayoutEntry {
                binding: i + 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false
                },
                count: None 
            });
        }
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("pierro_rect_bind_group_layout"),
            entries: &layout_entries
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pierro_rect_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[] 
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pierro_rect_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[RectData::desc()], 
                compilation_options: wgpu::PipelineCompilationOptions::default(), 
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false 
            },
            multiview: None,
            cache: None
        });

        let mut rect_batch = Vec::new();
        rect_batch.reserve_exact(MAX_RECTS_IN_BATCH);

        let mut textures = Vec::new();
        textures.reserve_exact(MAX_TEXTURES_IN_BATCH);

        let filler_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("pierro_rect_filler_texture"),
            size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let filler_texture_view = filler_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let filler_texture = Texture::new(filler_texture, filler_texture_view);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("pierro_rect_uniform_buffer"),
            contents: bytemuck::cast_slice(&[Uniforms {
                screen_size: [600.0, 400.0],
            }]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            pipeline,
            buffers: Vec::new(),
            curr_buffer: 0,
            bind_group_layout,
            uniform_buffer,

            rect_batch,
            textures,

            filler_texture
        }
    }

    fn get_texture_idx(&mut self, texture: Texture, device: &wgpu::Device, queue: &wgpu::Queue, render_pass: &mut wgpu::RenderPass) -> u32 {
        for i in 0..self.textures.len() {
            if self.textures[i] == texture {
                return i as u32;
            }
        }

        if self.textures.len() == MAX_TEXTURES_IN_BATCH {
            self.flush_buffer(device, queue, render_pass);
        }
        self.textures.push(texture);

        (self.textures.len() - 1) as u32
    }

    pub(super) fn begin_frame(&mut self, queue: &wgpu::Queue, logical_size: Vec2) {
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[Uniforms {
            screen_size: [logical_size.x, logical_size.y],
        }]));
    }

    fn push_rect(&mut self, rect: PaintRect, clip_rect: Rect, device: &wgpu::Device, queue: &wgpu::Queue, render_pass: &mut wgpu::RenderPass) {
        let rounding = rect.rounding;
        let data = RectData {
            min: rect.rect.tl().into(),
            size: rect.rect.size().into(),
            uv_min: rect.uv_min.into(),
            uv_size: (rect.uv_max - rect.uv_min).into(),
            color: rect.fill.into(),
            tex_idx: rect.texture.map(|tex| self.get_texture_idx(tex, device, queue, render_pass) + 1).unwrap_or(0),
            clip_min: clip_rect.tl().into(), 
            clip_max: clip_rect.br().into(), 
            rounding: [rounding.tl(), rounding.tr(), rounding.bl(), rounding.br()],
            stroke_color: rect.stroke.color.into(),
            stroke_width: rect.stroke.width
        };
        if self.rect_batch.len() == MAX_RECTS_IN_BATCH - 1 {
            self.flush_buffer(device, queue, render_pass);
        }
        self.rect_batch.push(data);
    }

    fn create_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("pierro_rect_batch_buffer"),
            size: (MAX_RECTS_IN_BATCH * std::mem::size_of::<RectData>()) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        })
    }

    pub(super) fn flush_buffer(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, render_pass: &mut wgpu::RenderPass) {

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("pierro_rect_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // set up the texture bindings
        let mut bind_group_entries = vec![
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&sampler) 
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &self.uniform_buffer,
                    offset: 0,
                    size: None
                }) 
            }
        ];
        if self.textures.is_empty() { // make sure there is *some* texture to bind
            self.textures.push(self.filler_texture.clone());
        }
        for i in 0..MAX_TEXTURES_IN_BATCH {
            let texture = &self.textures[i % self.textures.len()];
            bind_group_entries.push(wgpu::BindGroupEntry {
                binding: (i + 2) as u32, 
                resource: wgpu::BindingResource::TextureView(texture.texture_view()),
            });
        }

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("pierro_rect_bind_group"),
            layout: &self.bind_group_layout,
            entries: &bind_group_entries
        });

        if self.curr_buffer == self.buffers.len() {
            self.buffers.push(Self::create_buffer(device));
        }
        let buffer = &self.buffers[self.curr_buffer];
        let rect_data = bytemuck::cast_slice(self.rect_batch.as_slice());
        queue.write_buffer(buffer, 0, rect_data);
        queue.submit([]);
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, buffer.slice(..));
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..6, 0..(self.rect_batch.len() as u32));

        self.curr_buffer += 1;

        self.textures.clear();
        self.rect_batch.clear();
    }

    pub(super) fn finish(&mut self) {
        self.curr_buffer = 0;
    }

}

pub struct PaintRect {
    rect: Rect,
    fill: Color,
    texture: Option<Texture>,
    uv_min: Vec2,
    uv_max: Vec2,
    rounding: Rounding,
    stroke: Stroke
}

impl PaintRect {

    pub fn new(rect: Rect, fill: Color) -> Self {
        Self {
            rect, 
            fill,
            texture: None,
            uv_min: Vec2::ZERO,
            uv_max: Vec2::ONE,
            rounding: Rounding::ZERO,
            stroke: Stroke::NONE
        }
    }
    
    pub fn with_texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    } 

    pub fn with_uv(mut self, min: Vec2, max: Vec2) -> Self {
        self.uv_min = min;
        self.uv_max = max;
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

}

impl Painter<'_> {

    pub fn rect(&mut self, mut rect: PaintRect) {
        rect.rect = self.curr_transform() * rect.rect;
        rect.rounding *= self.curr_transform().scale;
        rect.stroke.width *= self.curr_transform().scale;
        self.resources.rect.push_rect(
            rect,
            self.curr_clip_rect(), 
            self.device,
            self.queue,
            &mut self.render_pass,
        );
    }

}
