
use cosmic_text::{Edit, FontSystem};

use crate::{vec2, CursorIcon, Key, LayoutInfo, LogicalKey, PaintRect, PaintText, Rect, Size, UINodeParams, Vec2, UI};

use super::{label_text_style, Theme};

struct TextEditMemory {
    editor: cosmic_text::Editor<'static>,
    scroll: f32
}

fn font_system<'a>(ui: &'a mut UI) -> &'a mut FontSystem {
    ui.font_system(ui.text_font()).unwrap()
}

pub fn text_edit(ui: &mut UI, text: &mut String) {

    let theme = ui.style::<Theme>();
    let color = theme.bg_text_field;
    let widget_margin = theme.widget_margin;
    let widget_rounding = theme.widget_rounding;
    let font_size = theme.label_font_size;
    let font_color = theme.text;
    let text_style = label_text_style(ui);

    let size = 200.0;

    let text_edit = ui.node(
        UINodeParams::new(Size::px(size), Size::px(font_size + 2.0 * widget_margin))
            .sense_mouse()
            .with_fill(color)
            .with_rounding(widget_rounding)
    );

    if text_edit.mouse_pressed() && !text_edit.is_focused(ui) {
        text_edit.request_focus(ui);
        let mut buffer = cosmic_text::Buffer::new(font_system(ui), cosmic_text::Metrics { font_size, line_height: font_size });
        buffer.set_text(font_system(ui), text, cosmic_text::Attrs::new().family(cosmic_text::Family::SansSerif), cosmic_text::Shaping::Advanced);
        let editor = cosmic_text::Editor::new(buffer);
        ui.memory().insert(text_edit.id, TextEditMemory {
            editor,
            scroll: 0.0
        });
    }
    if text_edit.mouse_pressed_outside(ui) {
        text_edit.release_focus(ui);
    }
    if !text_edit.is_focused(ui) {
        ui.memory().remove::<TextEditMemory>(text_edit.id);
    }

    let focused = text_edit.is_focused(ui); 
    let theme = ui.style::<Theme>();
    let target_color = if focused {
        theme.pressed_color(color)
    } else if text_edit.hovered {
        theme.hovered_color(color)
    } else {
        color
    };

    ui.set_fill(text_edit.node_ref, target_color);

    if let Some(mut memory) = ui.memory().remove::<TextEditMemory>(text_edit.id) {

        // Keyboard input
        ui.request_ime(text_edit.node_ref);

        for key in ui.input().keys_pressed.clone() {
            if let Some(text) = key.text {
                if ui.input().key_down(Key::COMMAND) && text.to_lowercase() == "v" {
                    for char in ui.get_clipboard_text().unwrap_or(String::new()).chars() {
                        memory.editor.action(font_system(ui), cosmic_text::Action::Insert(char));
                    }
                } else if ui.input().key_down(Key::COMMAND) && text.to_lowercase() == "c" {
                    if let Some(text) = memory.editor.copy_selection() {
                        ui.set_clipboard_text(text);
                    }
                } else if ui.input().key_down(Key::COMMAND) && text.to_lowercase() == "x" {
                    if let Some(text) = memory.editor.copy_selection() {
                        ui.set_clipboard_text(text);
                    }
                    memory.editor.delete_selection();
                } else {
                    for char in text.chars() {
                        memory.editor.action(font_system(ui), cosmic_text::Action::Insert(char));
                    }
                }
            }
            match key.logical_key {
                Some(LogicalKey::Space) => {
                    memory.editor.action(font_system(ui), cosmic_text::Action::Insert(' '));
                },
                Some(LogicalKey::ArrowLeft) | Some(LogicalKey::Home) => {
                    if !ui.input().key_down(Key::SHIFT) {
                        if let Some((min, _)) = memory.editor.selection_bounds() {
                            memory.editor.set_cursor(min);
                        }
                        memory.editor.set_selection(cosmic_text::Selection::None);
                    } else {
                        if memory.editor.selection_bounds().is_none() {
                            memory.editor.set_selection(cosmic_text::Selection::Normal(memory.editor.cursor()));
                        }
                    }

                    let motion = if key.logical_key == Some(LogicalKey::Home) {
                        cosmic_text::Motion::Home
                    } else if ui.input().key_down(Key::COMMAND) {
                        cosmic_text::Motion::LeftWord
                    } else {
                        cosmic_text::Motion::Left
                    };

                    memory.editor.action(font_system(ui), cosmic_text::Action::Motion(motion));
                },
                Some(LogicalKey::ArrowRight) | Some(LogicalKey::End) => {
                    if !ui.input().key_down(Key::SHIFT) {
                        if let Some((_, max)) = memory.editor.selection_bounds() {
                            memory.editor.set_cursor(max);
                        }
                        memory.editor.set_selection(cosmic_text::Selection::None);
                    } else {
                        if memory.editor.selection_bounds().is_none() {
                            memory.editor.set_selection(cosmic_text::Selection::Normal(memory.editor.cursor()));
                        }
                    }

                    let motion = if key.logical_key == Some(LogicalKey::End) {
                        cosmic_text::Motion::End
                    } else if ui.input().key_down(Key::COMMAND) {
                        cosmic_text::Motion::RightWord
                    } else {
                        cosmic_text::Motion::Right
                    };

                    memory.editor.action(font_system(ui), cosmic_text::Action::Motion(motion));
                },
                Some(LogicalKey::Backspace) => {
                    if ui.input().key_down(Key::COMMAND) {
                        if memory.editor.selection_bounds().is_none() {
                            memory.editor.set_selection(cosmic_text::Selection::Normal(memory.editor.cursor()));
                            memory.editor.action(font_system(ui), cosmic_text::Action::Motion(cosmic_text::Motion::LeftWord));
                        }
                    }
                    memory.editor.action(font_system(ui), cosmic_text::Action::Backspace);
                },
                Some(LogicalKey::Delete) => {
                    if ui.input().key_down(Key::COMMAND) {
                        if memory.editor.selection_bounds().is_none() {
                            memory.editor.set_selection(cosmic_text::Selection::Normal(memory.editor.cursor()));
                            memory.editor.action(font_system(ui), cosmic_text::Action::Motion(cosmic_text::Motion::RightWord));
                        }
                    }
                    memory.editor.action(font_system(ui), cosmic_text::Action::Delete);
                },
                _ => {}
            }
        }
        if !ui.input().ime_preedit.is_empty() {
            memory.editor.delete_selection();
        }
        if let Some(ime_commit_text) = ui.input().ime_commit.clone() {
            for char in ime_commit_text.chars() {
                memory.editor.action(font_system(ui), cosmic_text::Action::Insert(char));
            }
        }

        // Update text
        memory.editor.with_buffer(|buffer| {
            let buffer_text = buffer.lines.first()?.text();
            *text = buffer_text.to_string();
            Some(())
        });
        memory.editor.shape_as_needed(font_system(ui), true);

        // Update scroll
        let cursor_pos = memory.editor.cursor_position();
        let text_edit_width = ui.memory().get::<LayoutInfo>(text_edit.id).rect.size().x;
        if let Some((cursor_x, _)) = cursor_pos {
            memory.scroll = memory.scroll.max(cursor_x as f32 - text_edit_width + 10.0);
            memory.scroll = memory.scroll.min(cursor_x as f32);
        }
        let scroll = memory.scroll;

        // Mouse interactions
        if let Some(mouse_pos) = text_edit.mouse_pos(ui) {
            let mouse_pos = mouse_pos - Vec2::splat(widget_margin);
            if text_edit.mouse_pressed() {
                if !ui.input().key_down(Key::SHIFT) {
                    memory.editor.set_selection(cosmic_text::Selection::None);
                    memory.editor.action(font_system(ui), cosmic_text::Action::Click { x: (mouse_pos.x + scroll) as i32, y: mouse_pos.y as i32 });
                } else {
                    memory.editor.action(font_system(ui), cosmic_text::Action::Drag { x: (mouse_pos.x + scroll) as i32, y: mouse_pos.y as i32 });
                }
            }
            if text_edit.dragging() {
                memory.editor.action(font_system(ui), cosmic_text::Action::Drag { x: (mouse_pos.x + scroll) as i32, y: mouse_pos.y as i32 });
            }
            if text_edit.mouse_double_clicked() {
                memory.editor.action(font_system(ui), cosmic_text::Action::DoubleClick { x: (mouse_pos.x + scroll) as i32, y: mouse_pos.y as i32 });
            }
            if text_edit.mouse_triple_clicked() {
                memory.editor.action(font_system(ui), cosmic_text::Action::TripleClick { x: (mouse_pos.x + scroll) as i32, y: mouse_pos.y as i32 });
            }
        }

        // Paint text, cursor and selection
        let paint_text = text.clone();
        let ranges = memory.editor.selection_bounds().map(|(from, to)| {
            let from = from.index;
            let to = to.index;
            memory.editor.with_buffer(|buffer| {
                highlight_line(&buffer.lines[0], from, to).collect::<Vec<_>>()
            })
        }).unwrap_or_default();

        ui.set_on_paint(text_edit.node_ref, move |painter, rect| {
            painter.text(PaintText::new(paint_text, text_style, Rect::to_infinity(rect.tl() + Vec2::splat(widget_margin) - Vec2::X * scroll)));

            let origin = rect.tl() + Vec2::splat(widget_margin);
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
                        rect.tl() + vec2(from_x - scroll + widget_margin, widget_margin),
                        vec2(width, font_size) 
                    ),
                    font_color.with_alpha(0.2))
                );
            }
            
        });

        // Put the memory back where it belongs
        ui.memory().insert(text_edit.id, memory);
    } else {

        // Paint text
        let paint_text = text.clone();
        ui.set_on_paint(text_edit.node_ref, move |painter, rect| {
            painter.text(PaintText::new(paint_text, text_style, Rect::to_infinity(rect.tl() + Vec2::splat(widget_margin))));
        });

    }

    if text_edit.hovered && text_edit.contains_mouse(ui) {
        ui.set_cursor(CursorIcon::Text);
    }

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
