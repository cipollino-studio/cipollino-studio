
use crate::{panels::timeline::frame_area::audio::AudioInstanceBar, TimelinePanel};

use super::{layer::FrameDot, FrameArea};

/// Commands for painting frame dots, audio clips, etc in the timeline's frame area.
/// This is necessary because painting happens after the UI tree is constructed,
/// so we can't use any borrowed data in the paint callback. Using a command
/// queue gets around this problem.
pub(super) struct PaintCommands {
    pub frame_dots: Vec<FrameDot>,
    pub audio_bars: Vec<AudioInstanceBar>
}

impl PaintCommands {

    pub fn new() -> Self {
        Self {
            frame_dots: Vec::new(),
            audio_bars: Vec::new()
        }
    }

    pub fn paint(self, painter: &mut pierro::Painter, rect: pierro::Rect, framerate: f32, text_color: pierro::Color, accent_color: pierro::Color) {
        for frame_dot in self.frame_dots {
            frame_dot.paint(painter, rect, text_color, accent_color);
        }
        for audio_bar in self.audio_bars {
            audio_bar.paint(painter, rect, framerate, accent_color);
        }
    }

}

impl FrameArea {

    pub(super) fn paint_frame_area(
        painter: &mut pierro::Painter,
        rect: pierro::Rect,
        n_layers: usize,
        n_frames: u32,
        clip_length: u32,
        framerate: f32,

        curr_frame: i32,
        active_layer_idx: Option<usize>,
        selection_rect: Option<pierro::Rect>,

        text_color: pierro::Color,
        column_highlight: pierro::Color,
        accent_color: pierro::Color,

        paint_commands: PaintCommands

    ) {
        // Column highlight
        for i in ((TimelinePanel::FRAME_NUMBER_STEP - 1)..(n_frames as i32)).step_by(TimelinePanel::FRAME_NUMBER_STEP as usize) {
            let column_rect = pierro::Rect::min_size(
                rect.tl() + pierro::Vec2::X * (i as f32) * TimelinePanel::FRAME_WIDTH,
                pierro::vec2(TimelinePanel::FRAME_WIDTH, rect.height())
            );
            painter.rect(pierro::PaintRect::new(column_rect, column_highlight));
        }

        // Active layer highlight
        if let Some(active_layer_idx) = active_layer_idx {
            let layer_rect = pierro::Rect::min_size(
                rect.tl() + pierro::Vec2::Y * (active_layer_idx as f32) * TimelinePanel::LAYER_HEIGHT,
                pierro::vec2(rect.width(), TimelinePanel::LAYER_HEIGHT)
            );
            let highlight_color = accent_color.with_alpha(0.1);
            painter.rect(pierro::PaintRect::new(layer_rect, highlight_color));
        }
        
        paint_commands.paint(painter, rect, framerate, text_color, accent_color);

        // Box selection
        if let Some(selection_rect) = selection_rect {
            painter.rect(
                pierro::PaintRect::new(selection_rect.shift(pierro::Margin::same(0.5).grow(rect).tl()), accent_color.with_alpha(0.1))
                    .with_stroke(pierro::Stroke::new(accent_color, 1.0))
                    .with_rounding(pierro::Rounding::same(3.0))
            );
        }

        // Playback line
        let playback_line_thickness = 1.5;
        let playback_line = pierro::Rect::min_size(
            rect.tl() + pierro::Vec2::X * (((curr_frame as f32) + 0.5) * TimelinePanel::FRAME_WIDTH - playback_line_thickness / 2.0),
            pierro::vec2(playback_line_thickness, rect.height()) 
        );
        painter.rect(pierro::PaintRect::new(playback_line, accent_color));

        // Shadows
        let bottom_shadow_rect = pierro::Rect::min_max(
            rect.tl() + pierro::Vec2::Y * (n_layers as f32) * TimelinePanel::LAYER_HEIGHT,
            rect.br()
        );
        let right_shadow_rect = pierro::Rect::min_max(
            rect.tl() + pierro::Vec2::X * (clip_length as f32) * TimelinePanel::FRAME_WIDTH,
            bottom_shadow_rect.tr()
        );
        let shadow_color = pierro::Color::rgba(0.0, 0.0, 0.0, 0.4);
        painter.rect(pierro::PaintRect::new(bottom_shadow_rect, shadow_color));
        painter.rect(pierro::PaintRect::new(right_shadow_rect, shadow_color));

    }

}
