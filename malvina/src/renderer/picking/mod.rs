
use super::{Camera, LayerRenderer, Renderer, StrokeMesh};

mod buffer;
pub use buffer::*;

pub struct PickingRenderer<'a, 'rndr> {
    renderer: &'a mut LayerRenderer<'rndr>
}

impl PickingRenderer<'_, '_> {

    fn id_to_color(id: u32) -> glam::Vec4 {
        let r = (id >> 0)  & 0xFF;
        let g = (id >> 8)  & 0xFF;
        let b = (id >> 16) & 0xFF;
        
        let r = (r as f32) / 255.0;
        let g = (g as f32) / 255.0;
        let b = (b as f32) / 255.0;

        glam::vec4(r, g, b, 1.0)
    }

    pub fn render_stroke(&mut self, stroke: &StrokeMesh, id: u32) {
        self.renderer.render_stroke_picking(stroke, Self::id_to_color(id));
    }

}

impl Renderer {

    pub fn render_picking<F: FnOnce(&mut PickingRenderer)>(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, picking: &PickingBuffer, camera: Camera, contents: F) {

        let Some(texture) = picking.texture.as_ref() else {
            return;
        };

        self.render(device, queue, texture, camera, glam::vec4(0.0, 0.0, 0.0, 1.0), 1.0, |rndr| {
            let mut rndr = PickingRenderer {
                renderer: rndr,
            };
            contents(&mut rndr);
        });

    }


}
