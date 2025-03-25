
use project::{Action, Clip, ClipInner, Ptr, RenameClip, SetClipInnerFramerate, SetClipInnerHeight, SetClipInnerLength, SetClipInnerWidth};

use crate::State;

use super::ClipProperties;

pub struct ClipPropertiesDialog {
    pub properties: ClipProperties,
    pub clip_ptr: Ptr<Clip>,
    pub clip_inner_ptr: Ptr<ClipInner>
}

impl pierro::Window for ClipPropertiesDialog {
    type Context = State;

    const UNIQUE: bool = true;

    fn title(&self) -> impl Into<String> {
        "Clip Properties"
    }

    fn render(&mut self, ui: &mut pierro::UI, close: &mut bool, state: &mut State) {
        let Some(clip) = state.project.client.get(self.clip_ptr) else {
            *close = true;
            return;
        };
        let Some(clip_inner) = state.project.client.get(self.clip_inner_ptr) else {
            *close = true;
            return;
        };

        let response = self.properties.render_ui(ui);

        if response.name_response.done_editing {
            state.project.client.queue_action(Action::single(state.editor.action_context("Rename Clip"), RenameClip {
                ptr: self.clip_ptr,
                name: self.properties.name.clone(),
            }));
        } else if !response.name_response.response.is_focused(ui) {
            self.properties.name = clip.name.clone();
        }

        if response.width_response.done_editing {
            state.project.client.queue_action(Action::single(state.editor.action_context("Set Clip Width"), SetClipInnerWidth {
                ptr: self.clip_inner_ptr,
                width_value: self.properties.width,
            }));
        } else if !response.width_response.drag_value.is_focused(ui) {
            self.properties.width = clip_inner.width;
        }

        if response.height_response.done_editing {
            state.project.client.queue_action(Action::single(state.editor.action_context("Set Clip Height"), SetClipInnerHeight {
                ptr: self.clip_inner_ptr,
                height_value: self.properties.height,
            }));
        } else if !response.height_response.drag_value.is_focused(ui) {
            self.properties.height = clip_inner.height;
        }

        if response.length_response.done_editing {
            state.project.client.queue_action(Action::single(state.editor.action_context("Set Clip Length"), SetClipInnerLength {
                ptr: self.clip_inner_ptr,
                length_value: self.properties.length,
            }));
        } else if !response.length_response.drag_value.is_focused(ui) {
            self.properties.length = clip_inner.length;
        }
        
        if response.framerate_changed {
            state.project.client.queue_action(Action::single(state.editor.action_context("Set Clip Framerate"), SetClipInnerFramerate {
                ptr: self.clip_inner_ptr,
                framerate_value: self.properties.framerate,
            }));
        } else {
            self.properties.framerate = clip_inner.framerate;
        }

    }

}
