
use project::{Action, Client, Clip, ClipTreeData, CreateClip, CreateLayer, LayerParent, LayerTreeData, Ptr};

use crate::{EditorState, State};

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

    fn create_clip(&self, client: &Client, editor: &EditorState) -> Option<Ptr<Clip>> {
        let clip_ptr = client.next_ptr()?;
        let inner_ptr = client.next_ptr()?;
        let layer_ptr = client.next_ptr()?;

        let mut action = Action::new(editor.action_context("Create Clip"));
        action.push(CreateClip {
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
        });
        action.push(CreateLayer {
            ptr: layer_ptr,
            parent: LayerParent::Clip(clip_ptr),
            idx: 0,
            data: LayerTreeData {
                name: "Layer".to_string(),
                ..Default::default()
            },
        });

        client.queue_action(action);
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
            if pierro::button(ui, "Create Clip").mouse_clicked() {
                if let Some(clip) = self.create_clip(&state.project.client, &state.editor) {
                    state.editor.open_clip(clip);
                }
                *close = true;
            }
        });
        
    }

}
