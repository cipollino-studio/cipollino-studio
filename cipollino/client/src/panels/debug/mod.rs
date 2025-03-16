
use project::Action;

use crate::State;

use super::Panel;
 
#[derive(Default)]
pub struct DebugPanel {

}

impl DebugPanel {

    fn action_info(&self, ui: &mut pierro::UI, action: &Action) {
        pierro::collapsing_label(ui, &action.context.name, |ui| {
            for operation in action.iter_operations() {
                pierro::label(ui, operation.name());
                let debug_info = operation.debug_info();
                if !debug_info.is_empty() {
                    pierro::label(ui, format!("=> {}", debug_info));
                }
            }  
        });
    }

}

impl Panel for DebugPanel {
    const NAME: &'static str = "Debug";

    fn title(&self) -> String {
        "Debug".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut State) {
        let undo = state.project.client.undo_stack();
        pierro::collapsing_label(ui, format!("Undo: {}", undo.len()), |ui| {
            for action in undo.iter().rev() {
                self.action_info(ui, action);
            }
        });

        let redo = state.project.client.redo_stack();
        pierro::collapsing_label(ui, format!("Redo: {}", redo.len()), |ui| {
            for action in redo.iter().rev() {
                self.action_info(ui, action);
            }
        });
    }
}
