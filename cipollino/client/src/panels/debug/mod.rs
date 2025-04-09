
use project::Action;

use super::{Panel, PanelContext};
 
#[derive(Default)]
pub struct DebugPanel {
    request_redraw: bool
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

    fn render(&mut self, ui: &mut pierro::UI, context: &mut PanelContext) {
        pierro::checkbox_labeled(ui, "Request redraw", &mut self.request_redraw);
        if self.request_redraw {
            ui.request_redraw();
        }
        pierro::label(ui, format!("DT: {}", ui.input().delta_time));
        pierro::label(ui, format!("FPS: {}", 1.0 / ui.input().delta_time));
        pierro::v_spacing(ui, 10.0);

        if context.project.client.is_collab() {
            pierro::checkbox_labeled(ui, "Send collab messages", &mut context.editor.send_messages);
            pierro::checkbox_labeled(ui, "Receive collab messages", &mut context.editor.receive_messages);
            pierro::v_spacing(ui, 10.0);
        }

        let undo = context.project.client.undo_stack().borrow();
        pierro::collapsing_label(ui, format!("Undo: {}", undo.len()), |ui| {
            for action in undo.iter().rev() {
                self.action_info(ui, action);
            }
        });

        let redo = context.project.client.redo_stack().borrow();
        pierro::collapsing_label(ui, format!("Redo: {}", redo.len()), |ui| {
            for action in redo.iter().rev() {
                self.action_info(ui, action);
            }
        });
    }
}
