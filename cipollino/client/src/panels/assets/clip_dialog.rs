
use pierro::vertical_centered;
use project::{Action, ActionContext, Client, Clip, ClipTreeData, CreateClip, Ptr};

use crate::State;

pub(super) struct ClipDialog {
    name: String,
    length: u32,
    framerate: f32
}

impl ClipDialog {

    pub fn new() -> Self {
        Self {
            name: "Clip".to_owned(),
            length: 100,
            framerate: 24.0
        }
    }

    fn create_clip(&self, client: &Client) -> Option<Ptr<Clip>> {
        let clip_ptr = client.next_ptr()?;
        let inner_ptr = client.next_ptr()?;
        client.queue_action(Action::single(ActionContext::new("Create Clip"), CreateClip {
            ptr: clip_ptr,
            parent: Ptr::null(),
            data: ClipTreeData {
                name: self.name.clone(),
                length: self.length,
                framerate: self.framerate,
                inner_ptr,
                ..Default::default()
            },
        }));
        Some(clip_ptr)
    }

}

fn labeled<F: FnOnce(&mut pierro::UI)>(ui: &mut pierro::UI, label: &str, contents: F) {
    pierro::horizontal_fit_centered(ui, |ui| {
        pierro::container(ui, pierro::Size::px(60.0), pierro::Size::fit(), pierro::Layout::horizontal().justify_max(), |ui| {
            pierro::label(ui, label);
        });

        pierro::h_spacing(ui, 4.0);

        contents(ui);
    });
}

impl pierro::Window for ClipDialog {

    type Context = State;

    const UNIQUE: bool = true;

    fn title(&self) -> impl Into<String> {
        "Create Clip" 
    }

    fn render(&mut self, ui: &mut pierro::UI, close: &mut bool, state: &mut State) {
        
        labeled(ui, "Name:", |ui| {
            pierro::text_edit(ui, &mut self.name);
        });
        labeled(ui, "Length:", |ui| {
            pierro::drag_value(ui, &mut self.length);
        });
        labeled(ui, "FPS:", |ui| {
            pierro::dropdown(ui, format!("{}", self.framerate), |ui| {
                if pierro::menu_button(ui, "24").mouse_clicked() {
                    self.framerate = 24.0;
                }
            });
        });

        vertical_centered(ui, |ui| {
            if pierro::button(ui, "Create Dialog").mouse_clicked() {
                if let Some(clip) = self.create_clip(&state.project.client) {
                    state.editor.open_clip(clip);
                }
                *close = true;
            }
        });
        
    }

}
