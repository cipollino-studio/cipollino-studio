
use cosmic_text::Edit;

use crate::{theme, Key, Response, UI};
use super::{font_system, TextEditMemory};

pub(super) fn text_edit_mouse_input(ui: &mut UI, text_edit: &Response, memory: &mut TextEditMemory) {
    let widget_margin = ui.style::<theme::WidgetMargin>(); 

    if let Some(mouse_pos) = text_edit.mouse_pos(ui) {
        let mouse_pos = mouse_pos - widget_margin.min;
        if text_edit.mouse_pressed() {
            if !ui.input().key_down(&Key::SHIFT) {
                memory.editor.set_selection(cosmic_text::Selection::None);
                memory.editor.action(font_system(ui), cosmic_text::Action::Click { x: (mouse_pos.x + memory.scroll) as i32, y: mouse_pos.y as i32 });
            } else {
                memory.editor.action(font_system(ui), cosmic_text::Action::Drag { x: (mouse_pos.x + memory.scroll) as i32, y: mouse_pos.y as i32 });
            }
        }
        if text_edit.dragging() {
            memory.editor.action(font_system(ui), cosmic_text::Action::Drag { x: (mouse_pos.x + memory.scroll) as i32, y: mouse_pos.y as i32 });
        }
        if text_edit.mouse_double_clicked() {
            memory.editor.action(font_system(ui), cosmic_text::Action::DoubleClick { x: (mouse_pos.x + memory.scroll) as i32, y: mouse_pos.y as i32 });
        }
        if text_edit.mouse_triple_clicked() {
            memory.editor.action(font_system(ui), cosmic_text::Action::TripleClick { x: (mouse_pos.x + memory.scroll) as i32, y: mouse_pos.y as i32 });
        }
    }
}