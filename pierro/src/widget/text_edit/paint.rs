
use cosmic_text::Edit;

use crate::{theme, vec2, PaintRect, PaintText, Rect, Response, Vec2, UI};

use super::TextEditMemory;

pub(super) fn paint_text_edit(ui: &mut UI, text_edit: &Response, memory: &mut TextEditMemory, text: &mut String, cursor_pos: Option<(i32, i32)>) {

    let font_size = ui.style::<theme::LabelFontSize>();
    let widget_margin = ui.style::<theme::WidgetMargin>(); 
    let font_color = ui.style::<theme::TextColor>(); 
    let text_style = theme::label_text_style(ui);

    let paint_text = text.clone();
    let ranges = memory.editor.selection_bounds().map(|(from, to)| {
        let from = from.index;
        let to = to.index;
        memory.editor.with_buffer(|buffer| {
            highlight_line(&buffer.lines[0], from, to).collect::<Vec<_>>()
        })
    }).unwrap_or_default();

    let scroll = memory.scroll;
    ui.set_on_paint(text_edit.node_ref, move |painter, rect| {
        painter.text(PaintText::new(paint_text, text_style, Rect::to_infinity(rect.tl() + widget_margin.min - Vec2::X * scroll)));

        let origin = rect.tl() + widget_margin.min;
        if let Some((cursor_x, cursor_y)) = cursor_pos {
            let cursor_rect = Rect::min_size(
                origin + vec2(cursor_x as f32 - scroll, cursor_y as f32),
                vec2(1.0, font_size)
            );
            painter.rect(PaintRect::new(cursor_rect, font_color));
        }

        for (from_x, width) in ranges {
            painter.rect(PaintRect::new(
                Rect::min_size(
                    rect.tl() + vec2(from_x - scroll + widget_margin.min.x, widget_margin.min.y),
                    vec2(width, font_size) 
                ),
                font_color.with_alpha(0.2))
            );
        }
        
    });

}

// Taken from iced.
// TODO: proper bidi text selection
fn highlight_line(
    line: &cosmic_text::BufferLine,
    from: usize,
    to: usize,
) -> impl Iterator<Item = (f32, f32)> + '_ {
    let layout = line
        .layout_opt()
        .as_ref()
        .map(Vec::as_slice)
        .unwrap_or_default();

    layout.iter().map(move |visual_line| {
        let start = visual_line
            .glyphs
            .first()
            .map(|glyph| glyph.start)
            .unwrap_or(0);
        let end = visual_line
            .glyphs
            .last()
            .map(|glyph| glyph.end)
            .unwrap_or(0);

        let range = start.max(from)..end.min(to);

        if range.is_empty() {
            (0.0, 0.0)
        } else if range.start == start && range.end == end {
            (0.0, visual_line.w)
        } else {
            let first_glyph = visual_line
                .glyphs
                .iter()
                .position(|glyph| range.start <= glyph.start)
                .unwrap_or(0);

            let mut glyphs = visual_line.glyphs.iter();

            let x =
                glyphs.by_ref().take(first_glyph).map(|glyph| glyph.w).sum();

            let width: f32 = glyphs
                .take_while(|glyph| range.end > glyph.start)
                .map(|glyph| glyph.w)
                .sum();

            (x, width)
        }
    })
}
