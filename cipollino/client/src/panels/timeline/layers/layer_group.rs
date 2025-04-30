
use alisa::Ptr;
use project::{Action, DeleteLayerGroup, LayerGroup};

use crate::{EditorState, ProjectState, TimelinePanel};

use super::LayerList;

impl TimelinePanel {

    fn layer_group_context_menu(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &EditorState, layer_ptr: Ptr<LayerGroup>) {
        if pierro::menu_button(ui, "Delete").mouse_clicked() {
            project.client.queue_action(Action::single(editor.action_context("Delete Layer"), DeleteLayerGroup {
                ptr: layer_ptr,
            }));
        }
    }

    fn layer_group_mouse_interaction(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, layer_response: &pierro::Response, layer_group: &LayerGroup, layer_group_ptr: Ptr<LayerGroup>, render_list_idx: usize) {
        pierro::context_menu(ui, layer_response, |ui| {
            self.layer_group_context_menu(ui, project, &editor, layer_group_ptr); 
        });

        self.layer_dnd_source.source_without_cursor_icon(ui, &layer_response, || LayerList::single(layer_group_ptr));

        self.handle_layer_dropping(ui, &layer_response, render_list_idx);

        if layer_response.mouse_double_clicked() {
            self.start_rename(&layer_group.name, layer_group_ptr);
        }
    }

    pub(super) fn render_layer_group(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, render_list_idx: usize, depth: i32, layer_group: &LayerGroup, layer_group_ptr: Ptr<LayerGroup>) {
        
        ui.push_id_seed(&layer_group_ptr);
        let (layer_response, _) = pierro::container(ui, pierro::Size::fr(1.0), pierro::Size::px(Self::LAYER_HEIGHT), pierro::Layout::horizontal().align_center(), |ui| {
            pierro::container(ui, pierro::Size::fr(1.0).with_grow(1.0), pierro::Size::fr(1.0), pierro::Layout::horizontal().align_center().with_horizontal_overflow(), |ui| {

                self.layer_depth_spacing(ui, depth);

                pierro::h_spacing(ui, 2.0);
                let icon = if editor.open_layer_groups.contains(&layer_group_ptr) {
                    pierro::icons::CARET_DOWN
                } else {
                    pierro::icons::CARET_RIGHT
                };
                if pierro::clickable_icon(ui, icon).mouse_clicked() {
                    if !editor.open_layer_groups.remove(&layer_group_ptr) {
                        editor.open_layer_groups.insert(layer_group_ptr);
                    }
                }
                pierro::h_spacing(ui, 2.0);

                self.renameable_layer_label(ui, project, &editor, &layer_group.name, layer_group_ptr); 
            });

        });

        self.layer_group_mouse_interaction(ui, project, editor, &layer_response, layer_group, layer_group_ptr, render_list_idx);

    }

}
