
mod stroke;
pub use stroke::*;

mod camera;
pub use camera::*;

mod layer;
pub use layer::*;

mod picking;
pub use picking::*;

mod canvas_border;
use canvas_border::*;

mod brush;
pub use brush::*;

pub struct Renderer {
    stroke: StrokeRenderer,
    canvas_border: CanvasBorderRenderer,
    circle_brush: BrushTexture
}

impl Renderer {

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let brush_texture_resources = BrushTextureResources::new(device);
        let stroke = StrokeRenderer::new(device, &brush_texture_resources);
        let canvas_border = CanvasBorderRenderer::new(device);
        let circle_brush = BrushTexture::circle(device, queue, &brush_texture_resources, 100);
        Self {
            stroke,
            canvas_border,
            circle_brush
        }
    }

    pub fn render<F: FnOnce(&mut LayerRenderer)>(&mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &wgpu::Texture,
        camera: Camera,
        fill_color: glam::Vec4,
        dpi_factor: f32,
        contents: F
    ) {

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let resolution = glam::vec2(texture.width() as f32, texture.height() as f32);

        let view_proj = camera.calc_view_proj(resolution);

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
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: fill_color.x as f64,
                            g: fill_color.y as f64,
                            b: fill_color.z as f64,
                            a: fill_color.w as f64
                        }),
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
                view_proj: view_proj,
                resolution,
                dpi_factor,
                stroke_renderer: &mut self.stroke,
                canvas_border_renderer: &mut self.canvas_border,
                circle_brush: &self.circle_brush
            };

            contents(&mut layer_renderer);

        }

        queue.submit([encoder.finish()]);
    }
    
}
