
use project::{Action, CreateFrame, FrameTreeData};

use crate::{keyboard_shortcut, AppSystems, Shortcut};
use super::{EditorState, LayerRenderList, ProjectState, SceneRenderList};

keyboard_shortcut!(CopyShortcut, C, pierro::KeyModifiers::CONTROL);
keyboard_shortcut!(PasteShortcut, V, pierro::KeyModifiers::CONTROL);
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

impl EditorState {

    fn playback_shortcuts(&mut self, project: &ProjectState, ui: &mut pierro::UI, systems: &mut AppSystems) {
        let Some(clip) = project.client.get(self.open_clip) else { return; };
        let Some(clip) = project.client.get(clip.inner) else { return; };

        if PlayShortcut::used_globally(ui, systems) {
            self.playing = !self.playing;
        }

        let frame_idx = clip.frame_idx(self.time);
        if StepBwdShortcut::used_globally(ui, systems) {
            self.jump_to((frame_idx as f32 - 0.5) * clip.frame_len());
        }
        if StepFwdShortcut::used_globally(ui, systems) {
            self.jump_to((frame_idx as f32 + 1.5) * clip.frame_len());
        }
        if NextFrameShortcut::used_globally(ui, systems) {
            self.jump_to_next_frame(&project.client, clip);
        }
        if PrevFrameShortcut::used_globally(ui, systems) {
            self.jump_to_prev_frame(&project.client, clip);
        }
        if JumpToEndShortcut::used_globally(ui, systems) {
            self.jump_to((clip.length as f32 - 0.5) * clip.frame_len());
        }
        if JumpToStartShortcut::used_globally(ui, systems) {
            self.jump_to(0.0);
        }

        if NewKeyframeShortcut::used_globally(ui, systems) {
            if !self.locked_layers.contains(&self.active_layer) {
                self.playing = false;
                project.client.queue_action(Action::single(self.action_context("New Frame"), CreateFrame {
                    ptr: project.client.next_ptr(),
                    layer: self.active_layer,
                    data: FrameTreeData {
                        time: clip.frame_idx(self.time),
                        ..Default::default()
                    },
                }));
            }
        }
    }

    pub fn use_shortcuts(&mut self, project: &ProjectState, layer_render_list: Option<&LayerRenderList>, scene_render_list: Option<&SceneRenderList>, ui: &mut pierro::UI, systems: &mut AppSystems) {

        if CopyShortcut::used_globally(ui, systems) {
            if let Some(clipboard) = self.selection.collect_clipboard(&project.client, self, layer_render_list, scene_render_list) {
                self.clipboard = Some(clipboard);
            }
        }
        if PasteShortcut::used_globally(ui, systems) {
            if let Some(clipboard) = &self.clipboard {
                self.next_selection = clipboard.paste(&project.client, &self, layer_render_list);
            }
        }
        if UndoShortcut::used_globally(ui, systems) {
            self.will_undo = true; 
        }
        if RedoShortcut::used_globally(ui, systems) {
            self.will_redo = true; 
        }

        self.playback_shortcuts(project, ui, systems);

        if ToggleOnionSkinShortcut::used_globally(ui, systems) {
            self.show_onion_skin = !self.show_onion_skin;
        }

    }

}
