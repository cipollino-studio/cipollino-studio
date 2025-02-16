
use crate::EditorState;

use super::Panel;

#[derive(Default)]
pub struct ScenePanel {

}

impl Panel for ScenePanel {

    fn title(&self) -> String {
        "Scene".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, _state: &mut EditorState) {
        pierro::label(ui, "Scene!");
    }

}