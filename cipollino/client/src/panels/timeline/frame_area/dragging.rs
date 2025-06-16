
use std::f32;

use super::{FrameArea, DragState};
use crate::{EditorState, LayerRenderList, ProjectState, RenderLayerKind, TimelinePanel};
use alisa::Ptr;
use project::{Action, AudioInstance, AudioLayer, ClipInner, Frame, Layer, SetAudioInstanceBounds, SetAudioInstanceOffset, SetFrameTime};

impl FrameArea {

    // TODO: this is kinda code duplication - basically the same calculations get done to check if something on a timeline needs to
    // be highlighted during box-selection. A rework would be nice :)
    fn box_select_layer(&mut self, project: &ProjectState, editor: &mut EditorState, layer: &Layer, x_range: pierro::Range) {
        for frame_ptr in layer.frames.iter() {
            if let Some(frame) = project.client.get(frame_ptr.ptr()) {
                let frame_x_range = pierro::Range::min_size((frame.time as f32) * TimelinePanel::FRAME_WIDTH, TimelinePanel::FRAME_WIDTH);
                if frame_x_range.intersects(x_range) {
                    editor.selection.select(frame_ptr.ptr());
                }
            }
        }
    }

    fn box_select_audio_layer(&mut self, project: &ProjectState, editor: &mut EditorState, clip: &ClipInner, layer: &AudioLayer, x_range: pierro::Range) {
        for audio_ptr in layer.audio_instances.iter() {
            if let Some(audio) = project.client.get(audio_ptr.ptr()) {
                let audio_x_range = pierro::Range::min_size(
                    audio.start * clip.framerate * TimelinePanel::FRAME_WIDTH,
                    audio.length() * clip.framerate * TimelinePanel::FRAME_WIDTH
                );
                if audio_x_range.intersects(x_range) {
                    editor.selection.select(audio_ptr.ptr());
                }
            }
        }
    }

    fn box_select(&mut self, project: &ProjectState, editor: &mut EditorState, clip: &ClipInner, render_list: &LayerRenderList, from: pierro::Vec2, to: pierro::Vec2) {
        let min = from.min(to);
        let max = from.max(to);
        
        let x_range = pierro::Range::new(min.x, max.x);
        let top_layer_idx = ((min.y / TimelinePanel::LAYER_HEIGHT).floor() as i32).max(0);
        let bottom_layer_idx = ((max.y / TimelinePanel::LAYER_HEIGHT).floor() as i32).max(0);
        for layer_idx in top_layer_idx..=bottom_layer_idx {
            if layer_idx as usize >= render_list.len() {
                break;
            }
            let layer = &render_list.layers[layer_idx as usize];
            match &layer.kind {
                RenderLayerKind::Layer(layer_ptr, layer) => {
                    if !editor.locked_layers.contains(layer_ptr) {
                        self.box_select_layer(project, editor, layer, x_range);
                    }
                },
                RenderLayerKind::AudioLayer(_, layer) => {
                    self.box_select_audio_layer(project, editor, clip, layer, x_range);
                },
                RenderLayerKind::LayerGroup(_, _) => {}
            }
        }
    }

    fn move_selected_frames(project: &ProjectState, editor: &EditorState, drag: f32, action: &mut Action) {
        let frame_offset = FrameArea::drag_to_frame_offset(drag);
        let mut selected_frames = Vec::new();
        for frame_ptr in editor.selection.iter() {
            if let Some(frame) = project.client.get::<Frame>(frame_ptr) {
                selected_frames.push((frame.time, frame_ptr));
            }
        }
        selected_frames.sort_by_key(|(time, _)| *time);
        if frame_offset > 0 {
            selected_frames.reverse();
        }
        for (time, frame) in selected_frames {
            action.push(SetFrameTime {
                frame,
                new_time: time + frame_offset,
                ..Default::default()
            });
        }
    }

    pub(super) fn calc_audio_move_bounds(project: &ProjectState, editor: &EditorState) -> (f32, f32) {
        let mut min = -f32::INFINITY;
        let mut max =  f32::INFINITY;
        for audio in editor.selection.iter::<AudioInstance>() {
            let Some(audio) = project.client.get(audio) else { continue; };
            let Some(layer) = project.client.get(audio.layer) else { continue; };
            min = min.max(-audio.start);

            for other_audio in layer.audio_instances.iter() {
                if editor.selection.selected(other_audio.ptr()) {
                    continue;
                } 
                let Some(other_audio) = project.client.get(other_audio.ptr()) else { continue; };
                if other_audio.end <= audio.start {
                    min = min.max(other_audio.end - audio.start);
                }
                if other_audio.start >= audio.end {
                    max = max.min(other_audio.start - audio.end);
                }
            }
        }
        (min, max)
    }

    pub(super) fn calc_audio_move_offset(editor: &EditorState, clip: &ClipInner, offset: f32) -> f32 {
        let mut audio_offset = Self::pixels_to_seconds(clip, offset);
        if editor.selection.contains::<Frame>() {
            // Quantize audio offset if necessary
            audio_offset = (audio_offset / clip.frame_len()).round() as f32 * clip.frame_len();
        }
        audio_offset
    }

