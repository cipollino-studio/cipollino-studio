
use cosmic_text::Edit;

use crate::{Key, KeyModifiers, KeyboardShortcut, Response, UI};

use super::{font_system, TextEditMemory};

const COPY: KeyboardShortcut = KeyboardShortcut::new(KeyModifiers::CONTROL, Key::C);
const PASTE: KeyboardShortcut = KeyboardShortcut::new(KeyModifiers::CONTROL, Key::V);
const CUT: KeyboardShortcut = KeyboardShortcut::new(KeyModifiers::CONTROL, Key::X);
const ALL: KeyboardShortcut = KeyboardShortcut::new(KeyModifiers::CONTROL, Key::A);

pub(super) fn text_edit_keyboard_input(ui: &mut UI, text_edit: &Response, memory: &mut TextEditMemory, done_editing: &mut bool) {

    ui.request_ime(text_edit.node_ref);

    if COPY.used(ui, text_edit) {
        if let Some(text) = memory.editor.copy_selection() {
            ui.set_clipboard_text(text);
        }
    } else if PASTE.used(ui, text_edit) {
        for char in ui.get_clipboard_text().unwrap_or(String::new()).chars() {
            memory.editor.action(font_system(ui), cosmic_text::Action::Insert(char));
        }
    } else if CUT.used(ui, text_edit) {
        if let Some(text) = memory.editor.copy_selection() {
            ui.set_clipboard_text(text);
        }
        memory.editor.delete_selection();
    } else if ALL.used(ui, text_edit) {
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
        for char in ui.input().text.clone().chars() {
            memory.editor.action(font_system(ui), cosmic_text::Action::Insert(char));
        }
    }

    for key in ui.input().keys_pressed.clone() {
        
        match key {
            Key::Space => {
                memory.editor.action(font_system(ui), cosmic_text::Action::Insert(' '));
            },
            Key::ArrowLeft | Key::Home => {
                if !ui.input().key_modifiers.contains(KeyModifiers::SHIFT) {
                    if let Some((min, _)) = memory.editor.selection_bounds() {
                        memory.editor.set_cursor(min);
                    }
                    memory.editor.set_selection(cosmic_text::Selection::None);
                } else {
                    if memory.editor.selection_bounds().is_none() {
                        memory.editor.set_selection(cosmic_text::Selection::Normal(memory.editor.cursor()));
                    }
                }

                let motion = if key == Key::Home {
                    cosmic_text::Motion::Home
                } else if ui.input().key_modifiers.contains(KeyModifiers::CONTROL) {
                    cosmic_text::Motion::LeftWord
                } else {
                    cosmic_text::Motion::Left
                };

                memory.editor.action(font_system(ui), cosmic_text::Action::Motion(motion));
            },
            Key::ArrowRight | Key::End => {
                if !ui.input().key_modifiers.contains(KeyModifiers::SHIFT) {
                    if let Some((_, max)) = memory.editor.selection_bounds() {
                        memory.editor.set_cursor(max);
                    }
                    memory.editor.set_selection(cosmic_text::Selection::None);
                } else {
                    if memory.editor.selection_bounds().is_none() {
                        memory.editor.set_selection(cosmic_text::Selection::Normal(memory.editor.cursor()));
                    }
                }

                let motion = if key == Key::End {
                    cosmic_text::Motion::End
                } else if ui.input().key_modifiers.contains(KeyModifiers::CONTROL) {
                    cosmic_text::Motion::RightWord
                } else {
                    cosmic_text::Motion::Right
                };

                memory.editor.action(font_system(ui), cosmic_text::Action::Motion(motion));
            },
            Key::Backspace => {
                if ui.input().key_modifiers.contains(KeyModifiers::CONTROL) {
                    if memory.editor.selection_bounds().is_none() {
                        memory.editor.set_selection(cosmic_text::Selection::Normal(memory.editor.cursor()));
                        memory.editor.action(font_system(ui), cosmic_text::Action::Motion(cosmic_text::Motion::LeftWord));
                    }
                }
                memory.editor.action(font_system(ui), cosmic_text::Action::Backspace);
            },
            Key::Delete => {
                if ui.input().key_modifiers.contains(KeyModifiers::CONTROL) {
                    if memory.editor.selection_bounds().is_none() {
                        memory.editor.set_selection(cosmic_text::Selection::Normal(memory.editor.cursor()));
                        memory.editor.action(font_system(ui), cosmic_text::Action::Motion(cosmic_text::Motion::RightWord));
                    }
                }
                memory.editor.action(font_system(ui), cosmic_text::Action::Delete);
            },
            Key::Enter => {
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
