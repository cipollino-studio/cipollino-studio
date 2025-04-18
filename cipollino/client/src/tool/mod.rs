
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

use project::{Action, ClipInner, CreateFrame, Frame, FrameTreeData, Layer, Ptr, Stroke};
use std::collections::HashSet;
use crate::{AppSystems, EditorState, ProjectState, Shortcut};

pub struct ToolContext<'ctx> {
    pub device: &'ctx pierro::wgpu::Device,
    pub queue: &'ctx pierro::wgpu::Queue,

    pub project: &'ctx ProjectState,
    pub systems: &'ctx mut AppSystems,
    pub clip: &'ctx ClipInner,
    pub active_layer: Ptr<Layer>,
    pub frame_time: i32,

    pub rendered_strokes: &'ctx HashSet<Ptr<Stroke>>,
    pub modifiable_strokes: &'ctx HashSet<Ptr<Stroke>>,

    // Picking
    pub picking_buffer: &'ctx mut malvina::PickingBuffer,
    pub picking_mouse_pos: Option<(u32, u32)>, 

    pub pressure: f32,
    pub cam_zoom: f32,
    pub key_modifiers: pierro::KeyModifiers
}

impl ToolContext<'_> {

    pub fn active_frame(&self, editor: &EditorState, action: &mut Action) -> Option<Ptr<Frame>> {
        if !editor.can_modify_layer(self.active_layer) {
            return None;
        }

        let layer = self.project.client.get(self.active_layer)?;
        if let Some(frame) = layer.frame_at(&self.project.client, self.frame_time) {
            return Some(frame);
        }

        let new_frame_ptr = self.project.client.next_ptr()?;
        action.push(CreateFrame {
            ptr: new_frame_ptr,
            layer: self.active_layer,
            data: FrameTreeData {
                time: self.frame_time,
                ..Default::default()
            },
        });

        Some(new_frame_ptr)
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
