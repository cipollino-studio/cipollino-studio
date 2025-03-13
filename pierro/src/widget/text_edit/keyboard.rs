
use cosmic_text::Edit;

use crate::{Key, LogicalKey, Response, UI};

use super::{font_system, TextEditMemory};

pub(super) fn text_edit_keyboard_input(ui: &mut UI, text_edit: &Response, memory: &mut TextEditMemory, done_editing: &mut bool) {

    ui.request_ime(text_edit.node_ref);

    for key in ui.input().keys_pressed.clone() {
        if let Some(text) = key.text {
            if ui.input().key_down(&Key::COMMAND) && text.to_lowercase() == "v" {
                for char in ui.get_clipboard_text().unwrap_or(String::new()).chars() {
                    memory.editor.action(font_system(ui), cosmic_text::Action::Insert(char));
                }
            } else if ui.input().key_down(&Key::COMMAND) && text.to_lowercase() == "c" {
                if let Some(text) = memory.editor.copy_selection() {
                    ui.set_clipboard_text(text);
                }
            } else if ui.input().key_down(&Key::COMMAND) && text.to_lowercase() == "x" {
                if let Some(text) = memory.editor.copy_selection() {
                    ui.set_clipboard_text(text);
                }
                memory.editor.delete_selection();
            } else if ui.input().key_down(&Key::COMMAND) && text.to_lowercase() == "a" {
                memory.editor.set_cursor(cosmic_text::Cursor::default());
                let last_cursor = memory.editor.with_buffer(|buffer| {
                    let line_i = buffer.lines.len().saturating_sub(1);
                    buffer.lines
                        .last()
                        .map(|x| x.text().len())
                        .map(|index| cosmic_text::Cursor::new(line_i, index))
                        .unwrap_or_default()
                });
                memory.editor.set_selection(cosmic_text::Selection::Normal(last_cursor));
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
                if !ui.input().key_down(&Key::SHIFT) {
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
                } else if ui.input().key_down(&Key::COMMAND) {
                    cosmic_text::Motion::LeftWord
                } else {
                    cosmic_text::Motion::Left
                };

                memory.editor.action(font_system(ui), cosmic_text::Action::Motion(motion));
            },
            Some(LogicalKey::ArrowRight) | Some(LogicalKey::End) => {
                if !ui.input().key_down(&Key::SHIFT) {
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
                } else if ui.input().key_down(&Key::COMMAND) {
                    cosmic_text::Motion::RightWord
                } else {
                    cosmic_text::Motion::Right
                };

                memory.editor.action(font_system(ui), cosmic_text::Action::Motion(motion));
            },
            Some(LogicalKey::Backspace) => {
                if ui.input().key_down(&Key::COMMAND) {
                    if memory.editor.selection_bounds().is_none() {
                        memory.editor.set_selection(cosmic_text::Selection::Normal(memory.editor.cursor()));
                        memory.editor.action(font_system(ui), cosmic_text::Action::Motion(cosmic_text::Motion::LeftWord));
                    }
                }
                memory.editor.action(font_system(ui), cosmic_text::Action::Backspace);
            },
            Some(LogicalKey::Delete) => {
                if ui.input().key_down(&Key::COMMAND) {
                    if memory.editor.selection_bounds().is_none() {
                        memory.editor.set_selection(cosmic_text::Selection::Normal(memory.editor.cursor()));
                        memory.editor.action(font_system(ui), cosmic_text::Action::Motion(cosmic_text::Motion::RightWord));
                    }
                }
                memory.editor.action(font_system(ui), cosmic_text::Action::Delete);
            },
            Some(LogicalKey::Enter) => {
                *done_editing = true;
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

}
