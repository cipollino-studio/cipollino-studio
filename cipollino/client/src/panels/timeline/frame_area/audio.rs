
use std::sync::Arc;

use alisa::Ptr;
use project::{Action, AudioInstance, AudioLayer, ClipInner, CreateAudioInstance};

use crate::{panels::timeline::frame_area::{DragState, PaintCommands}, AssetList, EditorState, ProjectState, SampleBlock, TimelinePanel};

use super::FrameArea;

pub(super) struct AudioInstanceBar {
    pub layer_idx: usize, 
    // The start/end times are in seconds
    pub start: f32,
    pub end: f32,
    pub offset: f32,
    pub selected: bool,

    pub volume_previews: Vec<(pierro::Range, Arc<SampleBlock>)>
}

impl AudioInstanceBar {

    pub fn paint(self, painter: &mut pierro::Painter, rect: pierro::Rect, framerate: f32, accent_color: pierro::Color) {
        let rect = pierro::Rect::min_size(
            rect.tl() + pierro::vec2(self.start * framerate * TimelinePanel::FRAME_WIDTH, (self.layer_idx as f32) * TimelinePanel::LAYER_HEIGHT),
            pierro::vec2((self.end - self.start) * framerate * TimelinePanel::FRAME_WIDTH, TimelinePanel::LAYER_HEIGHT)
        );

        let bg_color = accent_color.darken(0.25);
        let rounding = pierro::Rounding::same(5.0);
        painter.rect(pierro::PaintRect::new(rect, bg_color).with_rounding(rounding));

        let volume_color = accent_color.darken(0.6);
        for (range, samples) in self.volume_previews {
            if samples.volume.is_empty() {
                continue;
            }
            let x_range = (range.shift(-self.offset) * framerate * TimelinePanel::FRAME_WIDTH).shift(rect.left());
            let render_range = pierro::Range::new(
                x_range.min,
                x_range.max.min(rect.right())
            );
            for x in ((render_range.min.floor() as i32)..(render_range.max.ceil() as i32)).step_by(2) {
                let t = (x as f32 - x_range.min) / x_range.size(); 
                let idx = ((t * (samples.volume.len() as f32)).round() as i32).clamp(0, samples.volume.len() as i32 - 1) as usize;
                let volume = samples.volume[idx].powf(0.6) * 0.97 + 0.03;
                let volume_sample_rect = pierro::Rect::center_size(
                    pierro::vec2(x as f32 + 1.0, rect.center().y),
                    pierro::vec2(2.0, rect.height() * volume)
                ).intersect(rect);
                painter.rect(pierro::PaintRect::new(volume_sample_rect, volume_color));
            }
        }

        if self.selected {
            painter.rect(
                pierro::PaintRect::new(rect, pierro::Color::TRANSPARENT)
                    .with_rounding(rounding)
                    .with_stroke(pierro::Stroke::new(accent_color, 2.0))
            );
        }
    }

}

impl FrameArea {

    pub(super) fn pixels_to_seconds(clip: &ClipInner, pixels: f32) -> f32 {
        pixels / clip.framerate / TimelinePanel::FRAME_WIDTH 
    }

    pub(super) fn seconds_to_pixels(clip: &ClipInner, time: f32) -> f32 {
        time * clip.framerate * TimelinePanel::FRAME_WIDTH
    }

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

        let start_time = Self::pixels_to_seconds(clip, mouse_pos.x);
        let end_time = start_time + (audio_clip.length as f32) / (audio_clip.format.sample_rate as f32);

        paint_commands.audio_bars.push(AudioInstanceBar {
            layer_idx,
            start: start_time,
            end: end_time,
            offset: 0.0,
            selected: false,
            volume_previews: Vec::new()
        });

