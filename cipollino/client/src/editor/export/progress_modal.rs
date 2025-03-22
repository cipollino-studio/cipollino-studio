
use std::path::PathBuf;

use project::{ClipInner, Ptr};

use crate::{render_scene, EditorState, ProjectState, State};

use super::VideoWriter;

pub(super) struct ExportProgressModal {
    writer: VideoWriter,
    time: i32,
    clip_ptr: Ptr<ClipInner>,

    width: u32,
    height: u32,
    msaa: u32,

    render_texture_width: u32,
    x_padding_offset: u32,

    render_texture: pierro::Texture,
    pixel_buffer: pierro::wgpu::Buffer

}

impl ExportProgressModal {

    pub fn new(out: PathBuf, clip_ptr: Ptr<ClipInner>, width: u32, height: u32, msaa: u32, framerate: f32, device: &pierro::wgpu::Device) -> Self {
        // Can't encode videos with odd dimensions
        let width = if width % 2 == 0 {
            width
        } else {
            width + 1
        };

        let height = if height % 2 == 0 {
           height 
        } else {
            height + 1
        };

        let render_texture_width = width * msaa;
        let render_texture_height = height * msaa;

        let padding_step = pierro::wgpu::COPY_BYTES_PER_ROW_ALIGNMENT / 4;
        let padded_render_texture_width = if render_texture_width % padding_step == 0 {
            render_texture_width
        } else {
            render_texture_width + padding_step - (render_texture_width % padding_step)
        };

        let x_padding_offset = (padded_render_texture_width - render_texture_width) / 2;

        Self {
            writer: VideoWriter::new(out, width, height, framerate).unwrap(),
            time: 0,
            clip_ptr,

            width,
            height,
            msaa,

            render_texture_width: padded_render_texture_width,
            x_padding_offset,

            render_texture: pierro::Texture::create_render_texture(device, padded_render_texture_width, render_texture_height),
            pixel_buffer: device.create_buffer(&pierro::wgpu::BufferDescriptor {
                label: Some("cipollino_export_pixel_buffer"),
                size: (padded_render_texture_width * render_texture_height * msaa * msaa * 4) as u64,
                usage: pierro::wgpu::BufferUsages::COPY_DST | pierro::wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            })
        }
    }

    fn render_frame(&mut self, ui: &mut pierro::UI, close: &mut bool, project: &ProjectState, editor: &mut EditorState, renderer: &mut Option<malvina::Renderer>, clip: &ClipInner) {
        if self.time >= clip.length as i32 {
            return;
        }

        if renderer.is_none() {
            *renderer = Some(malvina::Renderer::new(ui.wgpu_device(), ui.wgpu_queue()));
        }
        let Some(renderer) = renderer else {
            *close = true;
            let _ = self.writer.close();
            return;
        };

        // Render the scene into the render texture
        let camera = malvina::Camera::new(0.0, 0.0, (self.msaa as f32) * (self.width as f32) / (clip.width as f32));
        renderer.render(ui.wgpu_device(), ui.wgpu_queue(), self.render_texture.texture(), camera, elic::Color::WHITE, 1.0, |rndr| {
            render_scene(rndr, &project.client, editor, clip, self.time);
        });

        // Copy the render texture to the pixel copy buffer
        let mut encoder = ui.wgpu_device().create_command_encoder(&pierro::wgpu::CommandEncoderDescriptor {
            label: Some("cipollino_export_copy_pixels_encoder"),
        });
        let texture_copy_source = pierro::wgpu::ImageCopyTextureBase {
            texture: self.render_texture.texture(),
            mip_level: 0,
            origin: pierro::wgpu::Origin3d { x: 0, y: 0, z: 0 },
            aspect: pierro::wgpu::TextureAspect::All,
        };
        let texture_copy_dest = pierro::wgpu::ImageCopyBufferBase {
            buffer: &self.pixel_buffer,
            layout: pierro::wgpu::ImageDataLayout { offset: 0, bytes_per_row: Some(self.render_texture_width * 4), rows_per_image: None },
        };
        encoder.copy_texture_to_buffer(texture_copy_source, texture_copy_dest, pierro::wgpu::Extent3d { width: self.render_texture_width, height: self.height * self.msaa, depth_or_array_layers: 1 });
        ui.wgpu_queue().submit([encoder.finish()]);

        // Read the pixel copy buffer to the CPU
        self.pixel_buffer.slice(..).map_async(pierro::wgpu::MapMode::Read, |_| {});
        ui.wgpu_device().poll(pierro::wgpu::MaintainBase::Wait);
        let pixel_data = self.pixel_buffer.slice(..).get_mapped_range();

        // Extract the RGB pixel data and apply MSAA to get the frame that will be sent to the video writer
        // TODO: potentially apply MSAA on the GPU to speed up exports
        let mut rgb_data = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                for i in 0..self.msaa {
                    for j in 0..self.msaa {
                        let x_coord = x * self.msaa + i + self.x_padding_offset;
                        let y_coord = y * self.msaa + j;
                        let pixel_idx = x_coord + y_coord * self.render_texture_width;
                        let pixel_byte_idx = (pixel_idx as usize) * 4;
                        r += pixel_data[pixel_byte_idx + 0] as f32;
                        g += pixel_data[pixel_byte_idx + 1] as f32;
                        b += pixel_data[pixel_byte_idx + 2] as f32;
                    }
                }
                r /= (self.msaa * self.msaa) as f32;
                g /= (self.msaa * self.msaa) as f32;
                b /= (self.msaa * self.msaa) as f32;
                rgb_data.push(r.clamp(0.0, 255.0).round() as u8);
                rgb_data.push(g.clamp(0.0, 255.0).round() as u8);
                rgb_data.push(b.clamp(0.0, 255.0).round() as u8);
            }
        }

        // Write the frame to the video file
        let _ = self.writer.write_frame(rgb_data);

        // Unmap the pixel buffer
        drop(pixel_data);
        self.pixel_buffer.unmap();

        self.time += 1;

    }

}

impl pierro::Window for ExportProgressModal {

    type Context = State;

    const UNIQUE: bool = true;
    const MODAL: bool = true;

    fn title(&self) -> impl Into<String> {
        "Export"
    }

    fn render(&mut self, ui: &mut pierro::UI, close: &mut bool, state: &mut State) {
        let Some(clip) = state.project.client.get(self.clip_ptr) else {
            *close = true;
            let _ = self.writer.close();
            return;
        };
        if self.time == clip.length as i32 {
            pierro::label(ui, "Encoding video..."); 
        } else {
            pierro::label(ui, format!("Rendering frame #{} of {}.", self.time + 1, clip.length));
        }
        pierro::v_spacing(ui, 3.0);
       
        self.render_frame(ui, close, &state.project, &mut state.editor, &mut state.renderer, clip);

        pierro::image_with_width(ui, 300.0, self.render_texture.clone());

        if self.time >= clip.length as i32 {
            let _ = self.writer.close();
            if self.writer.done() {
                *close = true;
            }
        }

        ui.request_redraw();
    }

}
