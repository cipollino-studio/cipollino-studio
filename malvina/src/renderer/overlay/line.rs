
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(super) struct LineUniforms {
    view_proj: [[f32; 4]; 4],
    color: [f32; 4],
    a: [f32; 2],
    b: [f32; 2],
    r: f32,
}

pub(crate) struct OverlayLineRenderer {
    render_pipeline: wgpu::RenderPipeline
}

impl OverlayLineRenderer {
    
    pub fn new(device: &wgpu::Device) -> Self {

        let line_shader = device.create_shader_module(wgpu::include_wgsl!("line.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("malvina_overlay_line_render_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[
                wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    range: 0..(size_of::<LineUniforms>() as u32),
                }
            ],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("malvina_overlay_line_render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &line_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
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
                module: &line_shader,
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
            render_pipeline
        }
    }

    pub fn render_line(&mut self, render_pass: &mut wgpu::RenderPass, a: elic::Vec2, b: elic::Vec2, r: f32, color: elic::Color, view_proj: elic::Mat4) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_push_constants(wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT, 0, bytemuck::cast_slice(&[
            LineUniforms {
                view_proj: view_proj.into(),
                a: a.into(),
                b: b.into(),
                r,
                color: color.into() 
            }
        ]));
        render_pass.draw(0..6, 0..1);
    }

}
