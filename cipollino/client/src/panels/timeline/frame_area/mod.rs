
use paint::PaintCommands;
use project::ClipInner;

use crate::{EditorState, ProjectState};

use super::{render_list::RenderLayerKind, RenderList, TimelinePanel};

mod paint;
mod layer;
mod selection;
mod dragging;

pub use selection::*;

enum DragState {
    None,
    Move {
        offset: f32
    },
    BoxSelect {
        from: pierro::Vec2,
        to: pierro::Vec2
    }
}

impl DragState {

    fn move_offset(&self) -> f32 {
        match self {
            DragState::None => 0.0,
            DragState::Move { offset } => *offset,
            DragState::BoxSelect { .. } => 0.0,
        }
    }

    fn selection_rect(&self) -> Option<pierro::Rect> {
        let Self::BoxSelect { from, to } = self else { return None; }; 
        let min = from.min(*to);
        let max = from.max(*to);
        Some(pierro::Rect::min_max(min, max))
    }

}

pub(super) struct FrameArea {
    selection: FrameSelection,

    drag_consumed: bool,
    drag_state: DragState
}

impl FrameArea {

    pub fn new() -> Self {
        Self {
            selection: FrameSelection::new(),
            drag_consumed: false,
            drag_state: DragState::None
        }
    }

    fn render_layers(&mut self, ui: &mut pierro::UI, project: &ProjectState, frame_area: &pierro::Response, paint_commands: &mut PaintCommands, render_list: &RenderList) {
        for (idx, render_layer) in render_list.iter().enumerate() {
            match render_layer.kind {
                RenderLayerKind::Layer(_ptr, layer) => self.render_layer_contents(ui, project, frame_area, paint_commands, idx, layer),
            }
        }
    }
    
    fn frame_area_contents(&mut self, ui: &mut pierro::UI, editor: &mut EditorState, project: &ProjectState, render_list: &RenderList, clip: &ClipInner, n_frames: u32) {
        let bg_base_color = ui.style::<pierro::theme::BgDark>();
        let bg = bg_base_color.darken(0.2);
        let column_highlight = bg_base_color.darken(0.1);
        let accent_color = ui.style::<pierro::theme::AccentColor>();

        let width = (n_frames as f32) * TimelinePanel::FRAME_WIDTH;
        let height = (render_list.len() as f32) * TimelinePanel::LAYER_HEIGHT;
        let frame_area = ui.node(
            pierro::UINodeParams::new(pierro::Size::px(width), pierro::Size::px(height).with_grow(1.0))
                .with_fill(bg)
                .sense_mouse()
        );

        // Focus and selection
        if !frame_area.is_focused(ui) {
            self.selection.clear();
        }
        if frame_area.mouse_clicked() && !ui.input().key_down(pierro::Key::SHIFT) {
            self.selection.clear();
        }

        // Dragging
        match &mut self.drag_state {
            DragState::None => {},
            DragState::Move { offset } => {
                *offset += frame_area.drag_delta(ui).x;
            },
            DragState::BoxSelect { from: _, to } => {
                if let Some(mouse_pos) = frame_area.mouse_pos(ui) {
                    *to = mouse_pos;
                }
            },
        } 
        self.drag_consumed = false;

        // Rendering
        let mut paint_commands = PaintCommands::new();
        self.render_layers(ui, project, &frame_area, &mut paint_commands, render_list);

        if frame_area.drag_started() && !self.drag_consumed {
            if let Some(origin) = frame_area.mouse_pos(ui) {
                self.drag_state = DragState::BoxSelect { from: origin, to: origin }; 
                frame_area.request_focus(ui);
            }
            if !ui.input().key_down(pierro::Key::SHIFT) {
                self.selection.clear();
            }
        }
        // stop_drag() called after rendering to avoid a flicker as the frames are moved 
        if frame_area.drag_stopped() {
            self.drag_stopped(project, render_list);
        }

        // Painting the frame area contents 
        let n_layers = render_list.len();
        let clip_length = clip.length;
        let curr_frame = clip.frame_idx(editor.time); 
        let text_color = ui.style::<pierro::theme::TextColor>();
        let active_layer_idx = render_list.iter().position(|layer| match layer.kind {
            RenderLayerKind::Layer(layer, _) => layer == editor.active_layer,
        });
        let selection_rect = self.drag_state.selection_rect();
        ui.set_on_paint(frame_area.node_ref, move |painter, rect| {
            Self::paint_frame_area(
                painter,
                rect,
                n_layers,
                n_frames,
                clip_length,
                curr_frame,
                active_layer_idx,
                selection_rect,
                text_color,
                column_highlight,
                accent_color,
                paint_commands
            );
        });
    }

}

impl TimelinePanel {

    pub(super) const FRAME_WIDTH: f32 = 12.0;
    pub(super) const LAYER_HEIGHT: f32 = 20.0; 
    
    pub const FRAME_SIZE: pierro::Vec2 = pierro::vec2(Self::FRAME_WIDTH, Self::LAYER_HEIGHT);

    pub(super) fn frame_area(&mut self, ui: &mut pierro::UI, editor: &mut EditorState, project: &ProjectState, render_list: &RenderList, clip: &ClipInner, n_frames: u32) -> pierro::ScrollAreaResponse<()> {
        let mut scroll_state = self.scroll_state;
        let response = pierro::ScrollArea::default()
            .with_state(&mut scroll_state)
            .render(ui, |ui| {
                self.frame_area.frame_area_contents(ui, editor, project, render_list, clip, n_frames);
            });
        self.scroll_state = scroll_state;
        response
    }

}
