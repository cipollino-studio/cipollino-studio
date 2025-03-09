
use crate::AppSystems;
use super::Editor;

impl Editor {

    pub fn use_shortcuts(&mut self, ui: &mut pierro::UI, _systems: &mut AppSystems) {

        let play_shortcut = pierro::KeyboardShortcut::new(pierro::KeyModifiers::empty(), pierro::Key::SPACE);
        if play_shortcut.used_globally(ui) {
            self.state.editor.playing = !self.state.editor.playing;
        }

        let undo_shortcut = pierro::KeyboardShortcut::new(pierro::KeyModifiers::COMMAND, pierro::Key::text("z"));
        let redo_shortcut = pierro::KeyboardShortcut::new(pierro::KeyModifiers::COMMAND | pierro::KeyModifiers::SHIFT, pierro::Key::text("z"));
        
        if undo_shortcut.used_globally(ui) {
            self.state.project.client.undo();
        }
        if redo_shortcut.used_globally(ui) {
            self.state.project.client.redo();
        }

    }

}
