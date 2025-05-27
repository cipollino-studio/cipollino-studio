
use std::collections::HashMap;

use crate::{AppSystems, Shortcut};

fn shortcut_setting<S: Shortcut>(ui: &mut pierro::UI, systems: &mut AppSystems, shortcut_occurences: &mut HashMap<pierro::KeyboardShortcut, pierro::UIRef>) {
    let mut shortcut = systems.prefs.get::<S>();
    let response = pierro::shortcut::rebindable_shortcut(ui, &mut shortcut);
    if response.changed {
        systems.prefs.set::<S>(&shortcut);
    }

    if let Some(prev_occurence) = shortcut_occurences.insert(shortcut, response.response.node_ref) {
        pierro::error_outline(ui, prev_occurence);
        pierro::error_outline(ui, response.response.node_ref);
    }
}

pub(super) fn shortcuts(ui: &mut pierro::UI, systems: &mut AppSystems) {

    macro_rules! shortcut {
        ($builder: ident, $occurences: ident, $label: literal, $name: ident) => {
            $builder.labeled(concat!($label, ":"), |ui| {
                shortcut_setting::<crate::$name>(ui, systems, &mut $occurences);
            });
        };
    }

    let mut shortcut_occurences = HashMap::new();

    pierro::key_value_layout(ui, |builder| {
        let spacing = 7.5;

        builder.labeled("", |ui| {
            pierro::label(ui, "General");
        });
        shortcut!(builder, shortcut_occurences, "Copy", CopyShortcut);
        shortcut!(builder, shortcut_occurences, "Paste", PasteShortcut);
        shortcut!(builder, shortcut_occurences, "Cut", CutShortcut);
        shortcut!(builder, shortcut_occurences, "Undo", UndoShortcut);
        shortcut!(builder, shortcut_occurences, "Redo", RedoShortcut);
        shortcut!(builder, shortcut_occurences, "Delete", DeleteShortcut);

        builder.spacing(spacing);
        builder.labeled("", |ui| {
            pierro::label(ui, "Playback");
        });
        shortcut!(builder, shortcut_occurences, "Play", PlayShortcut);
        shortcut!(builder, shortcut_occurences, "Step Backward", StepBwdShortcut);
        shortcut!(builder, shortcut_occurences, "Step Forward", StepFwdShortcut);
        shortcut!(builder, shortcut_occurences, "Previous Frame", PrevFrameShortcut);
        shortcut!(builder, shortcut_occurences, "Next Frame", NextFrameShortcut);
        shortcut!(builder, shortcut_occurences, "Jump To Start", JumpToStartShortcut);
        shortcut!(builder, shortcut_occurences, "Jump To End", JumpToEndShortcut);
        shortcut!(builder, shortcut_occurences, "New Keyframe", NewKeyframeShortcut);

        builder.spacing(spacing);
        builder.labeled("", |ui| {
            pierro::label(ui, "Scene");
        });
        shortcut!(builder, shortcut_occurences, "Toggle Onion Skin", ToggleOnionSkinShortcut);
        shortcut!(builder, shortcut_occurences, "Toggle Mirror", MirrorSceneShortcut);
        shortcut!(builder, shortcut_occurences, "Recenter Scene", RecenterSceneShortcut);

        builder.spacing(spacing);
        builder.labeled("", |ui| {
            pierro::label(ui, "Tools");
        });
        shortcut!(builder, shortcut_occurences, "Select", SelectToolShortcut);
        shortcut!(builder, shortcut_occurences, "Pencil", PencilToolShortcut);
        shortcut!(builder, shortcut_occurences, "Eraser", EraserToolShortcut);
        shortcut!(builder, shortcut_occurences, "Bucket", BucketToolShortcut);
        shortcut!(builder, shortcut_occurences, "Color Picker", ColorPickerShortcut);
    });
}
