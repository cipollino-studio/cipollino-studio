
use crate::AppSystems;
use super::Editor;

impl Editor {

    pub fn use_shortcuts(&mut self, ui: &mut pierro::UI, _systems: &mut AppSystems) {

        let play_shortcut = pierro::KeyboardShortcut::new(pierro::KeyModifiers::empty(), pierro::Key::Space);
        if play_shortcut.used_globally(ui) {
            self.state.editor.playing = !self.state.editor.playing;
        }

        let undo_shortcut = pierro::KeyboardShortcut::new(pierro::KeyModifiers::CONTROL, pierro::Key::Z);
        let redo_shortcut = pierro::KeyboardShortcut::new(pierro::KeyModifiers::CONTROL | pierro::KeyModifiers::SHIFT, pierro::Key::Z);
        
        if undo_shortcut.used_globally(ui) {
            self.state.editor.will_undo = true; 
        }
        if redo_shortcut.used_globally(ui) {
            self.state.editor.will_redo = true; 
        }

    }

}
