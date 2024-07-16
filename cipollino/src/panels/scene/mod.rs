
use crate::{app::AppSystems, editor::EditorState};

use super::Panel;

#[derive(Default)]
pub struct Scene {

}

impl Panel for Scene {
    
    fn ui(&mut self, ui: &mut egui::Ui, state: &mut EditorState, systems: &mut AppSystems) {
        ui.label("Scene!!!");        
    }

}
