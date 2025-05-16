
mod mesh;
pub use mesh::*;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct FillUniforms {
    trans: [[f32; 4]; 4],
    view_proj: [[f32; 4]; 4],
    pub color: [f32; 4],
    pub resolution: [f32; 2]
}

pub(super) struct FillRenderer {
    render_pipeline: wgpu::RenderPipeline,
    stencil_pipeline: wgpu::RenderPipeline,
    selected_pipeline: wgpu::RenderPipeline,
}

impl FillRenderer {

    pub fn new(device: &wgpu::Device) -> Self {

        let shader = device.create_shader_module(wgpu::include_wgsl!("render.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("malvina_fill_render_pipeline_layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStages::VERTEX_FRAGMENT,
                range: 0..(size_of::<FillUniforms>() as u32),
            }] 
        });

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("malvina_fill_render_pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[FillVertex::DESC] 
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false 
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Stencil8,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState {
                    front: wgpu::StencilFaceState {
                        compare: wgpu::CompareFunction::NotEqual,
                        fail_op: wgpu::StencilOperation::Zero,
                        depth_fail_op: wgpu::StencilOperation::Zero,
                        pass_op: wgpu::StencilOperation::Zero 
                    },
                    back: wgpu::StencilFaceState {
                        compare: wgpu::CompareFunction::NotEqual,
                        fail_op: wgpu::StencilOperation::Zero,
                        depth_fail_op: wgpu::StencilOperation::Zero,
                        pass_op: wgpu::StencilOperation::Zero 
                    },
                    read_mask: 0xFF,
                    write_mask: 0xFF 
                },
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
                })] 
            }),
            multiview: None,
            cache: None,
        };

        let render_pipeline = device.create_render_pipeline(&render_pipeline_descriptor);

        let stencil_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("malvina_fil_stencil_render_pipeline"),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Stencil8,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState {
                    front: wgpu::StencilFaceState {
                        compare: wgpu::CompareFunction::Always,
                        fail_op: wgpu::StencilOperation::Invert,
                        depth_fail_op: wgpu::StencilOperation::Invert,
                        pass_op: wgpu::StencilOperation::Invert
                    },
                    back: wgpu::StencilFaceState {
                        compare: wgpu::CompareFunction::Always,
                        fail_op: wgpu::StencilOperation::Invert,
                        depth_fail_op: wgpu::StencilOperation::Invert,
                        pass_op: wgpu::StencilOperation::Invert 
                    },
                    read_mask: 0xFF,
                    write_mask: 0xFF 
                },
                bias: wgpu::DepthBiasState::default() 
            }),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::empty() 
                })]
            }),
            ..render_pipeline_descriptor.clone()
        });

        let selected_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("malvina_selected_fill_render_pipeline"),
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
            stencil_pipeline,
            selected_pipeline,
        }
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass, fill: &FillMesh, color: elic::Color, resolution: elic::Vec2, view_proj: elic::Mat4, trans: elic::Mat4) {
        render_pass.set_pipeline(&self.stencil_pipeline);
        render_pass.set_push_constants(wgpu::ShaderStages::VERTEX_FRAGMENT, 0, bytemuck::cast_slice(&[FillUniforms {
            trans: trans.into(),
            view_proj: view_proj.into(),
            color: color.into(),
            resolution: resolution.into(),
        }]));
        render_pass.set_vertex_buffer(0, fill.vertex_buffer.slice(..));
        render_pass.draw(0..fill.n_verts, 0..1);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.draw(0..fill.n_verts, 0..1);
    }

    pub fn render_selection(&mut self, render_pass: &mut wgpu::RenderPass, fill: &FillMesh, color: elic::Color, resolution: elic::Vec2, view_proj: elic::Mat4, trans: elic::Mat4) {
        render_pass.set_pipeline(&self.stencil_pipeline);
        render_pass.set_push_constants(wgpu::ShaderStages::VERTEX_FRAGMENT, 0, bytemuck::cast_slice(&[FillUniforms {
            trans: trans.into(),
            view_proj: view_proj.into(),
            color: color.contrasting_color().into(),
            resolution: resolution.into(),
        }]));
        render_pass.set_vertex_buffer(0, fill.vertex_buffer.slice(..));
        render_pass.draw(0..fill.n_verts, 0..1);
        render_pass.set_pipeline(&self.selected_pipeline);
        render_pass.draw(0..fill.n_verts, 0..1);
    }

}
