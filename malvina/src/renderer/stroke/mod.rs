
use super::CameraUniformsBuffer;

mod mesh;
pub use mesh::*;

pub(super) struct StrokeRenderer {
    render_pipeline: wgpu::RenderPipeline
}

impl StrokeRenderer {

    pub fn new(device: &wgpu::Device, camera: &CameraUniformsBuffer) -> Self {

        let render_shader = device.create_shader_module(wgpu::include_wgsl!("render.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("malvina_stroke_render_pipeline_layout"),
            bind_group_layouts: &[&camera.bind_group_layout],
            push_constant_ranges: &[] 
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
        });

        Self {
            render_pipeline
        }
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass, stroke: &StrokeMesh, camera: &CameraUniformsBuffer) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &camera.bind_group, &[]);
        render_pass.set_vertex_buffer(0, stroke.instance_buffer.slice(..));
        render_pass.draw(0..6, 0..stroke.n_stamps);
    }

}
