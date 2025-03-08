
use project::{Client, ClipInner, Frame, Layer, LayerChildList, LayerChildPtr, Ptr, SceneChildPtr, Stroke};

use crate::{EditorState, ProjectState, ToolContext};

use super::ScenePanel;

impl ScenePanel {

    fn render_stroke(&mut self, rndr: &mut malvina::LayerRenderer, _client: &Client, stroke: &Stroke) {
        let stroke_mesh = malvina::StrokeMesh::new(rndr.device(), &stroke.stroke.0);
        rndr.render_stroke(&stroke_mesh);
    }

    fn render_frame(&mut self, rndr: &mut malvina::LayerRenderer, client: &Client, frame: &Frame) {
        for scene_child in frame.scene.iter() {
            match scene_child {
                SceneChildPtr::Stroke(stroke_ptr) => {
                    if let Some(stroke) = client.get(stroke_ptr.ptr()) {
                        self.render_stroke(rndr, client, stroke);
                    }
                }
            }
        }
    }

    fn render_layer(&mut self, rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner, layer: &Layer, layer_ptr: Ptr<Layer>) {
        let Some(frame_ptr) = layer.frame_at(client, clip.frame_idx(editor.time)) else { return; };
        if let Some(frame) = client.get(frame_ptr) {
            self.render_frame(rndr, client, frame);
        }

        if layer_ptr == editor.active_layer {
            if let Some(stroke_preview) = &editor.stroke_preview {
                rndr.render_stroke(stroke_preview);
            }
        } 
    }

    fn render_layer_list(&mut self, rndr: &mut malvina::LayerRenderer, client: &Client, editor: &EditorState, clip: &ClipInner, layer_list: &LayerChildList) {
        for layer in layer_list.iter() {
            match layer {
                LayerChildPtr::Layer(layer_ptr) => {
                    if let Some(layer) = client.get(layer_ptr.ptr()) {
                        self.render_layer(rndr, client, editor, clip, layer, layer_ptr.ptr());
                    }
                }
            } 
        }
    }

    pub(super) fn canvas(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, renderer: &mut malvina::Renderer, clip: &ClipInner) {
        pierro::canvas(ui, |ui, texture, response| {
            ui.set_sense_mouse(response.node_ref, true);

            let camera = malvina::Camera::new(0.0, 0.0, ui.scale_factor());

            // Calculate the world-space mouse position
            let resolution = texture.size();
            let canvas = ui.get_node_id(ui.curr_parent());
            let canvas_size = ui.memory().get::<pierro::LayoutInfo>(canvas).screen_rect.size();
            let offset = ((resolution / ui.scale_factor()) - canvas_size) / 2.0;
            let mouse_pos = response.mouse_pos(ui)
                .map(|pos| (pos + offset) * ui.scale_factor())
                .map(|pos| pierro::vec2(pos.x, resolution.y - pos.y))
                .map(|pos| camera.screen_to_world(malvina::vec2(pos.x, pos.y), malvina::vec2(resolution.x, resolution.y))); 

            // Use the current tool
            let tool = editor.curr_tool.clone();
            let mut tool = tool.borrow_mut();
            let mut tool_context = ToolContext {
                project,
                clip,
                active_layer: editor.active_layer,
                frame_time: clip.frame_idx(editor.time),
                editor,
                device: ui.wgpu_device(),
                clear_stroke_preview: false 
            };
            if let Some(mouse_pos) = mouse_pos {
                if response.mouse_clicked() {
                    tool.mouse_clicked(&mut tool_context, mouse_pos);
                }
                if response.mouse_pressed() {
                    tool.mouse_pressed(&mut tool_context, mouse_pos);
                }
                if response.mouse_released() {
                    tool.mouse_released(&mut tool_context, mouse_pos);
                }
                if response.drag_started() {
                    tool.mouse_drag_started(&mut tool_context, mouse_pos);
                }
                if response.dragging() {
                    tool.mouse_dragged(&mut tool_context, mouse_pos);
                }
                if response.drag_stopped() {
                    tool.mouse_drag_stopped(&mut tool_context, mouse_pos);
                }
            }
            let clear_stroke_preview = tool_context.clear_stroke_preview;

            // Render the scene
            renderer.render(ui.wgpu_device(), ui.wgpu_queue(), texture.texture(), camera, |rndr| {
                self.render_layer_list(rndr, &project.client, &editor, clip, &clip.layers); 
            });

            if clear_stroke_preview {
                editor.stroke_preview = None;
            }
        });
    }

}
