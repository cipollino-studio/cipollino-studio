
use std::sync::Arc;

use alisa::Ptr;
use project::{Action, AudioInstance, AudioLayer, ClipInner, CreateAudioInstance};

use crate::{panels::timeline::frame_area::PaintCommands, AssetList, EditorState, ProjectState, SampleBlock, TimelinePanel};

use super::FrameArea;

pub(super) struct AudioInstanceBar {
    pub layer_idx: usize, 
    // The start/end times are in seconds
    pub start: f32,
    pub end: f32,

    pub volume_previews: Vec<(pierro::Range, Arc<SampleBlock>)>
}

impl AudioInstanceBar {

    pub fn paint(self, painter: &mut pierro::Painter, rect: pierro::Rect, framerate: f32, accent_color: pierro::Color) {
        let rect = pierro::Rect::min_size(
            rect.tl() + pierro::vec2(self.start * framerate * TimelinePanel::FRAME_WIDTH, (self.layer_idx as f32) * TimelinePanel::LAYER_HEIGHT),
            pierro::vec2((self.end - self.start) * framerate * TimelinePanel::FRAME_WIDTH, TimelinePanel::LAYER_HEIGHT)
        );

        let bg_color = accent_color.darken(0.2);
        painter.rect(pierro::PaintRect::new(rect, bg_color).with_rounding(pierro::Rounding::same(5.0)));

        let volume_color = accent_color.darken(0.6);
        for (range, samples) in self.volume_previews {
            if samples.volume.is_empty() {
                continue;
            }
            let x_range = (range * framerate * TimelinePanel::FRAME_WIDTH).shift(rect.left());
            for x in ((x_range.min.floor() as i32)..(x_range.max.ceil() as i32)).step_by(2) {
                let t = (x as f32 - x_range.min) / x_range.size(); 
                let idx = ((t * (samples.volume.len() as f32)).round() as i32).clamp(0, samples.volume.len() as i32 - 1) as usize;
                let volume = samples.volume[idx].powf(0.6) * 0.97 + 0.03;
                let volume_sample_rect = pierro::Rect::center_size(
                    pierro::vec2(x as f32 + 1.0, rect.center().y),
                    pierro::vec2(2.0, rect.height() * volume)
                );
                painter.rect(pierro::PaintRect::new(volume_sample_rect, volume_color));
            }
        }
    }

}

impl FrameArea {

    fn handle_dnd(
        &mut self,
        ui: &mut pierro::UI,
        project: &ProjectState,
        editor: &mut EditorState,
        _frame_area: &pierro::Response,
        paint_commands: &mut PaintCommands,
        clip: &ClipInner,
        layer_idx: usize,
        layer_ptr: Ptr<AudioLayer>,
        mouse_pos: pierro::Vec2
    ) {
        let Some(assets) = ui.memory().get_dnd_payload::<AssetList>() else { return; };
        if assets.audio_clips.len() != 1 {
            return;
        }
        let audio_clip_ptr = *assets.audio_clips.iter().next().unwrap();
        let Some(audio_clip) = project.client.get(audio_clip_ptr) else { return; };

        let start_time = mouse_pos.x / TimelinePanel::FRAME_WIDTH / clip.framerate;
        let end_time = start_time + (audio_clip.length as f32) / (audio_clip.format.sample_rate as f32);

        paint_commands.audio_bars.push(AudioInstanceBar {
            layer_idx,
            start: start_time,
            end: end_time,
            volume_previews: Vec::new()
        });

        if ui.input().l_mouse.released() {
            ui.memory().clear_dnd_payload();

            project.client.queue_action(Action::single(editor.action_context("Add audio instance"), CreateAudioInstance {
                ptr: project.client.next_ptr(),
                layer: layer_ptr,
                clip: audio_clip_ptr,
                start: start_time,
                end: end_time
            }));            
        }
    }

    fn mouse_interaction(
        &mut self,
        ui: &mut pierro::UI,
        project: &ProjectState,
        editor: &mut EditorState,
        frame_area: &pierro::Response,
        paint_commands: &mut PaintCommands,
        clip: &ClipInner,
        layer_idx: usize,
        layer_ptr: Ptr<AudioLayer>,
        mouse_pos: pierro::Vec2
    ) {

        if frame_area.dnd_hovered {
            self.handle_dnd(ui, project, editor, frame_area, paint_commands, clip, layer_idx, layer_ptr, mouse_pos);
        }
        
    }

    fn get_volume_previews(&mut self, project: &ProjectState, editor: &mut EditorState, audio: &AudioInstance) -> Vec<(pierro::Range, Arc<SampleBlock>)> {
        let Some(clip) = project.client.get(audio.clip) else { return Vec::new(); };

        let mut previews = Vec::new();
        let mut t = 0;
        for (block_size, block_ptr) in clip.blocks.iter() {
            let range = pierro::Range::min_size(
                (t as f32) / (clip.format.sample_rate as f32),
                (*block_size as f32) / (clip.format.sample_rate as f32)
            );
            t += *block_size;

            let Some(block) = project.client.get(*block_ptr) else { continue; };
            let sample_block = editor.audio_cache.get_samples(clip.format, *block_ptr, block);
            previews.push((range, sample_block));
        }

        previews
    }

    pub(super) fn render_audio_layer_contents(&mut self,
        ui: &mut pierro::UI,
        project: &ProjectState,
        editor: &mut EditorState,
        frame_area: &pierro::Response,
        paint_commands: &mut PaintCommands,
        clip: &ClipInner,
        layer_idx: usize,
        layer: &AudioLayer,
        layer_ptr: Ptr<AudioLayer>
    ) {

        for audio_ptr in layer.audio_instances.iter() {
            let Some(audio) = project.client.get(audio_ptr.ptr()) else { continue; };

            paint_commands.audio_bars.push(AudioInstanceBar {
                layer_idx,
                start: audio.start,
                end: audio.end,
                volume_previews: self.get_volume_previews(project, editor, audio)
            });

        }

        let mouse_over_bar = false;

        let layer_rect = pierro::Rect::min_size(
            pierro::Vec2::Y * TimelinePanel::LAYER_HEIGHT * (layer_idx as f32),
            pierro::vec2(f32::INFINITY, TimelinePanel::LAYER_HEIGHT)
        );
        if let Some(mouse_pos) = frame_area.mouse_pos(ui) {
            if layer_rect.contains(mouse_pos) && !mouse_over_bar {
                self.mouse_interaction(ui, project, editor, frame_area, paint_commands, clip, layer_idx, layer_ptr, mouse_pos);
            }
        }
         
    }

}
