
use project::{Action, DeleteLayer, Layer, Ptr};

use crate::{ProjectState, TimelinePanel};

use super::LayerSelection;

impl TimelinePanel {

    fn layer_context_menu(&mut self, ui: &mut pierro::UI, project: &ProjectState, layer_ptr: Ptr<Layer>) {
        if pierro::menu_button(ui, "Delete").mouse_clicked() {
            let mut action = Action::new();
            project.client.perform(&mut action, DeleteLayer {
                ptr: layer_ptr,
            });
            project.undo_redo.add(action);
        }
    }

    pub(super) fn render_layer(&mut self, ui: &mut pierro::UI, project: &ProjectState, render_list_idx: usize, layer: &Layer, layer_ptr: Ptr<Layer>) {

        ui.push_id_seed(&layer_ptr);
        let (layer_response, _) = pierro::container(ui, pierro::Size::fr(1.0), pierro::Size::px(Self::LAYER_HEIGHT), pierro::Layout::horizontal().align_center(), |ui| {
            pierro::container(ui, pierro::Size::fr(1.0).with_grow(1.0), pierro::Size::fr(1.0), pierro::Layout::horizontal().align_center().with_horizontal_overflow(), |ui| {
                pierro::h_spacing(ui, 7.0);
                let layer_name = self.renameable_layer_label(ui, project, &layer.name, layer_ptr); 
                ui.set_sense_mouse(layer_name.node_ref, true);

                if layer_name.mouse_double_clicked() {
                    self.start_rename(&layer.name, layer_ptr);
                }
                
                // We want to be able to interact with the layer even when clicking the layer name 
                pierro::context_menu(ui, &layer_name, |ui| {
                    self.layer_context_menu(ui, project, layer_ptr);
                });
                self.layer_dnd_source.source_without_cursor_icon(ui, &layer_name, || LayerSelection::single(layer_ptr));
            });

            pierro::container(ui, pierro::Size::fit(), pierro::Size::fr(1.0), pierro::Layout::horizontal().align_center(), |ui| {
                pierro::clickable_icon(ui, pierro::icons::EYE);
                pierro::h_spacing(ui, 3.0);
                pierro::clickable_icon(ui, pierro::icons::LOCK);
                pierro::h_spacing(ui, 5.0);
            });
        });

        pierro::context_menu(ui, &layer_response, |ui| {
            self.layer_context_menu(ui, project, layer_ptr); 
        });

        self.layer_dnd_source.source_without_cursor_icon(ui, &layer_response, || LayerSelection::single(layer_ptr));
        self.handle_layer_dropping(ui, &layer_response, render_list_idx);
    }

}
