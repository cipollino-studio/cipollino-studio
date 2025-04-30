
use project::{Action, Client, CreateFrame, DeleteFrame, Frame, FrameTreeData, Layer, Ptr};

use crate::{EditorState, LayerRenderList, RenderLayerKind, Selection};

use super::SceneClipboard;

pub enum LayerClipboard {
    Layer(Vec<(i32, SceneClipboard)>)
}

pub struct TimelineClipboard {
    pub layers: Vec<(i32, LayerClipboard)>
}

fn collect_frame_scene_clipboard(frame: &Frame, client: &Client) -> SceneClipboard {
    let mut scene = SceneClipboard::new();
    for scene_obj in frame.scene.iter().rev() {
        scene.add_object(scene_obj, client);
    }
    scene
}

impl Selection {

    pub(super) fn collect_timeline_clipboard(&self, client: &Client, editor: &EditorState, layer_render_list: &LayerRenderList) -> TimelineClipboard {
        let mut layers = Vec::new();

        for frame_ptr in self.iter::<Frame>() {
            let Some(frame) = client.get(frame_ptr) else { continue; };
            let Some(layer_idx) = layer_render_list.iter().position(|layer| layer.any_ptr() == frame.layer.any()) else { continue; };
            let layer_idx = layer_idx as i32;
            let clipboard = collect_frame_scene_clipboard(frame, client);
            
            if let Some((_, layer)) = layers.iter_mut().find(|(idx, _)| *idx == layer_idx) {
                let LayerClipboard::Layer(frames) = layer;
                frames.push((frame.time, clipboard));
            } else {
                layers.push((layer_idx, LayerClipboard::Layer(vec![(frame.time, clipboard)])));
            }
        }

        // Shift the layers vertically so the active layer has offset 0 
        let active_layer_idx = layer_render_list.iter().position(|layer| layer.any_ptr() == editor.active_layer.any()).unwrap_or(0) as i32;
        for (layer_idx, _) in &mut layers {
            *layer_idx -= active_layer_idx;
        }

        // Shift the frames horizontally so the left one has time 0
        let min_frame_time = layers.iter().filter_map(|(_, layer)| match layer {
            LayerClipboard::Layer(frames) => Some(frames),
        }).map(|frames| frames.iter().map(|(time, _)| *time)).flatten().min().unwrap_or(0);
        for (_, layer) in &mut layers {
            match layer {
                LayerClipboard::Layer(frames) => {
                    for (time, _) in frames {
                        *time -= min_frame_time;
                    }
                },
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
                children: frame_data.objects.iter().map(|obj| obj.tree_data(client.next_key())).collect(),
            },
        },
    });
    ptr
}

impl TimelineClipboard {
    
    pub(super) fn paste(&self, client: &Client, editor: &EditorState, layer_render_list: &LayerRenderList) -> Option<Selection> {
        let mut action = Action::new(editor.action_context("Paste Frames"));
        let cursor_layer = layer_render_list.iter().position(|layer| layer.any_ptr() == editor.active_layer.any())? as i32;
        let clip = client.get(editor.open_clip)?;
        let clip = client.get(clip.inner)?;
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
                _ => {}
            }
        } 

        client.queue_action(action); 
        Some(selection)
    }

}
