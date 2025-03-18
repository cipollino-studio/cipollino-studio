
use project::{Action, ActionContext, Client, Clip, ClipTreeData, CreateClip, Ptr};

use crate::State;

use super::ClipProperties;

pub struct CreateClipDialog {
    data: ClipProperties
}

impl CreateClipDialog {

    pub fn new() -> Self {
        Self {
            data: ClipProperties::new()
        }
    }

    fn create_clip(&self, client: &Client) -> Option<Ptr<Clip>> {
        let clip_ptr = client.next_ptr()?;
        let inner_ptr = client.next_ptr()?;
        client.queue_action(Action::single(ActionContext::new("Create Clip"), CreateClip {
            ptr: clip_ptr,
            parent: Ptr::null(),
            data: ClipTreeData {
                name: self.data.name.clone(),
                length: self.data.length,
                framerate: self.data.framerate,
                width: self.data.width,
                height: self.data.height,
                inner_ptr,
                ..Default::default()
            },
        }));
        Some(clip_ptr)
    }

}

impl pierro::Window for CreateClipDialog {

    type Context = State;

    const UNIQUE: bool = true;

    fn title(&self) -> impl Into<String> {
        "Create Clip" 
    }

    fn render(&mut self, ui: &mut pierro::UI, close: &mut bool, state: &mut State) {
        self.data.render_ui(ui);
        pierro::v_spacing(ui, 5.0);
        pierro::vertical_centered(ui, |ui| {
            if pierro::button(ui, "Create Dialog").mouse_clicked() {
                if let Some(clip) = self.create_clip(&state.project.client) {
                    state.editor.open_clip(clip);
                }
                *close = true;
            }
        });
        
    }

}
