
use project::{Action, AudioInstance, CreateFrame, DeleteAudioInstance, DeleteFill, DeleteFrame, DeleteStroke, Fill, Frame, FrameTreeData, Stroke};

use crate::{keyboard_shortcut, AppSystems, Shortcut};
use super::{EditorState, LayerRenderList, ProjectState, SceneRenderList};

keyboard_shortcut!(CopyShortcut, C, pierro::KeyModifiers::CONTROL);
keyboard_shortcut!(PasteShortcut, V, pierro::KeyModifiers::CONTROL);
keyboard_shortcut!(CutShortcut, X, pierro::KeyModifiers::CONTROL);
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

    fn delete(&mut self, project: &ProjectState) {
        let mut action = Action::new(self.action_context("Delete"));
        for frame in self.selection.iter::<Frame>() {
            action.push(DeleteFrame {
                ptr: frame,
            });
        }
        for audio_instance in self.selection.iter::<AudioInstance>() {
            action.push(DeleteAudioInstance {
                ptr: audio_instance,
            });
        }
        for stroke in self.selection.iter::<Stroke>() {
            action.push(DeleteStroke {
                ptr: stroke,
            });
        }
        for fill in self.selection.iter::<Fill>() {
            action.push(DeleteFill {
                ptr: fill,
            });
        }
        project.client.queue_action(action);
    }

    pub fn use_shortcuts(&mut self, project: &ProjectState, layer_render_list: Option<&LayerRenderList>, scene_render_list: Option<&SceneRenderList>, ui: &mut pierro::UI, systems: &mut AppSystems) {

        let clip = project.client.get(self.open_clip).and_then(|clip| project.client.get(clip.inner));

        if CopyShortcut::used_globally(ui, systems) {
            if let Some(clipboard) = self.selection.collect_clipboard(&project.client, self, layer_render_list, clip, scene_render_list) {
                self.clipboard = Some(clipboard);
            }
        }
        if PasteShortcut::used_globally(ui, systems) {
            if let Some(clipboard) = &self.clipboard {
                self.next_selection = clipboard.paste(&project.client, &self, clip, layer_render_list);
            }
        }
        if CutShortcut::used_globally(ui, systems) {
            if let Some(clipboard) = self.selection.collect_clipboard(&project.client, self, layer_render_list, clip, scene_render_list) {
                self.clipboard = Some(clipboard);
            }
            self.delete(project);
        }
        if UndoShortcut::used_globally(ui, systems) {
            self.will_undo = true; 
        }
        if RedoShortcut::used_globally(ui, systems) {
            self.will_redo = true; 
        }
        if DeleteShortcut::used_globally(ui, systems) {
            self.delete(project);
        }

        self.playback_shortcuts(project, ui, systems);

        if ToggleOnionSkinShortcut::used_globally(ui, systems) {
            self.show_onion_skin = !self.show_onion_skin;
        }

    }

}
