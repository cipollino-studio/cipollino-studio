
use crate::{app::prefs::{UserPref, UserPrefs}, util::ui::keybind::consume_shortcut};

pub trait Keybind : UserPref<Type = egui::KeyboardShortcut> + Sized {

    fn display_name() -> &'static str;

    fn consume(ctx: &egui::Context, user_prefs: &mut UserPrefs) -> bool {
        consume_shortcut(ctx, &user_prefs.get::<Self>()) 
    }

}

#[macro_export]
macro_rules! keybind {
    ($name: ident, $display_name: literal, $modifiers: ident, $key: ident) => {
        pub struct $name; 

        impl UserPref for $name {
            type Type = egui::KeyboardShortcut;

            fn default() -> egui::KeyboardShortcut {
                egui::KeyboardShortcut::new(egui::Modifiers::$modifiers, egui::Key::$key)
            }

            fn name() -> &'static str {
                stringify!($name)
            }
        }

        impl Keybind for $name {

            fn display_name() -> &'static str {
                $display_name
            }

        }
    };
}

keybind!(UndoKeybind, "Undo", COMMAND, Z);
keybind!(RedoKeybind, "Redo", COMMAND, Y);
keybind!(DeleteKeybind, "Delete", NONE, X);

keybind!(PlayKeybind, "Play", NONE, Space);
keybind!(NewFrameKeybind, "New Frame", NONE, K);
keybind!(StepBackKeybind, "Step Back", NONE, Comma);
keybind!(StepForwardKeybind, "Step Forward", NONE, Period);
keybind!(PrevFrameKeybind, "Previous Frame", COMMAND, Comma);
keybind!(NextFrameKeybind, "Next Frame", COMMAND, Period);

keybind!(CenterSceneKeybind, "Center Scene", COMMAND, G);