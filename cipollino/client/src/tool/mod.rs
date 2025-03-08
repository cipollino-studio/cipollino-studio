
mod select;
pub use select::*;

mod pencil;
pub use pencil::*;

use project::{Action, ClipInner, CreateFrame, Frame, FrameTreeData, Layer, Ptr};

use crate::{EditorState, ProjectState};

pub struct ToolContext<'ctx> {
    pub device: &'ctx pierro::wgpu::Device,
    pub project: &'ctx ProjectState,
    pub editor: &'ctx mut EditorState,
    pub clip: &'ctx ClipInner,
    pub active_layer: Ptr<Layer>,
    pub frame_time: i32,

    // Misc
    pub clear_stroke_preview: bool
}

impl ToolContext<'_> {

    pub fn active_frame(&self, action: &mut Action) -> Option<Ptr<Frame>> {
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

    fn mouse_pressed(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2) {}
    fn mouse_released(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2) {}
    fn mouse_clicked(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2) {}

    fn mouse_drag_started(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2) {}
    fn mouse_drag_stopped(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2) {}
    fn mouse_dragged(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2) {}

    fn cursor_icon(&self) -> pierro::CursorIcon {
        pierro::CursorIcon::Default
    }
    
}

pub trait ToolDyn {

    fn icon(&self) -> &'static str;

    fn mouse_pressed(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2);
    fn mouse_released(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2);
    fn mouse_clicked(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2);

    fn mouse_drag_started(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2);
    fn mouse_drag_stopped(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2);
    fn mouse_dragged(&mut self, _ctx: &mut ToolContext, _pos: malvina::Vec2);

    fn cursor_icon(&self) -> pierro::CursorIcon;

}

impl<T: Tool> ToolDyn for T {

    fn icon(&self) -> &'static str {
        Self::ICON
    }

    fn mouse_pressed(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_pressed(ctx, pos);
    }

    fn mouse_released(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_released(ctx, pos);
    }

    fn mouse_clicked(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_clicked(ctx, pos);
    }

    fn mouse_drag_started(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_drag_started(ctx, pos);
    }

    fn mouse_drag_stopped(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_drag_stopped(ctx, pos);
    }

    fn mouse_dragged(&mut self, ctx: &mut ToolContext, pos: malvina::Vec2) {
        self.mouse_dragged(ctx, pos);
    }

    fn cursor_icon(&self) -> pierro::CursorIcon {
        self.cursor_icon()
    }

}
