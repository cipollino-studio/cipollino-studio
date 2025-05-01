
mod stroke;
pub use stroke::*;

mod fill;
pub use fill::*;

mod camera;
pub use camera::*;

mod layer;
pub use layer::*;

mod picking;
pub use picking::*;

mod canvas_border;
use canvas_border::*;

mod overlay;
pub(crate) use overlay::*;

mod brush;
pub use brush::*;

pub struct Renderer {
    stroke: StrokeRenderer,
    fill: FillRenderer,
    canvas_border: CanvasBorderRenderer,
    line_renderer: OverlayLineRenderer,
    circle_renderer: OverlayCircleRenderer,
    circle_brush: BrushTexture,
    
    stencil_texture: wgpu::Texture
}

fn make_stencil_texture(device: &wgpu::Device, w: u32, h: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("malvina_stencil_texture"),
        size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Stencil8,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    })
}

impl Renderer {

    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let brush_texture_resources = BrushTextureResources::new(device);
        let stroke = StrokeRenderer::new(device, &brush_texture_resources);
        let fill = FillRenderer::new(device);
        let canvas_border = CanvasBorderRenderer::new(device);
        let line_renderer = OverlayLineRenderer::new(device);
        let circle_renderer = OverlayCircleRenderer::new(device);
        let circle_brush = BrushTexture::circle(device, queue, &brush_texture_resources, 512);
        
        let stencil_texture = make_stencil_texture(device, 100, 100);

        Self {
            stroke,
            fill,
            canvas_border,
            line_renderer,
            circle_renderer,
            circle_brush,
            stencil_texture
        }
    }

    pub fn render<F: FnOnce(&mut LayerRenderer)>(&mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        texture: &wgpu::Texture,
        camera: Camera,
        fill_color: elic::Color,
        dpi_factor: f32,
        contents: F
    ) {

        // Resize the stencil texture if needed
        if texture.width() != self.stencil_texture.width() || texture.height() != self.stencil_texture.height() {
            self.stencil_texture = make_stencil_texture(device, texture.width(), texture.height());
        }

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let resolution = elic::vec2(texture.width() as f32, texture.height() as f32);

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
                            r: fill_color.r as f64,
                            g: fill_color.g as f64,
                            b: fill_color.b as f64,
                            a: fill_color.a as f64
                        }),
                        store: wgpu::StoreOp::Store
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.stencil_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                    depth_ops: None,
                    stencil_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0),
                        store: wgpu::StoreOp::Store
                    }) 
                }), 
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
                zoom: camera.zoom,
                stroke_renderer: &mut self.stroke,
                fill_renderer: &mut self.fill,
                canvas_border_renderer: &mut self.canvas_border,
                overlay_line_renderer: &mut self.line_renderer,
                overlay_circle_renderer: &mut self.circle_renderer,
                
                circle_brush: &self.circle_brush
            };

            contents(&mut layer_renderer);

        }

        queue.submit([encoder.finish()]);
    }
    
}
