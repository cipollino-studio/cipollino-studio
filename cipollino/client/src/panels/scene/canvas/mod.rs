
use std::collections::HashSet;

use project::{ClipInner, Ptr, SceneChildPtr, Stroke};

use crate::{render_scene, EditorState, ProjectState, ToolContext};

use super::ScenePanel;

mod util;
mod picking;
mod selection;
mod onion_skin;

impl ScenePanel {

    fn calc_camera(&self, scale_factor: f32) -> malvina::Camera {
        malvina::Camera::new(self.cam_pos.x, self.cam_pos.y, scale_factor / self.cam_size)
    }

    fn canvas_contents(&mut self,
        ui: &mut pierro::UI,
        texture: &pierro::Texture,
        response: &pierro::Response,
        project: &ProjectState,
        editor: &mut EditorState,
        renderer: &mut malvina::Renderer,
        clip: &ClipInner,
        render_list: &Vec<SceneChildPtr>,
        rendered_strokes: &HashSet<Ptr<Stroke>>,
        canvas_width: u32,
        canvas_height: u32,
        resize_margin: u32
    ) {
        ui.set_sense_mouse(response.node_ref, true);
        ui.set_sense_scroll(response.node_ref, true);

        // Focus
        if response.mouse_pressed() {
            response.request_focus(ui);
        }
        if response.mouse_released() {
            response.release_focus(ui);
        }

        let camera = self.calc_camera(ui.scale_factor());

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
        let panning = ui.input().key_modifiers.contains(pierro::KeyModifiers::CONTROL);
        if panning && response.dragging() {
            let drag_delta = response.drag_delta(ui);
            let drag_delta = malvina::vec2(-drag_delta.x, drag_delta.y) * self.cam_size;
            self.cam_pos += drag_delta;
            editor.selection.keep_selection();
        }

        // Use the current tool
        let tool = editor.curr_tool.clone();
        let mut tool = tool.borrow_mut();
        let mut picking_buffer = self.picking_buffer.borrow_mut();
        let mut tool_context = ToolContext {
            project,
            clip,
            active_layer: editor.active_layer,
            frame_time: clip.frame_idx(editor.time),

            rendered_strokes,

            picking_buffer: &mut picking_buffer,
            picking_mouse_pos: response.mouse_pos(ui)
                .filter(|pos| pos.x > 0.0 && pos.y > 0.0)
                .filter(|pos| pos.x < canvas_width as f32 - 1.0 && pos.y < canvas_height as f32 - 1.0 )
                .map(|pos| (pos.x.floor() as u32 + resize_margin / 2, pos.y.floor() as u32 + resize_margin / 2)),

            device: ui.wgpu_device(),
            queue: ui.wgpu_queue(),

            editor,

            clear_stroke_preview: false,
            pressure: ui.input().pressure
        };
        let mut pause = false;
        if let Some(mouse_pos) = mouse_pos {
            if response.mouse_clicked() && !panning {
                pause = true;
                tool.mouse_clicked(&mut tool_context, mouse_pos);
            }
            if response.mouse_pressed() && !panning {
                pause = true;
                tool.mouse_pressed(&mut tool_context, mouse_pos);
            }
            if response.mouse_released() && !panning {
                pause = true;
                tool.mouse_released(&mut tool_context, mouse_pos);
            }
            if response.drag_started() && !panning {
                pause = true;
                tool.mouse_drag_started(&mut tool_context, mouse_pos);
            }
            if response.dragging() && !panning {
                pause = true;
                tool.mouse_dragged(&mut tool_context, mouse_pos);
            }
            if response.drag_stopped() && !panning {
                pause = true;
                tool.mouse_drag_stopped(&mut tool_context, mouse_pos);
            }
        }
        let tool_cursor_icon = tool.cursor_icon();
        let clear_stroke_preview = tool_context.clear_stroke_preview;

        if pause {
            editor.playing = false;
        }
        
        drop(picking_buffer); // We mutably borrow self later on, so we need to drop this here

        // Recalculate the camera
        // We need to recalculate it because the user might have panned/zoomed this frame
        let camera = self.calc_camera(ui.scale_factor());

        // Render the scene
        let accent_color = ui.style::<pierro::theme::AccentColor>();
        renderer.render(ui.wgpu_device(), ui.wgpu_queue(), texture.texture(), camera, elic::Color::WHITE, ui.scale_factor(), |rndr| {
            if editor.show_onion_skin {
                self.render_onion_skin(rndr, &project.client, &editor, clip);
            }
            render_scene(rndr, &project.client, editor, clip, clip.frame_idx(editor.time));
            self.render_selection(rndr, &editor, &project.client, render_list);

            tool.render_overlay(rndr, accent_color);

            rndr.render_canvas_border(malvina::vec2(clip.width as f32, clip.height as f32));
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
    }

    pub(super) fn canvas(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, renderer: &mut malvina::Renderer, clip: &ClipInner) {

        // Get the list of things to render in the scene
        let render_list = Self::render_list(&project.client, clip, clip.frame_idx(editor.time));
        let rendered_strokes = Self::rendered_strokes(&render_list);

        let canvas_container = ui.node(pierro::UINodeParams::new(pierro::Size::fr(1.0), pierro::Size::fr(1.0)));

        // Render the picking buffer
        let resize_margin = 50;
        let canvas_size = ui.memory().get::<pierro::LayoutInfo>(canvas_container.id).screen_rect.size();
        let canvas_width = canvas_size.x.ceil() as u32 + resize_margin;
        let canvas_height = canvas_size.y.ceil() as u32 + resize_margin;
        self.picking_buffer.borrow_mut().update_texture(ui.wgpu_device(), canvas_width, canvas_height);
        renderer.render_picking(ui.wgpu_device(), ui.wgpu_queue(), &self.picking_buffer.clone().borrow(), self.calc_camera(1.0), |rndr| {
            self.render_picking(rndr, editor, &render_list);
        });

        // Render the scene
        ui.with_parent(canvas_container.node_ref, |ui| {
            pierro::canvas(ui, (resize_margin as f32 * ui.scale_factor()) as u32, |ui, texture, response| {
                self.canvas_contents(ui, texture, response, project, editor, renderer, clip, &render_list, &rendered_strokes, canvas_width, canvas_height, resize_margin); 
            });
        });
        
    }

}
