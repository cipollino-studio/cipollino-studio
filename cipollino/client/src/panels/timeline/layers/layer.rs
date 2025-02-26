
use project::{Action, DeleteLayer, Layer, Ptr};

use crate::{EditorState, ProjectState, TimelinePanel};

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

    fn layer_mouse_interaction(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, layer_response: &pierro::Response, layer: &Layer, layer_ptr: Ptr<Layer>, render_list_idx: usize) {
        pierro::context_menu(ui, layer_response, |ui| {
            self.layer_context_menu(ui, project, layer_ptr); 
        });

        self.layer_dnd_source.source_without_cursor_icon(ui, &layer_response, || LayerSelection::single(layer_ptr));

        self.handle_layer_dropping(ui, &layer_response, render_list_idx);

        if layer_response.mouse_clicked() {
            editor.active_layer = layer_ptr;
        }

        if layer_response.mouse_double_clicked() {
            self.start_rename(&layer.name, layer_ptr);
        }
    }

    pub(super) fn render_layer(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, render_list_idx: usize, layer: &Layer, layer_ptr: Ptr<Layer>) {

        let highlight = layer_ptr == editor.active_layer;
        if highlight {
            let active_text_color = ui.style::<pierro::theme::ActiveTextColor>();
            ui.push_style::<pierro::theme::TextColor>(active_text_color);
        }

        ui.push_id_seed(&layer_ptr);
        let (layer_response, _) = pierro::container(ui, pierro::Size::fr(1.0), pierro::Size::px(Self::LAYER_HEIGHT), pierro::Layout::horizontal().align_center(), |ui| {
            pierro::container(ui, pierro::Size::fr(1.0).with_grow(1.0), pierro::Size::fr(1.0), pierro::Layout::horizontal().align_center().with_horizontal_overflow(), |ui| {

                pierro::h_spacing(ui, 2.0);
                if layer_ptr == editor.active_layer {
                    pierro::icon(ui, pierro::icons::PENCIL);
                } else {
                    pierro::icon_gap(ui);
                }
                pierro::h_spacing(ui, 2.0);

                self.renameable_layer_label(ui, project, &layer.name, layer_ptr); 
            });

            pierro::container(ui, pierro::Size::fit(), pierro::Size::fr(1.0), pierro::Layout::horizontal().align_center(), |ui| {
                pierro::clickable_icon(ui, pierro::icons::EYE);
                pierro::h_spacing(ui, 3.0);
                pierro::clickable_icon(ui, pierro::icons::LOCK);
                pierro::h_spacing(ui, 5.0);
            });
        });

        if highlight {
            ui.pop_style(); // Reset text color to normal

            let accent_color = ui.style::<pierro::theme::AccentColor>();
            ui.set_fill(layer_response.node_ref, accent_color.with_alpha(0.2));
        }

        self.layer_mouse_interaction(ui, project, editor, &layer_response, layer, layer_ptr, render_list_idx); 
    }

}