        if ui.input().l_mouse.released() {
            ui.memory().clear_dnd_payload();

            project.client.queue_action(Action::single(editor.action_context("Add audio instance"), CreateAudioInstance {
                ptr: project.client.next_ptr(),
                layer: layer_ptr,
                clip: audio_clip_ptr,
                start: start_time,
                end: end_time,
                offset: 0.0
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

        let mut mouse_over_bar = false;
        let (move_offset_min, move_offset_max) = Self::calc_audio_move_bounds(project, editor);

        for audio_ptr in layer.audio_instances.iter() {
            let Some(audio) = project.client.get(audio_ptr.ptr()) else { continue; };

            let audio_interaction_rect = pierro::Rect::min_size(
                pierro::vec2(Self::seconds_to_pixels(clip, audio.start), (layer_idx as f32) * TimelinePanel::LAYER_HEIGHT),
                pierro::vec2(Self::seconds_to_pixels(clip, audio.end - audio.start), TimelinePanel::LAYER_HEIGHT)
            );

            let trim_rect_width = (audio_interaction_rect.width() / 2.0).min(10.0);
            let trim_start_rect = pierro::Rect::min_size(
                audio_interaction_rect.tl(),
                pierro::vec2(trim_rect_width, TimelinePanel::LAYER_HEIGHT)
            );
            let trim_end_rect = pierro::Rect::min_size(
                audio_interaction_rect.tr() - pierro::vec2(trim_rect_width, 0.0),
                pierro::vec2(trim_rect_width, TimelinePanel::LAYER_HEIGHT)
            );

            let selected = editor.selection.selected(audio_ptr.ptr());

            let in_selection_rect = if let Some(selection_rect) = self.drag_state.selection_rect() {
                selection_rect.intersects(audio_interaction_rect)
            } else {
                false
            };

            if let Some(mouse_pos) = frame_area.mouse_pos(ui) {
                if audio_interaction_rect.contains(mouse_pos) {
                    mouse_over_bar = true;
                    if frame_area.mouse_clicked() {
                        editor.selection.extend_select(audio_ptr.ptr());
                        frame_area.request_focus(ui);
                    }
                    if frame_area.drag_started() {
                        if !editor.selection.selected(audio_ptr.ptr()) && !editor.selection.shift_down() {
                            editor.selection.clear();
                        }
                        editor.selection.select(audio_ptr.ptr());
                        self.drag_consumed = true;
                        self.drag_state = if trim_start_rect.contains(mouse_pos) {
                            DragState::AudioTrimStart { offset: 0.0 }
                        } else if trim_end_rect.contains(mouse_pos) {
                            DragState::AudioTrimEnd { offset: 0.0 }
                        } else {
                            DragState::Move { offset: 0.0 }
                        };
                        frame_area.request_focus(ui);
                    }

                    if trim_end_rect.contains(mouse_pos) {
                        ui.set_cursor(pierro::CursorIcon::EResize);
                    }
                    if trim_start_rect.contains(mouse_pos) {
                        ui.set_cursor(pierro::CursorIcon::WResize);
                    }
                }
            }

            let start = match (&self.drag_state, editor.selection.selected(audio_ptr.ptr())) {
                (DragState::AudioTrimStart { offset }, true) => {
                    Self::get_audio_trim_start_bounds(project, clip, audio_ptr.ptr(), *offset).map(|(start, _)| start).unwrap_or(audio.start + Self::pixels_to_seconds(clip, *offset))
                },
                (DragState::Move { offset }, true) => {
                    let offset = Self::calc_audio_move_offset(editor, clip, *offset);
                    audio.start + offset.clamp(move_offset_min, move_offset_max)
                }
                _ => audio.start
            };

            let offset = match (&self.drag_state, editor.selection.selected(audio_ptr.ptr())) {
                (DragState::AudioTrimStart { .. }, true) => audio.offset + start - audio.start, 
                _ => audio.offset 
            };

            let end = match (&self.drag_state, editor.selection.selected(audio_ptr.ptr())) {
                (DragState::AudioTrimEnd { offset }, true) => {
                    Self::get_audio_trim_end_bounds(project, clip, audio_ptr.ptr(), *offset).map(|(_, end)| end).unwrap_or(audio.end + Self::pixels_to_seconds(clip, *offset))
                },
                (DragState::Move { offset }, true) => {
                    let offset = Self::calc_audio_move_offset(editor, clip, *offset);
                    audio.end + offset.clamp(move_offset_min, move_offset_max)
                },
                _ => audio.end 
            };

            paint_commands.audio_bars.push(AudioInstanceBar {
                layer_idx,
                start,
                end,
                offset,
                selected: selected || in_selection_rect,
                volume_previews: self.get_volume_previews(project, editor, audio)
            });

        }

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
