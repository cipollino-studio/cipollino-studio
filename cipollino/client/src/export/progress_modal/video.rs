
use project::ClipInner;

use crate::{export::{video_writer::VideoWriter, ExportProgressModal}, render_scene, EditorState, ProjectState, RendererState};

impl ExportProgressModal {

    pub(super) fn render_frame(
        ui: &mut pierro::UI,
        close: &mut bool,
        project: &ProjectState,
        editor: &mut EditorState,
        renderer: &mut Option<RendererState>,
        clip: &ClipInner,

        width: u32,
        height: u32,
        render_texture_width: u32,
        x_padding_offset: u32,
        msaa: u32,
        time: &mut i32,
        writer: &mut VideoWriter,
        render_texture: &mut pierro::Texture,
        pixel_buffer: &mut pierro::wgpu::Buffer
    ) {
        if *time >= clip.length as i32 {
            return;
        }

        if renderer.is_none() {
            *renderer = Some(RendererState::new(ui.wgpu_device(), ui.wgpu_queue()));
        }
        let Some(renderer) = renderer else {
            *close = true;
            let _ = writer.close();
            return;
        };

        // Render the scene into the render texture
        let camera = malvina::Camera::new(elic::Vec2::ZERO, (msaa as f32) * (width as f32) / (clip.width as f32));
        renderer.renderer.render(ui.wgpu_device(), ui.wgpu_queue(), render_texture.texture(), camera, clip.background_color.into(), 1.0, |rndr| {
            render_scene(rndr, &renderer.builtin_brushes, &project.client, editor, clip, *time, false);
        });

        // Copy the render texture to the pixel copy buffer
        let mut encoder = ui.wgpu_device().create_command_encoder(&pierro::wgpu::CommandEncoderDescriptor {
            label: Some("cipollino_export_copy_pixels_encoder"),
        });
        let texture_copy_source = pierro::wgpu::ImageCopyTextureBase {
            texture: render_texture.texture(),
            mip_level: 0,
            origin: pierro::wgpu::Origin3d { x: 0, y: 0, z: 0 },
            aspect: pierro::wgpu::TextureAspect::All,
        };
        let texture_copy_dest = pierro::wgpu::ImageCopyBufferBase {
            buffer: &*pixel_buffer,
            layout: pierro::wgpu::ImageDataLayout { offset: 0, bytes_per_row: Some(render_texture_width * 4), rows_per_image: None },
        };
        encoder.copy_texture_to_buffer(texture_copy_source, texture_copy_dest, pierro::wgpu::Extent3d { width: render_texture_width, height: height * msaa, depth_or_array_layers: 1 });
        ui.wgpu_queue().submit([encoder.finish()]);

        // Read the pixel copy buffer to the CPU
        pixel_buffer.slice(..).map_async(pierro::wgpu::MapMode::Read, |_| {});
        ui.wgpu_device().poll(pierro::wgpu::MaintainBase::Wait);
        let pixel_data = pixel_buffer.slice(..).get_mapped_range();

        // Extract the RGB pixel data and apply MSAA to get the frame that will be sent to the video writer
        // TODO: potentially apply MSAA on the GPU to speed up exports
        let mut rgb_data = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                for i in 0..msaa {
                    for j in 0..msaa {
                        let x_coord = x * msaa + i + x_padding_offset;
                        let y_coord = y * msaa + j;
                        let pixel_idx = x_coord + y_coord * render_texture_width;
                        let pixel_byte_idx = (pixel_idx as usize) * 4;
                        r += pixel_data[pixel_byte_idx + 0] as f32;
                        g += pixel_data[pixel_byte_idx + 1] as f32;
                        b += pixel_data[pixel_byte_idx + 2] as f32;
                    }
                }
                r /= (msaa * msaa) as f32;
                g /= (msaa * msaa) as f32;
                b /= (msaa * msaa) as f32;
                rgb_data.push(r.clamp(0.0, 255.0).round() as u8);
                rgb_data.push(g.clamp(0.0, 255.0).round() as u8);
                rgb_data.push(b.clamp(0.0, 255.0).round() as u8);
            }
        }

        // Write the frame to the video file
        let _ = writer.write_frame(rgb_data);

        // Unmap the pixel buffer
        drop(pixel_data);
        pixel_buffer.unmap();

        *time += 1;

    }

}
