
use super::{CameraUniformsBuffer, CanvasBorderRenderer, StrokeMesh, StrokeRenderer};

pub struct LayerRenderer<'renderer> {
    pub(super) device: &'renderer wgpu::Device,
    pub(super) queue: &'renderer wgpu::Queue,

    pub(super) render_pass: &'renderer mut wgpu::RenderPass<'renderer>,
    pub(super) camera: &'renderer CameraUniformsBuffer,
    pub(super) stroke_renderer: &'renderer mut StrokeRenderer,
    pub(super) canvas_border_renderer: &'renderer mut CanvasBorderRenderer
}

impl LayerRenderer<'_> {

    pub fn device(&self) -> &wgpu::Device {
        self.device
    }

    pub fn render_stroke(&mut self, stroke: &StrokeMesh) {
        self.stroke_renderer.render(self.render_pass, stroke, self.camera);
    }

    pub fn render_canvas_border(&mut self, canvas_size: glam::Vec2) {
        self.canvas_border_renderer.render(self.queue, self.render_pass, canvas_size, self.camera);
    }

}
