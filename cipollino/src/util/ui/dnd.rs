
use std::sync::Arc;

use egui::{InnerResponse, LayerId, Order};

use super::NO_MARGIN;

pub fn draggable_label<P>(ui: &mut egui::Ui, text: &str, payload: P) -> egui::Response where P: std::marker::Send + std::marker::Sync + 'static {
    draggable_widget(ui, payload, |ui, _| {
        let label = egui::Label::new(text).selectable(false).sense(egui::Sense::click());
        let resp = ui.add(label);
        (resp.clone(), resp)
    })
}

pub fn draggable_widget<F, P, R>(ui: &mut egui::Ui, payload: P, mut add_contents: F) -> R
    where F: FnMut(&mut egui::Ui, bool) -> (R, egui::Response),
          P: std::marker::Send + std::marker::Sync + 'static {
    
    let id = ui.next_auto_id();
    let dragged = ui.ctx().is_being_dragged(id);
    if dragged {
         
        let layer_id = LayerId::new(Order::Tooltip, ui.next_auto_id());
        let InnerResponse { inner, response } = ui.with_layer_id(layer_id, |ui| {
            add_contents(ui, true)
        });
        
        if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
            let delta = pointer_pos - response.rect.center();
            ui.ctx().transform_layer_shapes(layer_id, egui::emath::TSTransform::from_translation(delta));
        }

        inner.0
    } else {
        let (val, resp) = add_contents(ui, false);
        if resp.is_pointer_button_down_on() && ui.input(|i| i.pointer.delta()).length() > 1.0 {
            ui.ctx().set_dragged_id(id);
            egui::DragAndDrop::set_payload(ui.ctx(), payload);
        } else {
            if ui.ctx().dragged_id() == Some(id) {
                ui.ctx().stop_dragging();
            }
        } 
        val
    }
  
}

pub fn dnd_drop_zone<Payload: Send + Sync + 'static, R>(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> (egui::InnerResponse<R>, Option<Arc<Payload>>) {
    let frame = NO_MARGIN;
    let mut frame = frame.begin(ui);
    let inner = add_contents(&mut frame.content_ui);
    let response = frame.allocate_space(ui);
    frame.paint(ui);
    let payload = response.dnd_release_payload::<Payload>();
    (InnerResponse { inner, response }, payload)
}
