
use crate::{horizontal_fit_centered, icon, icons, label, KeyModifiers, UI};

use super::KeyboardShortcut;

fn key_modifers_text(modifiers: KeyModifiers) -> String {
    let mut result = String::new();
    let mut add = |text| {
        #[cfg(not(target_os = "macos"))]
        if !result.is_empty() {
            result += " + ";
        }
        result += text;
    };

    if modifiers.contains(KeyModifiers::SHIFT) {
        #[cfg(not(target_os = "macos"))]
        add("Shift");
        #[cfg(target_os = "macos")]
        add(icons::ARROW_FAT_UP);
    }
    if modifiers.contains(KeyModifiers::CONTROL) {
        #[cfg(not(target_os = "macos"))]
        add("Ctrl");
        #[cfg(target_os = "macos")]
        add(icons::COMMAND);
    }
    if modifiers.contains(KeyModifiers::OPTION) {
        #[cfg(not(target_os = "macos"))]
        add("Alt");
        #[cfg(target_os = "macos")]
        add(icons::OPTION);
    }

    result
}

pub fn key_modifiers_label(ui: &mut UI, modifiers: KeyModifiers) {
    #[cfg(not(target_os = "macos"))]
    label(ui, key_modifers_text(modifiers));
    #[cfg(target_os = "macos")]
    icon(ui, key_modifers_text(modifiers));
}

pub fn shortcut_label(ui: &mut UI, shortcut: KeyboardShortcut) {
    horizontal_fit_centered(ui, |ui| {
        key_modifiers_label(ui, shortcut.modifiers);
        label(ui, shortcut.key.name());
    });
}
