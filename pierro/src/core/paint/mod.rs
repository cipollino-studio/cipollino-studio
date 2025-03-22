
mod window_config;

pub use window_config::*;
use crate::TSTransform;

mod texture;
pub use texture::*;

mod stroke;
pub use stroke::*;

mod rect;
pub use rect::*;

mod text;
pub use text::*;

mod clip;
mod transform;

use crate::{text::TextResources, Rect, Vec2};

pub(crate) struct PainterResources {
    rect: RectResources
}

impl PainterResources {

    pub(crate) fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        let rect = RectResources::new(device, surface_format);
        Self {
            rect
        }
    }

    pub(super) fn begin_frame(&mut self, queue: &wgpu::Queue, screen_size: Vec2) {
        self.rect.begin_frame(queue, screen_size);
    }

}

pub struct Painter<'a> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    render_pass: wgpu::RenderPass<'a>,
    resources: &'a mut PainterResources,
    text_resources: &'a mut TextResources,

    dpi_scale: f32,

    clip_stack: Vec<Rect>,
    transform_stack: Vec<TSTransform>,

    text_render_cache: &'a mut TextRenderCache,
    next_text_render_cache: &'a mut TextRenderCache
}

impl<'a> Painter<'a> {

    pub(crate) fn new(
        device: &'a wgpu::Device, 
        queue: &'a wgpu::Queue,
        encoder: &'a mut wgpu::CommandEncoder,
        view: &'a wgpu::TextureView,

        resources: &'a mut PainterResources,
        text_resources: &'a mut TextResources,

        size: Vec2,
        dpi_scale: f32,

        text_render_cache: &'a mut TextRenderCache,
        next_text_render_cache: &'a mut TextRenderCache
    ) -> Self {

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("pierro_paint_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 }),
                    store: wgpu::StoreOp::Store
                }
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        Self {
            device,
            queue,
            render_pass,

            resources,
            text_resources,

            dpi_scale,

            clip_stack: vec![Rect::min_size(Vec2::ZERO, size)],
            transform_stack: vec![TSTransform::IDENTITY],

            text_render_cache,
            next_text_render_cache
        }
    }

    pub(super) fn finish(mut self) {
        self.resources.rect.flush_buffer(&self.device, self.queue, &mut self.render_pass);
        self.resources.rect.finish();
    }

    pub fn scale_factor(&self) -> f32 {
        self.dpi_scale
    }

}
