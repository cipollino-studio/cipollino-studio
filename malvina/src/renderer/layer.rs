
use super::{CameraUniformsBuffer, StrokeMesh, StrokeRenderer};

pub struct LayerRenderer<'renderer> {
    pub(super) device: &'renderer wgpu::Device,

    pub(super) render_pass: &'renderer mut wgpu::RenderPass<'renderer>,
    pub(super) camera: &'renderer CameraUniformsBuffer,
    pub(super) stroke_renderer: &'renderer mut StrokeRenderer
}

impl LayerRenderer<'_> {

    pub fn device(&self) -> &wgpu::Device {
        self.device
    }

    pub fn render_stroke(&mut self, stroke: &StrokeMesh) {
        self.stroke_renderer.render(self.render_pass, stroke, self.camera);
    }

}
