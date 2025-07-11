
use std::collections::HashSet;

use project::{ClipInner, Fill, SceneObjPtr, Stroke};

use crate::{presence_color, render_scene, AppSystems, EditorState, ProjectState, RendererState, SceneRenderList, ToolContext};

use super::ScenePanel;

mod picking;
mod selection;
mod onion_skin;

impl ScenePanel {

    fn calc_camera(&self, scale_factor: f32) -> malvina::Camera {
        malvina::Camera::new(self.cam_pos, scale_factor / self.cam_size).mirror(self.mirror)
    }

    fn canvas_space_to_world_space(
        mouse_pos: pierro::Vec2,
        camera: &malvina::Camera,
        scale_factor: f32,
        resolution: pierro::Vec2,
        offset: pierro::Vec2
    ) -> elic::Vec2 {
        let pos = (mouse_pos + offset) * scale_factor;
        let pos = pierro::vec2(pos.x, resolution.y - pos.y);
        let pos = camera.screen_to_world(pos, resolution); 
        pos
    } 

    fn canvas_contents(&mut self,
        ui: &mut pierro::UI,
        texture: &pierro::Texture,
        response: &pierro::Response,
        project: &ProjectState,
        editor: &mut EditorState,
        systems: &mut AppSystems,
        renderer: &mut RendererState,
        clip: &ClipInner,
        render_list: &SceneRenderList,
        modifiable_objs: &HashSet<SceneObjPtr>,
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
        let canvas_mouse_pos = response.mouse_pos(ui); 
        let mouse_pos = canvas_mouse_pos.map(|mouse_pos| Self::canvas_space_to_world_space(mouse_pos, &camera, ui.scale_factor(), resolution, offset));

        // Zoom
        if let Some(mouse_pos) = mouse_pos {
            let zoom_fac = (1.05 as f32).powf(-response.scroll.y.clamp(-4.0, 4.0) * 0.7); 
            let next_cam_size = (self.cam_size * zoom_fac).clamp(0.05, 20.0);
            self.cam_size = next_cam_size;

            let next_cam = self.calc_camera(ui.scale_factor());
            if let Some(canvas_mouse_pos) = canvas_mouse_pos {
                let mapped_mouse_pos = Self::canvas_space_to_world_space(canvas_mouse_pos, &next_cam, ui.scale_factor(), resolution, offset);
                let offset = mapped_mouse_pos - mouse_pos;
                self.cam_pos -= offset;
            }

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
        let accent_color = ui.style::<pierro::theme::AccentColor>();
        let picking_mouse_pos = response.mouse_pos(ui)
            .filter(|pos| pos.x > 0.0 && pos.y > 0.0)
            .filter(|pos| pos.x < canvas_width as f32 - 1.0 && pos.y < canvas_height as f32 - 1.0 )
            .map(|pos| (pos.x.floor() as u32 + resize_margin / 2, pos.y.floor() as u32 + resize_margin / 2));
        let mut tool_context = ToolContext {
            project,
            clip,
            active_layer: editor.active_layer,
            frame_time: clip.frame_idx(editor.time),

            modifiable_objs,

            picking_buffer: &mut picking_buffer,
            render_list,
            picking_mouse_pos,

            device: ui.wgpu_device(),
            queue: ui.wgpu_queue(),
            renderer: Some(renderer),

            systems,

            pressure: ui.input().pressure,
            cam_zoom: 1.0 / self.cam_size,
            key_modifiers: ui.input().key_modifiers
        };
        let mut pause = false;
        let tool_cursor_icon = if let Some(mouse_pos) = mouse_pos {
            if response.mouse_clicked() && !panning {
                pause = true;
                tool.mouse_clicked(editor, &mut tool_context, mouse_pos);
            }
            if response.mouse_pressed() && !panning {
                pause = true;
                tool.mouse_pressed(editor, &mut tool_context, mouse_pos);
            }
            if response.mouse_released() && !panning {
                pause = true;
                tool.mouse_released(editor, &mut tool_context, mouse_pos);
            }
            if response.drag_started() && !panning {
                pause = true;
                tool.mouse_drag_started(editor, &mut tool_context, mouse_pos);
            }
            if response.dragging() && !panning {
                pause = true;
                tool.mouse_dragged(editor, &mut tool_context, mouse_pos);
            }
            if response.drag_stopped() && !panning {
                pause = true;
                tool.mouse_drag_stopped(editor, &mut tool_context, mouse_pos);
            }

            tool.cursor_icon(editor, &mut tool_context, mouse_pos)
        } else {
            pierro::CursorIcon::default()
        };
        tool.tick(editor, &mut tool_context);

        // Recalculate the camera
        // We need to recalculate it because the user might have panned/zoomed this frame
        let camera = self.calc_camera(ui.scale_factor());

        // Render the scene
        let mut tool_context = ToolContext {
            project,
            clip,
            active_layer: editor.active_layer,
            frame_time: clip.frame_idx(editor.time),

            modifiable_objs,

            picking_buffer: &mut picking_buffer,
            render_list,
            picking_mouse_pos,

            device: ui.wgpu_device(),
            queue: ui.wgpu_queue(),
            renderer: None,

            systems,

            pressure: ui.input().pressure,
            cam_zoom: 1.0 / self.cam_size,
            key_modifiers: ui.input().key_modifiers
        };
        renderer.renderer.render(ui.wgpu_device(), ui.wgpu_queue(), texture.texture(), camera, clip.background_color.into(), ui.scale_factor(), |rndr| {
            if editor.show_onion_skin {
                Self::render_onion_skin(rndr, &project.client, &editor, tool_context.systems, clip);
            }
            render_scene(rndr, &renderer.builtin_brushes, &project.client, editor, clip, clip.frame_idx(editor.time), true);
            Self::render_selection(rndr, &renderer.builtin_brushes, &project.client, &editor, render_list);

            tool.render_overlay(&mut tool_context, rndr, accent_color);

            rndr.render_canvas_border(malvina::vec2(clip.width as f32, clip.height as f32));

            // Render the cursors of the other clients on this clip
            for (id, other_client) in &editor.other_clients {
                if let Some(other_client_mouse_pos) = other_client.mouse_pos {
                    if other_client.open_clip == editor.open_clip {
                        let color = presence_color(*id);
                        let pos = elic::vec2(other_client_mouse_pos[0], other_client_mouse_pos[1]);
                        rndr.overlay_circle(pos, 4.0, color);
                    }
                }
            }

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

            editor.presence.set_mouse_pos(mouse_pos);
        } else {
            editor.presence.set_mouse_pos(None);
        }

        if pause {
            editor.playing = false;
        }

    }

    pub(super) fn canvas(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, systems: &mut AppSystems, renderer: &mut RendererState, clip: &ClipInner, render_list: &SceneRenderList) {

        // Get the list of things to render in the scene
        let modifiable_objs: HashSet<SceneObjPtr> = render_list.objs.iter().filter(|obj| {
            let frame_ptr = match **obj {
                project::SceneObjPtr::Stroke(ptr) => {
                    let Some(stroke) = project.client.get(ptr) else { return false; };
                    stroke.frame
                },
                project::SceneObjPtr::Fill(ptr) => {
                    let Some(fill) = project.client.get(ptr) else { return false; };
                    fill.frame
                },
            };
            let Some(frame) = project.client.get(frame_ptr) else { return false };
            editor.can_modify_layer(frame.layer)
        }).copied().collect();

        editor.selection.retain::<Stroke, _>(|stroke| modifiable_objs.contains(&stroke.into()));
        editor.selection.retain::<Fill, _>(|fill| modifiable_objs.contains(&fill.into()));

        let canvas_container = ui.node(pierro::UINodeParams::new(pierro::Size::fr(1.0), pierro::Size::fr(1.0)));

        // Render the picking buffer
        let resize_margin = 50;
        let canvas_size = ui.memory().get::<pierro::LayoutInfo>(canvas_container.id).screen_rect.size();
        let canvas_width = canvas_size.x.ceil() as u32 + resize_margin;
        let canvas_height = canvas_size.y.ceil() as u32 + resize_margin;
        self.picking_buffer.borrow_mut().update_texture(ui.wgpu_device(), canvas_width, canvas_height);
        renderer.renderer.render_picking(ui.wgpu_device(), ui.wgpu_queue(), &self.picking_buffer.clone().borrow(), self.calc_camera(1.0), |rndr| {
            self.render_picking(rndr, editor, &render_list);
        });

        // Render the scene
        ui.with_parent(canvas_container.node_ref, |ui| {
            pierro::canvas(ui, (resize_margin as f32 * ui.scale_factor()) as u32, |ui, texture, response| {
                self.canvas_contents(ui, texture, response, project, editor, systems, renderer, clip, &render_list, &modifiable_objs, canvas_width, canvas_height, resize_margin); 
            });
        });
        
    }

}
