
use project::{Action, AudioClip, AudioInstance, AudioLayer, Client, ClipInner, CreateAudioInstance, CreateFrame, DeleteFrame, Frame, FrameTreeData, Layer, Ptr};

use crate::{EditorState, LayerRenderList, RenderLayerKind, Selection};

use super::SceneClipboard;

struct AudioInstanceClipboard {
    start_offset: f32,
    end_offset: f32,
    offset: f32,
    clip: Ptr<AudioClip>
}

enum LayerClipboard {
    Layer(Vec<(i32, SceneClipboard)>),
    AudioLayer(Vec<AudioInstanceClipboard>)
}

pub struct TimelineClipboard {
    layers: Vec<(i32, LayerClipboard)>
}

fn collect_frame_scene_clipboard(frame: &Frame, client: &Client) -> SceneClipboard {
    let mut scene = SceneClipboard::new();
    for scene_obj in frame.scene.iter().rev() {
        scene.add_object(scene_obj, client);
    }
    scene
}

impl Selection {

    pub(super) fn collect_timeline_clipboard(&self, client: &Client, editor: &EditorState, clip: &ClipInner, layer_render_list: &LayerRenderList) -> TimelineClipboard {
        let mut layers = Vec::new();

        for frame_ptr in self.iter::<Frame>() {
            let Some(frame) = client.get(frame_ptr) else { continue; };
            let Some(layer_idx) = layer_render_list.iter().position(|layer| layer.any_ptr() == frame.layer.any()) else { continue; };
            let layer_idx = layer_idx as i32;
            let clipboard = collect_frame_scene_clipboard(frame, client);
            
            if let Some((_, layer)) = layers.iter_mut().find(|(idx, _)| *idx == layer_idx) {
                if let LayerClipboard::Layer(frames) = layer {
                    frames.push((frame.time, clipboard));
                }
            } else {
                layers.push((layer_idx, LayerClipboard::Layer(vec![(frame.time, clipboard)])));
            }
        }

        for audio_ptr in self.iter::<AudioInstance>() {
            let Some(audio) = client.get(audio_ptr) else { continue; };
            let Some(layer_idx) = layer_render_list.iter().position(|layer| layer.any_ptr() == audio.layer.any()) else { continue; };
            let layer_idx = layer_idx as i32;
            let clipboard = AudioInstanceClipboard {
                start_offset: audio.start,
                end_offset: audio.end,
                offset: audio.offset,
                clip: audio.clip,
            };

            if let Some((_, layer)) = layers.iter_mut().find(|(idx, _)| *idx == layer_idx) {
                if let LayerClipboard::AudioLayer(audios) = layer {
                    audios.push(clipboard);
                }
            } else {
                layers.push((layer_idx, LayerClipboard::AudioLayer(vec![clipboard])));
            }
        }

        // Shift the layers vertically so the active layer has offset 0 
        let active_layer_idx = layer_render_list.iter().position(|layer| layer.any_ptr() == editor.active_layer.any()).unwrap_or(0) as i32;
        for (layer_idx, _) in &mut layers {
            *layer_idx -= active_layer_idx;
        }

        // Shift horizontally so the leftmost frame has offset 0
        let mut min_frame_time = i32::MAX;
        for (_, layer) in &layers {
            match layer {
                LayerClipboard::Layer(frames) => {
                    for (time, _) in frames {
                        min_frame_time = min_frame_time.min(*time);
                    }
                },
                LayerClipboard::AudioLayer(audios) => {
                    for audio in audios {
                        let time = (audio.start_offset / clip.frame_len()).round() as i32;
                        min_frame_time = min_frame_time.min(time);
                    }
                },
            }
        }

        for (_, layer) in &mut layers {
            match layer {
                LayerClipboard::Layer(frames) => {
                    for (time, _) in frames {
                        *time -= min_frame_time;
                    }
                },
                LayerClipboard::AudioLayer(audios) => {
                    for audio in audios {
                        audio.start_offset -= (min_frame_time as f32) * clip.frame_len();
                        audio.end_offset -= (min_frame_time as f32) * clip.frame_len();
                    }
                }
            }
        }

        TimelineClipboard {
            layers
        }
    }

}

fn paste_frame(client: &Client, action: &mut Action, layer_ptr: Ptr<Layer>, layer: &Layer, time: i32, frame_data: &SceneClipboard) -> Ptr<Frame> {
    if let Some(existing_frame) = layer.frame_exactly_at(client, time) {
        action.push(DeleteFrame {
            ptr: existing_frame,
        });
    } 
    let ptr = client.next_ptr();
    action.push(CreateFrame {
        ptr,
        layer: layer_ptr,
        data: FrameTreeData {
            time,
            scene: alisa::ChildListTreeData {
                children: frame_data.objects.iter().rev().map(|obj| obj.tree_data(client.next_key())).collect(),
            },
        },
    });
    ptr
}

fn paste_audio_instance(client: &Client, action: &mut Action, layer_ptr: Ptr<AudioLayer>, cursor_time: f32, audio_data: &AudioInstanceClipboard) -> Ptr<AudioInstance> {
    let ptr = client.next_ptr();
    action.push(CreateAudioInstance {
        ptr,
        layer: layer_ptr,
        clip: audio_data.clip,
        start: audio_data.start_offset + cursor_time,
        end: audio_data.end_offset + cursor_time,
        offset: audio_data.offset
    });
    ptr
}

impl TimelineClipboard {
    
    pub(super) fn paste(&self, client: &Client, editor: &EditorState, clip: &ClipInner, layer_render_list: &LayerRenderList) -> Option<Selection> {
        let mut action = Action::new(editor.action_context("Paste Frames"));
        let cursor_layer = layer_render_list.iter().position(|layer| layer.any_ptr() == editor.active_layer.any())? as i32;
        let cursor_frame = clip.frame_idx(editor.time);
        let mut selection = Selection::new();

        for (layer_offset, clipboard_layer) in &self.layers {
            let layer_idx = cursor_layer + layer_offset;
            if layer_idx < 0 {
                continue;
            }
            let Some(layer) = layer_render_list.layers.get(layer_idx as usize) else { continue; };
            match (&layer.kind, clipboard_layer) {
                (RenderLayerKind::Layer(layer_ptr, layer), LayerClipboard::Layer(frames)) => {
                    if editor.locked_layers.contains(layer_ptr) {
                        continue;
                    }
                    for (time_offset, frame_data) in frames {
                        let time = cursor_frame + *time_offset;
                        selection.select(paste_frame(client, &mut action, *layer_ptr, layer, time, frame_data));
                    }
                },
                (RenderLayerKind::AudioLayer(layer_ptr, _), LayerClipboard::AudioLayer(audios)) => {
                    for audio in audios {
                        selection.select(paste_audio_instance(client, &mut action, *layer_ptr, (cursor_frame as f32) * clip.frame_len(), audio));
                    }
                },
                _ => {}
            }
        } 

        client.queue_action(action); 
        Some(selection)
    }

}
