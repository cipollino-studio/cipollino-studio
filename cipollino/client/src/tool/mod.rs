
mod util;
pub use util::*;

mod select;
pub use select::*;

mod pencil;
pub use pencil::*;

mod color_picker;
pub use color_picker::*;

mod bucket;
pub use bucket::*;

use project::{Action, Client, ClipInner, CreateFrame, Frame, FrameTreeData, Layer, Ptr, SceneObjPtr};
use std::collections::HashSet;
use crate::{AppSystems, EditorState, ProjectState, SceneRenderList, Shortcut};

pub struct ToolContext<'ctx> {
    pub device: &'ctx pierro::wgpu::Device,
    pub queue: &'ctx pierro::wgpu::Queue,
    // None when rendering the scene overlay, since the renderer will be borrowed then
    pub renderer: Option<&'ctx mut malvina::Renderer>,

    pub project: &'ctx ProjectState,
    pub systems: &'ctx mut AppSystems,
    pub clip: &'ctx ClipInner,
    pub active_layer: Ptr<Layer>,
    pub frame_time: i32,

    pub render_list: &'ctx SceneRenderList,
    pub modifiable_objs: &'ctx HashSet<SceneObjPtr>,

    // Picking
    pub picking_buffer: &'ctx mut malvina::PickingBuffer,
    pub picking_mouse_pos: Option<(u32, u32)>, 

    pub pressure: f32,
    pub cam_zoom: f32,
    pub key_modifiers: pierro::KeyModifiers
}

pub fn get_active_frame(client: &Client, editor: &EditorState, action: &mut Action) -> Option<Ptr<Frame>> {
    if !editor.can_modify_layer(editor.active_layer) {
        return None;
    }

    let clip = client.get(client.get(editor.open_clip)?.inner)?;
    let frame_time = clip.frame_idx(editor.time);

    let layer = client.get(editor.active_layer)?;
    if let Some(frame) = layer.frame_at(client, frame_time) {
        return Some(frame);
    }

    let new_frame_ptr = client.next_ptr();
    action.push(CreateFrame {
        ptr: new_frame_ptr,
        layer: editor.active_layer,
        data: FrameTreeData {
            time: frame_time,
            ..Default::default()
        },
    });

    Some(new_frame_ptr)
}

impl ToolContext<'_> {

    pub fn active_frame(&self, editor: &EditorState, action: &mut Action) -> Option<Ptr<Frame>> {
        get_active_frame(&self.project.client, editor, action)    
    }

    pub fn pick(&mut self, x: u32, y: u32) -> Option<SceneObjPtr> {
        let idx = self.picking_buffer.read_pixel(self.device, self.queue, x, y) as usize;
        if idx == 0 || idx > self.render_list.objs.len() {
            return None;
        } 
        Some(self.render_list.objs[idx - 1])
    }

}

pub trait Tool: Default {

    const ICON: &'static str;

    type Shortcut: Shortcut;

    fn tick(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext) {}

    fn mouse_pressed(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, _pos: elic::Vec2) {}
    fn mouse_released(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, _pos: elic::Vec2) {}
    fn mouse_clicked(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, _pos: elic::Vec2) {}

    fn mouse_drag_started(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, _pos: elic::Vec2) {}
    fn mouse_drag_stopped(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, _pos: elic::Vec2) {}
    fn mouse_dragged(&mut self, _editor: &mut EditorState, _ctx: &mut ToolContext, _pos: elic::Vec2) {}

    fn render_overlay(&self, _ctx: &mut ToolContext, _rndr: &mut malvina::LayerRenderer, _accent_color: elic::Color) {}

    fn settings(&mut self, _ui: &mut pierro::UI, _systems: &mut AppSystems) {}

    fn cursor_icon(&self, _editor: &mut EditorState, _ctx: &mut ToolContext, _pos: elic::Vec2) -> pierro::CursorIcon {
        pierro::CursorIcon::Default
    }
    
}

pub trait ToolDyn {

    fn icon(&self) -> &'static str;

    fn tick(&mut self, editor: &mut EditorState, ctx: &mut ToolContext);

    fn mouse_pressed(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2);
    fn mouse_released(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2);
    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2);

    fn mouse_drag_started(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, _pos: malvina::Vec2);
    fn mouse_drag_stopped(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, _pos: malvina::Vec2);
    fn mouse_dragged(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, _pos: malvina::Vec2);

    fn render_overlay(&self, ctx: &mut ToolContext, rndr: &mut malvina::LayerRenderer, accent_color: elic::Color);

    fn settings(&mut self, ui: &mut pierro::UI, systems: &mut AppSystems);

    fn cursor_icon(&self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) -> pierro::CursorIcon;

}

impl<T: Tool> ToolDyn for T {

    fn icon(&self) -> &'static str {
        Self::ICON
    }

    fn tick(&mut self, editor: &mut EditorState, ctx: &mut ToolContext) {
        self.tick(editor, ctx); 
    }

    fn mouse_pressed(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_pressed(editor, ctx, pos);
    }

    fn mouse_released(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_released(editor, ctx, pos);
    }

    fn mouse_clicked(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_clicked(editor, ctx, pos);
    }

    fn mouse_drag_started(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_drag_started(editor, ctx, pos);
    }

    fn mouse_drag_stopped(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_drag_stopped(editor, ctx, pos);
    }

    fn mouse_dragged(&mut self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_dragged(editor, ctx, pos);
    }

    fn render_overlay(&self, ctx: &mut ToolContext, rndr: &mut malvina::LayerRenderer, accent_color: elic::Color) {
        self.render_overlay(ctx, rndr, accent_color);
    }

    fn settings(&mut self, ui: &mut pierro::UI, systems: &mut AppSystems) {
        self.settings(ui, systems);
    }

    fn cursor_icon(&self, editor: &mut EditorState, ctx: &mut ToolContext, pos: malvina::Vec2) -> pierro::CursorIcon {
        self.cursor_icon(editor, ctx, pos)
    }

}
