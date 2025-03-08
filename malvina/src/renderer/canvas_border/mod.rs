
use super::CameraUniformsBuffer;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct CanvasBorderUniforms {
    canvas_size: glam::Vec2
}

pub(super) struct CanvasBorderRenderer {
    render_pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup
}

impl CanvasBorderRenderer {

    pub fn new(device: &wgpu::Device, camera: &CameraUniformsBuffer) -> Self {

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("malvina_camera_border_uniforms_buffer"),
            size: size_of::<CanvasBorderUniforms>() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("malvina_camera_border_uniforms_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0, 
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None,
            }],
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("malvina_camera_border_uniforms_bind_group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding() 
                }
            ] 
        });

        let render_shader = device.create_shader_module(wgpu::include_wgsl!("render.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("malvina_camera_border_pipeline_layout"),
            bind_group_layouts: &[&camera.bind_group_layout, &uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("malvina_camera_border_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &render_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            render_pipeline,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub fn render(&mut self, queue: &wgpu::Queue, render_pass: &mut wgpu::RenderPass, canvas_size: glam::Vec2, camera: &CameraUniformsBuffer) {

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[
            CanvasBorderUniforms {
                canvas_size,
            } 
        ]));

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &camera.bind_group, &[]);
        render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
        render_pass.draw(0..24, 0..1);
    }

}
