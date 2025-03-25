
use project::ActionContext;

use super::{Editor, EditorState};

impl EditorState {

    pub fn action_context<S: Into<String>>(&self, name: S) -> ActionContext {
        ActionContext::new(name, self.open_clip, self.time)
    }

}

impl Editor {

    pub(super) fn tick_undo_redo(&mut self) {
        if self.state.editor.will_undo {
            if let Some(context) = self.state.project.client.undo() {
                self.state.editor.open_clip = context.open_clip;
                self.state.editor.jump_to(context.time);
            }
            self.state.editor.will_undo = false;
        }

        if self.state.editor.will_redo {
            if let Some(context) = self.state.project.client.redo() {
                self.state.editor.open_clip = context.open_clip;
                self.state.editor.jump_to(context.time);
            }
            self.state.editor.will_redo = false;
        }
    }

}