    fn move_selected_audio(project: &ProjectState, editor: &EditorState, clip: &ClipInner, drag: f32, action: &mut Action) {
        let audio_offset = Self::calc_audio_move_offset(editor, clip, drag); 

        let (min_offset, max_offset) = Self::calc_audio_move_bounds(project, editor);
        let audio_offset = audio_offset.clamp(min_offset, max_offset);

        let mut audios_to_move = Vec::new();
        for audio_ptr in editor.selection.iter::<AudioInstance>() {
            let Some(audio) = project.client.get(audio_ptr) else { continue; }; 
            audios_to_move.push((audio_ptr, audio));
        }

        audios_to_move.sort_by(|(_, a), (_, b)| a.start.total_cmp(&b.start));
        if audio_offset > 0.0 {
            audios_to_move.reverse();
        }

        for (audio_ptr, audio) in audios_to_move {
            action.push(SetAudioInstanceBounds {
                ptr: audio_ptr,
                start: audio.start + audio_offset,
                end: audio.end + audio_offset
            });
        }
    }

    fn move_selected(project: &ProjectState, editor: &EditorState, clip: &ClipInner, drag: f32) {
        let mut action = Action::new(editor.action_context("Move Frames"));

        Self::move_selected_frames(project, editor, drag, &mut action);
        Self::move_selected_audio(project, editor, clip, drag, &mut action);
         
        project.client.queue_action(action);

    }

    pub(super) fn get_audio_trim_start_bounds(project: &ProjectState, clip: &ClipInner, audio_ptr: Ptr<AudioInstance>, offset: f32) -> Option<(f32, f32)> {
        let audio = project.client.get(audio_ptr)?;
        let time_offset = Self::pixels_to_seconds(clip, offset); 
        let mut start = (audio.start + time_offset).min(audio.end - 0.01).max(audio.start - audio.offset);

        // Resolve collisions
        let layer = project.client.get(audio.layer)?;
        for other_audio in layer.audio_instances.iter() {
            if other_audio.ptr() == audio_ptr {
                continue;
            }
            let Some(other_audio) = project.client.get(other_audio.ptr()) else { continue; };
            if other_audio.start < audio.start {
                start = start.max(other_audio.end);
            }
        }

        Some((start, audio.end))
    }

    fn trim_audio_start(project: &ProjectState, editor: &mut EditorState, clip: &ClipInner, offset: f32) {
        let mut action = Action::new(editor.action_context("Trim Audio Start"));

        for audio_ptr in editor.selection.iter::<AudioInstance>() {
            let Some((start, end)) = Self::get_audio_trim_start_bounds(project, clip, audio_ptr, offset) else { continue; };
            let Some(audio) = project.client.get(audio_ptr) else { continue; };
            action.push(SetAudioInstanceBounds {
                ptr: audio_ptr,
                start,
                end 
            });
            action.push(SetAudioInstanceOffset {
                ptr: audio_ptr,
                offset_value: audio.offset + start - audio.start,
            })
        }

        project.client.queue_action(action);
    }

    pub(super) fn get_audio_trim_end_bounds(project: &ProjectState, clip: &ClipInner, audio_ptr: Ptr<AudioInstance>, offset: f32) -> Option<(f32, f32)> {
        let audio = project.client.get(audio_ptr)?;
        let time_offset = Self::pixels_to_seconds(clip, offset); 
        let mut end = (audio.end + time_offset).max(audio.start + 0.01);

        // Resolve collisions
        let layer = project.client.get(audio.layer)?;
        for other_audio in layer.audio_instances.iter() {
            if other_audio.ptr() == audio_ptr {
                continue;
            }
            let Some(other_audio) = project.client.get(other_audio.ptr()) else { continue; };
            if other_audio.start > audio.start {
                end = end.min(other_audio.start);
            }
        }

        Some((audio.start, end))
    }

    fn trim_audio_end(project: &ProjectState, editor: &mut EditorState, clip: &ClipInner, offset: f32) {
        let mut action = Action::new(editor.action_context("Trim Audio End"));

        for audio_ptr in editor.selection.iter::<AudioInstance>() {
            let Some((start, end)) = Self::get_audio_trim_end_bounds(project, clip, audio_ptr, offset) else { continue; };
            action.push(SetAudioInstanceBounds {
                ptr: audio_ptr,
                start,
                end 
            });
        }

        project.client.queue_action(action);
    }

    pub(super) fn drag_stopped(&mut self, project: &ProjectState, editor: &mut EditorState, clip: &ClipInner, render_list: &LayerRenderList) {
        match std::mem::replace(&mut self.drag_state, DragState::None) {
            DragState::None => {},
            DragState::Move { offset } => {
                Self::move_selected(project, editor, clip, offset);
            },
            DragState::BoxSelect { from, to } => {
                self.box_select(project, editor, clip, render_list, from, to);  
            },
            DragState::AudioTrimStart { offset } => {
                Self::trim_audio_start(project, editor, clip, offset);
            },
            DragState::AudioTrimEnd { offset } => {
                Self::trim_audio_end(project, editor, clip, offset);
            }
        }
    }
    
}
