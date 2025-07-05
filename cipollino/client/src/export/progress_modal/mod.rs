
use std::{collections::{HashSet, VecDeque}, path::PathBuf, thread::JoinHandle};

use project::{AudioBlock, AudioFormat, ClipInner, Ptr};
use rand::RngCore;

use crate::{export::audio_writer::audio_encoding_thread, AppSystems, AudioBlockCache, AudioPlaybackState, LayerRenderList, PanelContext, ProjectState, RenderLayerKind, Window};

use super::VideoWriter;

mod video;

// Video export process:
// 1. Load and resample all audio blocks in the clip
// 2. Export the audio to a temporary .mp3 file
// 3. Export the video, using the temporary .mp3 for the sound
// 4. Delete the temporary .mp3

enum ExportState {
    AudioLoad {
        blocks_to_load: Vec<(AudioFormat, Ptr<AudioBlock>)>,
        blocks_to_resample: VecDeque<(AudioFormat, Ptr<AudioBlock>)>,
        cache: AudioBlockCache,
        audio_length: i64,
        total_blocks: usize 
    },
    Audio {
        thread: JoinHandle<()>,
    },
    Video {
        time: i32,
        writer: VideoWriter
    }
}

pub(super) struct ExportProgressModal {
    state: ExportState,
    clip_ptr: Ptr<ClipInner>,
    audio_path: PathBuf,
    out_path: PathBuf,

    width: u32,
    height: u32,
    msaa: u32,
    sample_rate: u32,

    render_texture_width: u32,
    x_padding_offset: u32,

    render_texture: pierro::Texture,
    pixel_buffer: pierro::wgpu::Buffer
}

impl ExportProgressModal {

    pub fn new(project: &ProjectState, systems: &AppSystems, out: PathBuf, clip_ptr: Ptr<ClipInner>, clip: &ClipInner, layers: &LayerRenderList, width: u32, height: u32, msaa: u32, sample_rate: u32, device: &pierro::wgpu::Device) -> Self {
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

        // Audio
        let audio_file_name = format!("audioexport-{}.mp3", rand::rng().next_u32());
        let audio_path = systems.audio_tmp_path.join::<PathBuf>(audio_file_name.into());
        let audio_length = ((clip.length as f32 / clip.framerate) * (sample_rate as f32)).ceil() as i64;

        // Get blocks to load/resample
        let mut blocks_to_load = Vec::new();
        let mut blocks_to_resample = VecDeque::new();
        for layer in layers.iter() {
            match layer.kind {
                RenderLayerKind::AudioLayer(_, audio_layer) => {
                    for audio in audio_layer.audio_instances.iter() {
                        let Some(audio) = project.client.get(audio.ptr()) else { continue; };
                        let Some(clip) = project.client.get(audio.clip) else { continue; };
                        for (_, block) in clip.blocks.iter() {
                            if project.client.get_ref(*block).is_loaded() {
                                blocks_to_resample.push_back((clip.format, *block));
                            } else {
                                blocks_to_load.push((clip.format, *block));
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        let total_blocks = blocks_to_load.len() + blocks_to_resample.len();

        Self {
            state: ExportState::AudioLoad {
                blocks_to_load,
                blocks_to_resample,
                cache: AudioBlockCache::new(sample_rate),
                audio_length,
                total_blocks
            },
            clip_ptr,

            out_path: out,
            audio_path,

            width,
            height,
            msaa,
            sample_rate,

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

}

impl Window for ExportProgressModal {

    fn title(&self) -> String {
        "Export".to_owned()
    }

    fn render<'ctx>(&mut self, ui: &mut pierro::UI, close: &mut bool, ctx: &mut PanelContext<'ctx>) {
        let Some(clip) = ctx.project.client.get(self.clip_ptr) else {
            *close = true;
            return;
        };
        match &mut self.state {
            ExportState::AudioLoad { blocks_to_load, blocks_to_resample, cache, audio_length, total_blocks } => {
                let blocks_processed = *total_blocks - (blocks_to_load.len() + blocks_to_resample.len());
                pierro::label(ui, "Loading audio...");
                pierro::progress_bar(ui, (blocks_processed as f32) / (*total_blocks as f32));
                if let Some((format, to_resample)) = blocks_to_resample.front() {
                    match ctx.project.client.get_ref(*to_resample) {
                        alisa::ObjRef::Loading => {},
                        alisa::ObjRef::Loaded(block) => {
                            cache.get_samples(*format, *to_resample, block);
                            blocks_to_resample.pop_front();
                        },
                        alisa::ObjRef::None |
                        alisa::ObjRef::Deleted => {
                            blocks_to_resample.pop_front();
                        },
                    }
                }

                if let Some((format, to_load)) = blocks_to_load.pop() {
                    ctx.project.client.request_load(to_load);
                    blocks_to_resample.push_back((format, to_load));
                }

                if blocks_to_load.is_empty() && blocks_to_resample.is_empty() {
                    let Some(layers) = ctx.layer_render_list else {
                        *close = true;
                        return;
                    };
                    let audio_state = AudioPlaybackState::construct(ctx.project, layers, self.sample_rate, cache, &HashSet::new());
                    let audio_path = self.audio_path.clone();
                    let audio_length = *audio_length;
                    let thread = std::thread::spawn(move || {
                        audio_encoding_thread(audio_path, audio_state, audio_length);
                    });
                    self.state = ExportState::Audio {
                        thread
                    };
                }
            },
            ExportState::Audio { thread } => {
                pierro::label(ui, "Encoding audio...");
                if thread.is_finished() {
                    let Ok(writer) = VideoWriter::new(self.out_path.clone(), self.width, self.height, clip.framerate, &self.audio_path) else {
                        *close = true;
                        return;
                    };
                    self.state = ExportState::Video {
                        time: 0,
                        writer 
                    }
                }
            },
            ExportState::Video { time, writer } => {
                
                if *time == clip.length as i32 {
                    pierro::label(ui, "Encoding video...");
                } else {
                    pierro::label(ui, format!("Rendering frame #{} of {}.", *time + 1, clip.length));
                }
                pierro::v_spacing(ui, 3.0);
            
                Self::render_frame(
                    ui,
                    close,
                    &ctx.project,
                    &mut ctx.editor,
                    &mut ctx.renderer,
                    clip,
                    self.width,
                    self.height,
                    self.render_texture_width,
                    self.x_padding_offset,
                    self.msaa,
                    time,
                    writer,
                    &mut self.render_texture,
                    &mut self.pixel_buffer
                );

                pierro::image_with_width(ui, 300.0, self.render_texture.clone());

                if *time >= clip.length as i32 {
                    let _ = writer.close();
                    if writer.done() {
                        *close = true;
                    }
                }
            }
        }
        

        ui.request_redraw();
    }

    fn modal(&self) -> bool {
        true 
    }

    fn unique(&self) -> bool {
        true 
    }

}

impl Drop for ExportProgressModal {

    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.audio_path);
    }

}
