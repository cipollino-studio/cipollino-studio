
use crate::{theme, Id, Response, UI};
use crate::{LayoutInfo, PaintText, Rect};
use cosmic_text::{Edit, FontSystem};

use super::{paint_text_edit, text_edit_keyboard_input, text_edit_mouse_input};

pub(super) struct TextEditMemory {
    pub(super) editor: cosmic_text::Editor<'static>,
    pub(super) scroll: f32
}

pub struct TextEditResponse {
    pub response: Response,
    pub done_editing: bool 
}

pub(super) fn font_system<'a>(ui: &'a mut UI) -> &'a mut FontSystem {
    ui.font_system(ui.text_font()).unwrap()
}

pub fn text_edit_begin_editing(ui: &mut UI, text_edit: Id, text: &mut String) {
    let font_size = ui.style::<theme::LabelFontSize>();

    ui.memory().request_focus(text_edit);

    let mut buffer = cosmic_text::Buffer::new(font_system(ui), cosmic_text::Metrics { font_size, line_height: font_size });
    buffer.set_text(font_system(ui), text, cosmic_text::Attrs::new().family(cosmic_text::Family::SansSerif), cosmic_text::Shaping::Advanced);
    let mut editor = cosmic_text::Editor::new(buffer);
    editor.action(font_system(ui), cosmic_text::Action::Motion(cosmic_text::Motion::End));
    ui.memory().insert(text_edit, TextEditMemory {
        editor,
        scroll: 0.0
    });
}

pub fn editing_text(ui: &mut UI, text_edit: Id) -> bool {
    ui.memory().has::<TextEditMemory>(text_edit)
}

pub fn text_edit_interaction(ui: &mut UI, text_edit: Response, text: &mut String) -> TextEditResponse {

    let widget_margin = ui.style::<theme::WidgetMargin>(); 
    let text_style = theme::label_text_style(ui);

    let mut done_editing = false;

    if let Some(mut memory) = ui.memory().remove::<TextEditMemory>(text_edit.id) {
        // Keyboard input
        text_edit_keyboard_input(ui, &text_edit, &mut memory, &mut done_editing);

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

        // Mouse interactions
        text_edit_mouse_input(ui, &text_edit, &mut memory);

        // Paint text, cursor and selection
        paint_text_edit(ui, &text_edit, &mut memory, text, cursor_pos);

        // Put the memory back where it belongs
        ui.memory().insert(text_edit.id, memory);
    } else {
        // Paint text
        let paint_text = text.clone();
        ui.set_on_paint(text_edit.node_ref, move |painter, rect| {
            painter.text(PaintText::new(paint_text, text_style, Rect::to_infinity(rect.tl() + widget_margin.min)));
        });
    }

    if editing_text(ui, text_edit.id) && text_edit.mouse_pressed_outside(ui) {
        done_editing = true;
    }
    if done_editing {
        text_edit.release_focus(ui);
    }
    if !text_edit.is_focused(ui) {
        ui.memory().remove::<TextEditMemory>(text_edit.id);
    }

    TextEditResponse {
        response: text_edit,
        done_editing
    }
}