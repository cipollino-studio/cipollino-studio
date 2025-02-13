
use crate::{AssetsPanel, EditorPanel, ScenePanel};

pub struct EditorState {
    pub client: project::Client,
    pub undo_redo: project::UndoRedoManager
}

pub struct Editor {
    state: EditorState,
    docking: pierro::DockingState<EditorPanel>
}

impl Editor {

    pub fn new(client: project::Client) -> Self {
        Self {
            state: EditorState {
                client,
                undo_redo: project::UndoRedoManager::new(),
            },
            docking: pierro::DockingState::new(vec![
                EditorPanel::new::<ScenePanel>(),
                EditorPanel::new::<AssetsPanel>(),
            ]),
        }
    }

    pub fn tick(&mut self, ui: &mut pierro::UI) {

        pierro::menu_bar(ui, |ui| {
            pierro::menu_bar_item(ui, "File", |ui| {
                if pierro::menu_button(ui, "Open").mouse_clicked() {
                    
                }
            });
            pierro::menu_bar_item(ui, "Edit", |ui| {
                if pierro::menu_button(ui, "Undo").mouse_clicked() {
                    self.state.undo_redo.undo(&self.state.client);
                }
                if pierro::menu_button(ui, "Redo").mouse_clicked() {
                    self.state.undo_redo.undo(&self.state.client);
                }
            });
        });
        self.state.client.tick(&mut ());

        self.docking.render(ui, &mut self.state);
    }

}
