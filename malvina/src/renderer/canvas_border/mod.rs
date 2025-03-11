
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct CanvasBorderUniforms {
    view_proj: glam::Mat4,
    canvas_size: glam::Vec2,
    padding: [f32; 2] // Needed to make bytemuck::Pod compile
}

pub(super) struct CanvasBorderRenderer {
    render_pipeline: wgpu::RenderPipeline,
}

impl CanvasBorderRenderer {

    pub fn new(device: &wgpu::Device) -> Self {

        let render_shader = device.create_shader_module(wgpu::include_wgsl!("render.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("malvina_camera_border_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[
                wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::VERTEX,
                    range: 0..(size_of::<CanvasBorderUniforms>() as u32),
                }
            ],
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
        }
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass, canvas_size: glam::Vec2, view_proj: glam::Mat4) {

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_push_constants(wgpu::ShaderStages::VERTEX, 0, bytemuck::cast_slice(&[
            CanvasBorderUniforms {
                view_proj,
                canvas_size,
                padding: [0.0, 0.0]
            }
        ]));
        render_pass.draw(0..24, 0..1);
    }

}
