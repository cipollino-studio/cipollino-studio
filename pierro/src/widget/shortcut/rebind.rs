
use crate::{empty_button, label, Response, UI};

use super::{shortcut_label, KeyboardShortcut};

pub struct RebindableShortcutResponse {
    pub response: Response,
    pub changed: bool
}

pub fn rebindable_shortcut(ui: &mut UI, shortcut: &mut KeyboardShortcut) -> RebindableShortcutResponse {
    let button = empty_button(ui);

    let focused = button.is_focused(ui);
    ui.with_parent(button.node_ref, |ui| {
        if focused {
            label(ui, "Enter Shortcut...");
        } else {
            shortcut_label(ui, *shortcut);
        }
    });

    if button.mouse_clicked() {
        button.request_focus(ui);
    }

    let mut changed = false;
    if focused {
        if let Some(key) = ui.input().keys_pressed.first() {
            *shortcut = KeyboardShortcut::new(ui.input().key_modifiers, *key);
            button.release_focus(ui);
            changed = true;
        }
    }

    RebindableShortcutResponse { response: button, changed }
}
