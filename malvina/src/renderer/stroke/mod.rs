
mod mesh;
pub use mesh::*;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct StrokeUniforms {
    view_proj: glam::Mat4,
    pub color: glam::Vec4
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct SelectedStrokeUniforms {
    view_proj: glam::Mat4,
    resolution: glam::Vec2,
    color: f32,
    padding: f32
}

pub(super) struct StrokeRenderer {
    render_pipeline: wgpu::RenderPipeline,
    selected_pipeline: wgpu::RenderPipeline,
}

impl StrokeRenderer {

    pub fn new(device: &wgpu::Device) -> Self {

        let render_shader = device.create_shader_module(wgpu::include_wgsl!("render.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("malvina_stroke_render_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStages::VERTEX_FRAGMENT,
                range: 0..(size_of::<StrokeUniforms>() as u32),
            }]
        });

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("malvina_stroke_render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &render_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[StrokeStampInstance::DESC]
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
                alpha_to_coverage_enabled: false 
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL 
                })],
            }),
            multiview: None,
            cache: None 
        };

        let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

        let selected_shader = device.create_shader_module(wgpu::include_wgsl!("selected.wgsl"));
        let selected_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("malvina_selected_stroke_render_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStages::VERTEX_FRAGMENT,
                range: 0..(size_of::<SelectedStrokeUniforms>() as u32),
            }]
        });
        let selected_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("malvina_selected_stroke_render_pipeline"),
            layout: Some(&selected_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &selected_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[StrokeStampInstance::DESC]
            },
            fragment: Some(wgpu::FragmentState {
                module: &selected_shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL 
                })],
            }),
            ..render_pipeline_descriptor
        });

        Self {
            render_pipeline,
            selected_pipeline
        }
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass, stroke: &StrokeMesh, color: glam::Vec4, view_proj: glam::Mat4) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_push_constants(wgpu::ShaderStages::VERTEX_FRAGMENT, 0, bytemuck::cast_slice(&[StrokeUniforms {
            view_proj,
            color,
        }]));
        render_pass.set_vertex_buffer(0, stroke.instance_buffer.slice(..));
        render_pass.draw(0..6, 0..stroke.n_stamps);
    }

    pub fn render_selection(&mut self, render_pass: &mut wgpu::RenderPass, stroke: &StrokeMesh, color: glam::Vec4, resolution: glam::Vec2, dpi_factor: f32, view_proj: glam::Mat4) {
        let color = if (color.x + color.y + color.w) / 3.0 > 0.5 { 0.0 } else { 1.0 };
        render_pass.set_pipeline(&self.selected_pipeline);
        render_pass.set_push_constants(wgpu::ShaderStages::VERTEX_FRAGMENT, 0, bytemuck::cast_slice(&[SelectedStrokeUniforms {
            view_proj,
            resolution: resolution / dpi_factor,
            color,
            padding: 0.0
        }]));
        render_pass.set_vertex_buffer(0, stroke.instance_buffer.slice(..));
        render_pass.draw(0..6, 0..stroke.n_stamps);
    }

}
