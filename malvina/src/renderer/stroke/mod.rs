
mod mesh;
pub use mesh::*;

use super::{BrushTexture, BrushTextureResources};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct StrokeUniforms {
    trans: [[f32; 4]; 4],
    view_proj: [[f32; 4]; 4],
    pub color: [f32; 4],
    pub resolution: [f32; 2]
}

pub(super) struct StrokeRenderer {
    render_pipeline: wgpu::RenderPipeline,
    picking_pipeline: wgpu::RenderPipeline,
    selected_pipeline: wgpu::RenderPipeline,
}

impl StrokeRenderer {

    pub fn new(device: &wgpu::Device, brush_texture_resources: &BrushTextureResources) -> Self {

        let shader = device.create_shader_module(wgpu::include_wgsl!("render.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("malvina_stroke_render_pipeline_layout"),
            bind_group_layouts: &[&brush_texture_resources.bind_group_layout],
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStages::VERTEX_FRAGMENT,
                range: 0..(size_of::<StrokeUniforms>() as u32),
            }]
        });

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("malvina_stroke_render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[StrokeStampInstance::DESC]
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Stencil8,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default() 
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false 
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
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

        let picking_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("malvina_stroke_picking_render_pipeline"),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_picking",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL 
                })],
            }),
            ..(render_pipeline_descriptor.clone())
        });

        let selected_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("malvina_selected_stroke_render_pipeline"),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_selection",
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
            picking_pipeline,
            selected_pipeline,
        }
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass, stroke: &StrokeMesh, texture: &BrushTexture, color: elic::Color, resolution: elic::Vec2, view_proj: elic::Mat4, trans: elic::Mat4) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_push_constants(wgpu::ShaderStages::VERTEX_FRAGMENT, 0, bytemuck::cast_slice(&[StrokeUniforms {
            trans: trans.into(),
            view_proj: view_proj.into(),
            resolution: resolution.into(),
            color: color.into(),
        }]));
        render_pass.set_bind_group(0, &texture.bind_group, &[]);
        render_pass.set_vertex_buffer(0, stroke.instance_buffer.slice(..));
        render_pass.draw(0..6, 0..stroke.n_stamps);
    }

    pub fn render_picking(&mut self, render_pass: &mut wgpu::RenderPass, stroke: &StrokeMesh, texture: &BrushTexture, color: elic::Color, resolution: elic::Vec2, view_proj: elic::Mat4, trans: elic::Mat4) {
        render_pass.set_pipeline(&self.picking_pipeline);
        render_pass.set_push_constants(wgpu::ShaderStages::VERTEX_FRAGMENT, 0, bytemuck::cast_slice(&[StrokeUniforms {
            trans: trans.into(),
            view_proj: view_proj.into(),
            resolution: resolution.into(),
            color: color.into(),
        }]));
        render_pass.set_bind_group(0, &texture.bind_group, &[]);
        render_pass.set_vertex_buffer(0, stroke.instance_buffer.slice(..));
        render_pass.draw(0..6, 0..stroke.n_stamps);
    }

    pub fn render_selection(&mut self, render_pass: &mut wgpu::RenderPass, stroke: &StrokeMesh, texture: &BrushTexture, color: elic::Color, resolution: elic::Vec2, view_proj: elic::Mat4, trans: elic::Mat4) {
        render_pass.set_pipeline(&self.selected_pipeline);
        render_pass.set_push_constants(wgpu::ShaderStages::VERTEX_FRAGMENT, 0, bytemuck::cast_slice(&[StrokeUniforms {
            trans: trans.into(),
            view_proj: view_proj.into(),
            resolution: resolution.into(),
            color: color.contrasting_color().into(),
        }]));
        render_pass.set_bind_group(0, &texture.bind_group, &[]);
        render_pass.set_vertex_buffer(0, stroke.instance_buffer.slice(..));
        render_pass.draw(0..6, 0..stroke.n_stamps);
    }

}
