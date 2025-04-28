
use project::{Action, CreateFrame, FrameTreeData};

use crate::{keyboard_shortcut, AppSystems, Shortcut};
use super::Editor;

keyboard_shortcut!(UndoShortcut, Z, pierro::KeyModifiers::CONTROL);
keyboard_shortcut!(RedoShortcut, Y, pierro::KeyModifiers::CONTROL);
keyboard_shortcut!(DeleteShortcut, Backspace, pierro::KeyModifiers::empty());

keyboard_shortcut!(PlayShortcut, Space, pierro::KeyModifiers::empty());
keyboard_shortcut!(StepFwdShortcut, Period, pierro::KeyModifiers::empty());
keyboard_shortcut!(StepBwdShortcut, Comma, pierro::KeyModifiers::empty());
keyboard_shortcut!(NextFrameShortcut, Period, pierro::KeyModifiers::SHIFT);
keyboard_shortcut!(PrevFrameShortcut, Comma, pierro::KeyModifiers::SHIFT);
keyboard_shortcut!(JumpToEndShortcut, Period, pierro::KeyModifiers::SHIFT | pierro::KeyModifiers::CONTROL);
keyboard_shortcut!(JumpToStartShortcut, Comma, pierro::KeyModifiers::SHIFT | pierro::KeyModifiers::CONTROL);
keyboard_shortcut!(NewKeyframeShortcut, K, pierro::KeyModifiers::empty());

keyboard_shortcut!(ToggleOnionSkinShortcut, O, pierro::KeyModifiers::SHIFT);

impl Editor {

    fn playback_shortcuts(&mut self, ui: &mut pierro::UI, systems: &mut AppSystems) {
        let Some(clip) = self.state.project.client.get(self.state.editor.open_clip) else { return; };
        let Some(clip) = self.state.project.client.get(clip.inner) else { return; };

        if PlayShortcut::used_globally(ui, systems) {
            self.state.editor.playing = !self.state.editor.playing;
        }

        let frame_idx = clip.frame_idx(self.state.editor.time);
        if StepBwdShortcut::used_globally(ui, systems) {
            self.state.editor.jump_to((frame_idx as f32 - 0.5) * clip.frame_len());
        }
        if StepFwdShortcut::used_globally(ui, systems) {
            self.state.editor.jump_to((frame_idx as f32 + 1.5) * clip.frame_len());
        }
        if NextFrameShortcut::used_globally(ui, systems) {
            self.state.editor.jump_to_next_frame(&self.state.project.client, clip);
        }
        if PrevFrameShortcut::used_globally(ui, systems) {
            self.state.editor.jump_to_prev_frame(&self.state.project.client, clip);
        }
        if JumpToEndShortcut::used_globally(ui, systems) {
            self.state.editor.jump_to((clip.length as f32 - 0.5) * clip.frame_len());
        }
        if JumpToStartShortcut::used_globally(ui, systems) {
            self.state.editor.jump_to(0.0);
        }

        if NewKeyframeShortcut::used_globally(ui, systems) {
            if !self.state.editor.locked_layers.contains(&self.state.editor.active_layer) {
                self.state.editor.playing = false;
                self.state.project.client.queue_action(Action::single(self.state.editor.action_context("New Frame"), CreateFrame {
                    ptr: self.state.project.client.next_ptr(),
                    layer: self.state.editor.active_layer,
                    data: FrameTreeData {
                        time: clip.frame_idx(self.state.editor.time),
                        ..Default::default()
                    },
                }));
            }
        }
    }

    pub fn use_shortcuts(&mut self, ui: &mut pierro::UI, systems: &mut AppSystems) {

        if UndoShortcut::used_globally(ui, systems) {
            self.state.editor.will_undo = true; 
        }
        if RedoShortcut::used_globally(ui, systems) {
            self.state.editor.will_redo = true; 
        }

        self.playback_shortcuts(ui, systems);

        if ToggleOnionSkinShortcut::used_globally(ui, systems) {
            self.state.editor.show_onion_skin = !self.state.editor.show_onion_skin;
        }

    }

}
