
use cipollino_project::{crdt::fractional_index::FractionalIndex, project::{layer::Layer, obj::ObjPtr}};

use crate::{editor::EditorState, util::ui::dnd::{dnd_drop_zone, draggable_widget}};

use super::{Timeline, TimelineCommand, TimelineLayer, TimelineRenderInfo};

impl Timeline {

    pub(super) fn render_layers(&mut self, ui: &mut egui::Ui, state: &EditorState, info: &mut TimelineRenderInfo) {
        let scroll_resp = egui::ScrollArea::vertical()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .scroll_offset(egui::vec2(0.0, info.y_scroll)) // Always scroll to the current scroll y to sync with other scroll areas
            .show(ui, |ui| {
                ui.style_mut().spacing.item_spacing = egui::Vec2::ZERO;
                let mut drop_idx = None;
                if let (_, Some(layer)) = dnd_drop_zone::<ObjPtr<Layer>, ()>(ui, |ui| {
                    for layer in &info.layers {
                        self.render_layer(ui, state, layer, info.layer_h, &mut info.commands, &mut drop_idx);
                    }
                }) {
                    if let Some(idx) = drop_idx {
                        info.commands.push(TimelineCommand::TransferLayer(*layer, idx));
                    }
                }
            });
        
        // If the user hovered over the layer scroll area, update the scroll y based on the scroll area
        if ui.input(|i| i.pointer.hover_pos().map(|pos| scroll_resp.inner_rect.contains(pos)).unwrap_or(false)) {
            info.set_y_scroll = Some(scroll_resp.state.offset.y);
            
        }
    }

    fn render_layer(&mut self, ui: &mut egui::Ui, state: &EditorState, layer: &TimelineLayer, layer_h: f32, commands: &mut Vec<TimelineCommand>, drop_idx: &mut Option<FractionalIndex>) {
        draggable_widget(ui, layer.layer.ptr(), |ui, dragged| {
            let (rect, resp) = ui.allocate_exact_size(egui::vec2(ui.available_width(), layer_h), egui::Sense::click());

            // If this is the active layer, highlight it
            if layer.layer.ptr() == state.active_layer {
                ui.painter().rect_filled(rect, egui::Rounding::ZERO, ui.visuals().widgets.active.bg_fill);
            }

            // Eye/lock icons
            let icon_area_w = 32.0;
            let icon_rect = rect.with_min_x(rect.max.x - icon_area_w);
            let mut icon_ui = ui.child_ui(icon_rect, egui::Layout::left_to_right(egui::Align::Center));
            icon_ui.style_mut().spacing.item_spacing = egui::Vec2::splat(2.0);

            let eye_text = if *layer.layer.hide.value() { egui_phosphor::regular::EYE_CLOSED } else { egui_phosphor::regular::EYE };
            if icon_ui.add(egui::Label::new(eye_text).selectable(false).sense(egui::Sense::click())).clicked() {
                commands.push(TimelineCommand::HideLayer(layer.layer.ptr()));
            }

            let lock_text = if *layer.layer.lock.value() { egui_phosphor::regular::LOCK_KEY } else { egui_phosphor::regular::LOCK_SIMPLE_OPEN };
            if icon_ui.add(egui::Label::new(lock_text).selectable(false).sense(egui::Sense::click())).clicked() {
                commands.push(TimelineCommand::LockLayer(layer.layer.ptr()));
            }

            // Layer name
            let name_rect = rect.with_max_x(rect.max.x - icon_area_w);
            let mut name_ui = ui.child_ui(name_rect, egui::Layout::top_down(egui::Align::LEFT).with_main_justify(true));
            name_ui.add(egui::Label::new(layer.layer.name.value()).selectable(false).truncate(true));

            // Drop zone
            if !dragged {
                self.handle_layer_drop_zone(ui, layer, &rect, &resp, drop_idx);
            }

            // Interaction
            if resp.clicked() {
                commands.push(TimelineCommand::SetActiveLayer(layer.layer.ptr()));
            }

            ((), resp)
        });
    }

    fn handle_layer_drop_zone(&mut self, ui: &mut egui::Ui, layer: &TimelineLayer, rect: &egui::Rect, resp: &egui::Response, drop_idx: &mut Option<FractionalIndex>) {
        let Some(hover_pos) = ui.input(|i| i.pointer.hover_pos()) else { return; }; 
        if !rect.contains(hover_pos) {
            return;
        }

        let render_line = resp.dnd_hover_payload::<ObjPtr<Layer>>().is_some();

        let insert_line_stroke = egui::Stroke {
            width: 3.0,
            color: ui.visuals().strong_text_color(),
        };
        if hover_pos.y < rect.center().y { // Drop above
            if render_line {
                ui.painter().hline(rect.x_range(), rect.top(), insert_line_stroke);
            }
            *drop_idx = Some(layer.drop_top_idx.clone());
        } else { // Drop below
            if render_line {
                ui.painter().hline(rect.x_range(), rect.bottom(), insert_line_stroke);
            }
            *drop_idx = Some(layer.drop_bottom_idx.clone());
        };

    }

}
