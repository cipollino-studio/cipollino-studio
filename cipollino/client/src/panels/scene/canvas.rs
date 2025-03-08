
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
            ui.set_sense_scroll(response.node_ref, true);

            // Focus
            if response.mouse_pressed() {
                response.request_focus(ui);
            }
            if response.mouse_released() {
                response.release_focus(ui);
            }

            let camera = malvina::Camera::new(self.cam_pos.x, self.cam_pos.y, ui.scale_factor() / self.cam_size);

            // Calculate the world-space mouse position
            let resolution = texture.size();
            let canvas = ui.get_node_id(ui.curr_parent());
            let canvas_size = ui.memory().get::<pierro::LayoutInfo>(canvas).screen_rect.size();
            let offset = ((resolution / ui.scale_factor()) - canvas_size) / 2.0;
            let mouse_pos = response.mouse_pos(ui)
                .map(|pos| (pos + offset) * ui.scale_factor())
                .map(|pos| pierro::vec2(pos.x, resolution.y - pos.y))
                .map(|pos| camera.screen_to_world(malvina::vec2(pos.x, pos.y), malvina::vec2(resolution.x, resolution.y))); 

            // Zoom
            if let Some(mouse_pos) = mouse_pos {
                let zoom_fac = (1.05 as f32).powf(-response.scroll.y.clamp(-4.0, 4.0) * 0.7); 
                let next_cam_size = (self.cam_size * zoom_fac).clamp(0.05, 20.0);
                let zoom_fac = next_cam_size / self.cam_size;
                self.cam_pos -= (mouse_pos - self.cam_pos) * (zoom_fac - 1.0); 
                self.cam_size = next_cam_size;
            }

            // Panning
            let panning = ui.input().key_down(pierro::Key::COMMAND);
            if panning && response.dragging() {
                let drag_delta = response.drag_delta(ui);
                let drag_delta = malvina::vec2(-drag_delta.x, drag_delta.y) * self.cam_size;
                self.cam_pos += drag_delta;
            }

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
                if response.drag_started() && !panning {
                    tool.mouse_drag_started(&mut tool_context, mouse_pos);
                }
                if response.dragging() && !panning {
                    tool.mouse_dragged(&mut tool_context, mouse_pos);
                }
                if response.drag_stopped() && !panning {
                    tool.mouse_drag_stopped(&mut tool_context, mouse_pos);
                }
            }
            let tool_cursor_icon = tool.cursor_icon();
            let clear_stroke_preview = tool_context.clear_stroke_preview;

            // Recalculate the camera
            let camera = malvina::Camera::new(self.cam_pos.x, self.cam_pos.y, ui.scale_factor() / self.cam_size);

            // Render the scene
            renderer.render(ui.wgpu_device(), ui.wgpu_queue(), texture.texture(), camera, |rndr| {
                self.render_layer_list(rndr, &project.client, &editor, clip, &clip.layers); 
            });

            if response.hovered {
                let cursor = if panning {
                    if response.mouse_down() {
                        pierro::CursorIcon::Grabbing
                    } else {
                        pierro::CursorIcon::Grab
                    }
                } else {
                    tool_cursor_icon
                };
                ui.set_cursor(cursor);
            }

            if clear_stroke_preview {
                editor.stroke_preview = None;
            }
        });
    }

}
