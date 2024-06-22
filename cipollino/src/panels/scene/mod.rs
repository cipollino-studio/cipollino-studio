
use crate::app::{editor::EditorState, AppSystems};

use super::Panel;

#[derive(Default)]
pub struct Scene {

}

impl Panel for Scene {
    
    fn ui(&mut self, ui: &mut egui::Ui, state: &mut EditorState, systems: &mut AppSystems) {
        ui.label("Scene!!!");        
    }

}
