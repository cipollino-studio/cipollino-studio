
use paint::PaintCommands;
use project::Clip;

use crate::{EditorState, ProjectState};

use super::{render_list::RenderLayerKind, RenderList, TimelinePanel};

mod paint;
mod layer;
mod selection;

pub use selection::*;

impl TimelinePanel {

    pub(super) const FRAME_WIDTH: f32 = 12.0;
    pub(super) const LAYER_HEIGHT: f32 = 20.0; 
    
    pub const FRAME_SIZE: pierro::Vec2 = pierro::vec2(Self::FRAME_WIDTH, Self::LAYER_HEIGHT);

    fn stop_drag(&mut self, project: &ProjectState) {
        self.frame_selection.move_selected(project, self.frame_drag_x);
        self.frame_drag_x = 0.0;
    }

    fn render_layers(&mut self, ui: &mut pierro::UI, project: &ProjectState, frame_area: &pierro::Response, paint_commands: &mut PaintCommands, render_list: &RenderList) {
        for (idx, render_layer) in render_list.iter().enumerate() {
            match render_layer.kind {
                RenderLayerKind::Layer(_ptr, layer) => self.render_layer_contents(ui, project, frame_area, paint_commands, idx, layer),
            }
        }
    }

    fn paint_frame_area(
        painter: &mut pierro::Painter,
        rect: pierro::Rect,
        n_layers: usize,
        n_frames: u32,
        clip_length: u32,

        curr_frame: i32,
        active_layer_idx: Option<usize>,

        text_color: pierro::Color,
        column_highlight: pierro::Color,
        accent_color: pierro::Color,

        paint_commands: PaintCommands

    ) {
        // Column highlight
        for i in ((Self::FRAME_NUMBER_STEP - 1)..(n_frames as i32)).step_by(Self::FRAME_NUMBER_STEP as usize) {
            let column_rect = pierro::Rect::min_size(
                rect.tl() + pierro::Vec2::X * (i as f32) * Self::FRAME_WIDTH,
                pierro::vec2(Self::FRAME_WIDTH, rect.height())
            );
            painter.rect(pierro::PaintRect::new(column_rect, column_highlight));
        }

        // Active layer highlight
        if let Some(active_layer_idx) = active_layer_idx {
            let layer_rect = pierro::Rect::min_size(
                rect.tl() + pierro::Vec2::Y * (active_layer_idx as f32) * Self::LAYER_HEIGHT,
                pierro::vec2(rect.width(), Self::LAYER_HEIGHT)
            );
            let highlight_color = accent_color.with_alpha(0.1);
            painter.rect(pierro::PaintRect::new(layer_rect, highlight_color));
        }
        
        paint_commands.paint(painter, rect, text_color, accent_color);

        // Playback line
        let playback_line_thickness = 1.5;
        let playback_line = pierro::Rect::min_size(
            rect.tl() + pierro::Vec2::X * (((curr_frame as f32) + 0.5) * Self::FRAME_WIDTH - playback_line_thickness / 2.0),
            pierro::vec2(playback_line_thickness, rect.height()) 
        );
        painter.rect(pierro::PaintRect::new(playback_line, accent_color));

        // Shadows
        let bottom_shadow_rect = pierro::Rect::min_max(
            rect.tl() + pierro::Vec2::Y * (n_layers as f32) * Self::LAYER_HEIGHT,
            rect.br()
        );
        let right_shadow_rect = pierro::Rect::min_max(
            rect.tl() + pierro::Vec2::X * (clip_length as f32) * Self::FRAME_WIDTH,
            bottom_shadow_rect.tr()
        );
        let shadow_color = pierro::Color::rgba(0.0, 0.0, 0.0, 0.4);
        painter.rect(pierro::PaintRect::new(bottom_shadow_rect, shadow_color));
        painter.rect(pierro::PaintRect::new(right_shadow_rect, shadow_color));
    }

    fn frame_area_contents(&mut self, ui: &mut pierro::UI, editor: &mut EditorState, project: &ProjectState, render_list: &RenderList, clip: &Clip, n_frames: u32) {
        let bg_base_color = ui.style::<pierro::theme::BgDark>();
        let bg = bg_base_color.darken(0.2);
        let column_highlight = bg_base_color.darken(0.1);
        let accent_color = ui.style::<pierro::theme::AccentColor>();

        let width = (n_frames as f32) * Self::FRAME_WIDTH;
        let height = (render_list.len() as f32) * Self::LAYER_HEIGHT;
        let frame_area = ui.node(
            pierro::UINodeParams::new(pierro::Size::px(width), pierro::Size::px(height).with_grow(1.0))
                .with_fill(bg)
                .sense_mouse()
        );

        // Focus and selection
        if !frame_area.is_focused(ui) {
            self.frame_selection.clear();
        }
        if (frame_area.mouse_clicked() || frame_area.drag_started()) && !ui.input().key_down(pierro::Key::SHIFT) {
            self.frame_selection.clear();
        }

        // Dragging
        if frame_area.drag_started() {
            self.frame_drag_x = 0.0;
        }
        self.frame_drag_x += frame_area.drag_delta(ui).x;
        
        // Rendering
        let mut paint_commands = PaintCommands::new();
        self.render_layers(ui, project, &frame_area, &mut paint_commands, render_list);

        // stop_drag() called after rendering to avoid a flicker as the frames are moved 
        if frame_area.drag_stopped() {
            self.stop_drag(project);
        }

        let n_layers = render_list.len();
        let clip_length = clip.length;
        let curr_frame = clip.frame_idx(editor.time); 
        let text_color = ui.style::<pierro::theme::TextColor>();

        let active_layer_idx = render_list.iter().position(|layer| match layer.kind {
            RenderLayerKind::Layer(layer, _) => layer == editor.active_layer,
        });

        ui.set_on_paint(frame_area.node_ref, move |painter, rect| {
            Self::paint_frame_area(
                painter,
                rect,
                n_layers,
                n_frames,
                clip_length,
                curr_frame,
                active_layer_idx,
                text_color,
                column_highlight,
                accent_color,
                paint_commands
            );
        });
    }

    pub(super) fn frame_area(&mut self, ui: &mut pierro::UI, editor: &mut EditorState, project: &ProjectState, render_list: &RenderList, clip: &Clip, n_frames: u32) -> pierro::ScrollAreaResponse<()> {
        let mut scroll_state = self.scroll_state;
        let response = pierro::ScrollArea::default()
            .with_state(&mut scroll_state)
            .render(ui, |ui| {
                self.frame_area_contents(ui, editor, project, render_list, clip, n_frames);
            });
        self.scroll_state = scroll_state;
        response
    }

}
