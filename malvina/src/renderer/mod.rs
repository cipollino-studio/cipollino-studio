
mod stroke;
pub use stroke::*;

mod camera;
pub use camera::*;

mod layer;
pub use layer::*;

mod canvas_border;
use canvas_border::*;

pub struct Renderer {
    camera: CameraUniformsBuffer,
    stroke: StrokeRenderer,
    canvas_border: CanvasBorderRenderer
}

impl Renderer {

    pub fn new(device: &wgpu::Device) -> Self {
        let camera = CameraUniformsBuffer::new(device);
        let stroke = StrokeRenderer::new(device, &camera);
        let canvas_border = CanvasBorderRenderer::new(device, &camera);
        Self {
            camera,
            stroke,
            canvas_border
        }
    }

    pub fn render<F: FnOnce(&mut LayerRenderer)>(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, texture: &wgpu::Texture, camera: Camera, contents: F) {

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let resolution = glam::vec2(texture.width() as f32, texture.height() as f32);

        self.camera.update(queue, &camera, resolution);

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("malvina_encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("malvina_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None 
            });
            
            let mut layer_renderer = LayerRenderer {
                device,
                queue,
                render_pass: &mut render_pass,
                camera: &self.camera,
                stroke_renderer: &mut self.stroke,
                canvas_border_renderer: &mut self.canvas_border
            };

            contents(&mut layer_renderer);
        }

        queue.submit([encoder.finish()]);
    }

}
