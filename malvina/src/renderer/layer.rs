
use super::{BrushTexture, CanvasBorderRenderer, FillMesh, FillRenderer, OverlayCircleRenderer, OverlayLineRenderer, StrokeMesh, StrokeRenderer};

pub struct LayerRenderer<'rndr> {
    pub(super) device: &'rndr wgpu::Device,
    pub(super) queue: &'rndr wgpu::Queue,
    pub(super) render_pass: &'rndr mut wgpu::RenderPass<'rndr>,

    pub(super) view_proj: elic::Mat4,
    pub(super) resolution: elic::Vec2,
    pub(super) dpi_factor: f32,
    pub(super) zoom: f32,

    pub(super) stroke_renderer: &'rndr mut StrokeRenderer,
    pub(super) fill_renderer: &'rndr mut FillRenderer,
    pub(super) canvas_border_renderer: &'rndr mut CanvasBorderRenderer,
    pub(super) overlay_line_renderer: &'rndr mut OverlayLineRenderer, 
    pub(super) overlay_circle_renderer: &'rndr mut OverlayCircleRenderer, 

    pub(super) circle_brush: &'rndr BrushTexture
}

impl LayerRenderer<'_> {

    pub fn device(&self) -> &wgpu::Device {
        self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        self.queue
    }

    pub fn render_stroke(&mut self, stroke: &StrokeMesh, color: elic::Color, trans: elic::Mat4) {
        self.stroke_renderer.render(self.render_pass, stroke, self.circle_brush, color, self.resolution / self.dpi_factor, self.view_proj, trans);
    }

    pub(crate) fn render_stroke_picking(&mut self, stroke: &StrokeMesh, color: elic::Color, trans: elic::Mat4) {
        self.stroke_renderer.render_picking(self.render_pass, stroke, self.circle_brush, color, self.resolution / self.dpi_factor, self.view_proj, trans);
    }

    pub fn render_stroke_selection(&mut self, stroke: &StrokeMesh, color: elic::Color, trans: elic::Mat4) {
        self.stroke_renderer.render_selection(self.render_pass, stroke, self.circle_brush, color, self.resolution / self.dpi_factor, self.view_proj, trans);
    }

    pub fn render_fill(&mut self, fill: &FillMesh, color: elic::Color, trans: elic::Mat4) {
        self.fill_renderer.render(self.render_pass, fill, color, self.resolution / self.dpi_factor, self.view_proj, trans);
    }

    pub fn render_fill_selection(&mut self, fill: &FillMesh, color: elic::Color, trans: elic::Mat4) {
        self.fill_renderer.render_selection(self.render_pass, fill, color, self.resolution / self.dpi_factor, self.view_proj, trans);
    }

    pub fn render_canvas_border(&mut self, canvas_size: elic::Vec2) {
        self.canvas_border_renderer.render(self.render_pass, canvas_size, self.view_proj);
    }

    pub fn overlay_line(&mut self, a: elic::Vec2, b: elic::Vec2, color: elic::Color) {
        self.overlay_line_renderer.render_line(self.render_pass, a, b, 0.5 * self.dpi_factor / self.zoom, color, self.view_proj);
    }

    pub fn overlay_circle(&mut self, pos: elic::Vec2, r: f32, color: elic::Color) {
        self.overlay_circle_renderer.render_circle(self.render_pass, pos, r * self.dpi_factor / self.zoom, color, self.view_proj);
    }

    pub fn overlay_rect(&mut self, rect: elic::Rect, color: elic::Color) {
        self.overlay_line(rect.tl(), rect.tr(), color);
        self.overlay_line(rect.tr(), rect.br(), color);
        self.overlay_line(rect.br(), rect.bl(), color);
        self.overlay_line(rect.bl(), rect.tl(), color);
    }

}
