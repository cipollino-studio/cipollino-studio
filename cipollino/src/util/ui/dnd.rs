
use egui::{Color32, InnerResponse, LayerId, Order, Stroke};

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

pub struct DndDropZoneSetupColors {
    inactive_bg_fill: Color32,
    inactive_bg_stroke: Stroke,
    active_bg_fill: Color32,
    active_bg_stroke: Stroke 
}

pub fn dnd_drop_zone_setup_colors(ui: &mut egui::Ui) -> DndDropZoneSetupColors {
    let style = ui.style_mut();
    DndDropZoneSetupColors {
        inactive_bg_fill: std::mem::replace(&mut style.visuals.widgets.inactive.bg_fill, style.visuals.window_fill),
        inactive_bg_stroke: std::mem::replace(&mut style.visuals.widgets.inactive.bg_stroke, egui::Stroke::NONE),
        active_bg_fill: std::mem::replace(&mut style.visuals.widgets.active.bg_fill, style.visuals.window_fill),
        active_bg_stroke: std::mem::replace(&mut style.visuals.widgets.active.bg_stroke, egui::Stroke::NONE),
    }
}

pub fn dnd_drop_zone_reset_colors(ui: &mut egui::Ui, colors: DndDropZoneSetupColors) {
    let style = ui.style_mut();
    style.visuals.widgets.inactive.bg_fill = colors.inactive_bg_fill;
    style.visuals.widgets.inactive.bg_stroke = colors.inactive_bg_stroke;
    style.visuals.widgets.active.bg_fill = colors.active_bg_fill;
    style.visuals.widgets.active.bg_stroke = colors.active_bg_stroke;
}
