
use crate::EditorState;

use super::Panel;

#[derive(Default)]
pub struct AssetsPanel {

}

impl Panel for AssetsPanel {

    fn title(&self) -> String {
        "Assets".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut EditorState) {
         
    }

}
