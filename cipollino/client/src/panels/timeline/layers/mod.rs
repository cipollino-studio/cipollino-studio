
use std::collections::HashSet;

use project::{alisa::Object, Action, ClipInner, Layer, LayerGroup, LayerParent, Project, Ptr, SetLayerGroupName, SetLayerName};

use crate::{EditorState, LayerRenderList, ProjectState, RenderLayerKind};

use super::TimelinePanel;

mod layer;
mod layer_group;
mod list;
pub use list::*;

pub trait LayerUI: Object<Project = Project> {

    const ICON: &'static str;
    const NAME: &'static str;

    fn name(&self) -> &String;
    fn rename(action: &mut Action, ptr: Ptr<Self>, name: String);

    fn selection_list(selection: &LayerList) -> &HashSet<Ptr<Self>>;
    fn selection_list_mut(selection: &mut LayerList) -> &mut HashSet<Ptr<Self>>;

}

impl LayerUI for Layer {

    const ICON: &'static str = pierro::icons::FILE;
    const NAME: &'static str = "Layer";

    fn name(&self) -> &String {
        &self.name
    }

    fn rename(action: &mut Action, ptr: Ptr<Self>, name: String) {
        action.push(SetLayerName {
            ptr,
            name_value: name
        });
    }
    
    fn selection_list(selection: &LayerList) -> &HashSet<Ptr<Self>> {
        &selection.layers
    }

    fn selection_list_mut(selection: &mut LayerList) -> &mut HashSet<Ptr<Self>> {
        &mut selection.layers
    }

}

impl LayerUI for LayerGroup {

    const ICON: &'static str = pierro::icons::FOLDER;
    const NAME: &'static str = "Layer Group";

    fn name(&self) -> &String {
        &self.name
    }

    fn rename(action: &mut Action, ptr: Ptr<Self>, name: String) {
        action.push(SetLayerGroupName {
            ptr,
            name_value: name,
        });
    }

    fn selection_list(selection: &LayerList) -> &HashSet<Ptr<Self>> {
        &selection.layer_groups
    }

    fn selection_list_mut(selection: &mut LayerList) -> &mut HashSet<Ptr<Self>> {
        &mut selection.layer_groups
    }

}

#[derive(Clone, Copy)]
pub struct LayerDropLocation {
    pub render_list_idx: usize,
    pub above: bool
}

impl TimelinePanel {

    fn renameable_layer_label<L: LayerUI>(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &EditorState, curr_name: &String, ptr: Ptr<L>) -> pierro::Response {
        
        if let Some((curr_renaming, new_name)) = &mut self.renaming_state {
            if *curr_renaming == ptr.any() { 

                let text_edit = ui.with_style::<pierro::theme::WidgetMargin, _, _>(pierro::Margin::same(2.0), |ui| {
                    pierro::text_edit(ui, new_name)
                });

                if self.started_renaming {
                    self.started_renaming = false;
                    text_edit.response.request_focus(ui);
                }
                if text_edit.done_editing {
                    let mut action = Action::new(editor.action_context(format!("Rename {}", L::NAME)));
                    L::rename(&mut action, ptr, new_name.clone());
                    project.client.queue_action(action);
                    self.renaming_state = None;
                }

                return text_edit.response;
            }
        }

        return pierro::label(ui, curr_name);
    }

    fn layer_depth_spacing(&self, ui: &mut pierro::UI, depth: i32) {
        pierro::h_spacing(ui, (depth as f32) * 15.0);
    }

    fn start_rename<L: LayerUI>(&mut self, curr_name: &String, ptr: Ptr<L>) {
        self.renaming_state = Some((ptr.any(), curr_name.clone()));
        self.started_renaming = true;
    }

    fn handle_layer_dropping(&mut self, ui: &mut pierro::UI, layer_response: &pierro::Response, idx: usize) {
        ui.set_sense_dnd_hover(layer_response.node_ref, true);

        if ui.memory().has_dnd_payload_of_type::<LayerList>() {
            if let Some(mouse_pos) = ui.input().mouse_pos {
                let rect = ui.memory().get::<pierro::LayoutInfo>(layer_response.id).screen_rect;
                if rect.top_half().contains(mouse_pos) {
                    self.layer_dnd_hover_pos = Some(LayerDropLocation {
                        render_list_idx: idx,
                        above: true,
                    });
                } else if rect.bottom_half().contains(mouse_pos) {
                    self.layer_dnd_hover_pos = Some(LayerDropLocation {
                        render_list_idx: idx,
                        above: false,
                    });
                }
            }
        }

        if let Some(layers) = pierro::dnd_receive_payload(ui, layer_response) {
            self.layer_dnd_dropped_payload = Some(layers);
        }
    }

    pub(super) fn layers(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, render_list: &LayerRenderList, clip: &ClipInner) -> pierro::ScrollAreaResponse<pierro::UIRef> {

        self.layer_dnd_hover_pos = None;

        let mut scroll_state = self.scroll_state;
        let scroll_response = pierro::ScrollArea::default()
            .hide_scroll_bars()
            .with_state(&mut scroll_state)
            .scroll_x(false)
            .no_set_max_scroll()
            .render(ui, |ui| {

                let (layers_response, _) = pierro::vertical(ui, |ui| {
                    for (idx, render_layer ) in render_list.iter().enumerate() {
                        match &render_layer.kind {
                            &RenderLayerKind::Layer(ptr, layer) => {
                                self.render_layer(ui, project, editor, idx, render_layer.depth, layer, ptr);
                            },
                            &RenderLayerKind::LayerGroup(ptr, layer_group) => {
                                self.render_layer_group(ui, project, editor, idx, render_layer.depth, layer_group, ptr);
                            }
                        }
                    }
                    
                });
                
                layers_response.node_ref
            });
        self.scroll_state = scroll_state;

        ui.set_sense_dnd_hover(scroll_response.scroll_area.node_ref, true);
        if ui.memory().has_dnd_payload_of_type::<LayerList>() {
            if scroll_response.scroll_area.dnd_hovered {
                self.layer_dnd_hover_pos = Some(LayerDropLocation {
                    render_list_idx: render_list.len(),
                    above: false,
                });
            }
        }
        if let Some(layers) = pierro::dnd_receive_payload(ui, &scroll_response.scroll_area) {
            self.layer_dnd_dropped_payload = Some(layers);
        }

        if let Some(layer_dnd_hover_pos) = self.layer_dnd_hover_pos {
            let n_layers = render_list.len();
            ui.set_on_paint(scroll_response.inner, move |painter, rect| {
                let y_index = layer_dnd_hover_pos.render_list_idx.min(n_layers.saturating_sub(1)) + if layer_dnd_hover_pos.above { 0 } else { 1 };
                let line_y = rect.top() + (y_index as f32) * Self::LAYER_HEIGHT;
                let line_rect = pierro::Rect::center_size(pierro::vec2(rect.center().x, line_y), pierro::vec2(rect.width(), 1.0));
                painter.rect(pierro::PaintRect::new(line_rect, pierro::Color::WHITE));
            });

            if let Some(dropped_layers) = self.layer_dnd_dropped_payload.take() {
                let mut action = Action::new(editor.action_context("Move Layers"));
                let (new_parent, new_idx) = render_list.get_transfer_location(layer_dnd_hover_pos, editor.open_clip, clip);
                dropped_layers.transfer(&mut action, new_parent, new_idx);
                project.client.queue_action(action);

                if let LayerParent::LayerGroup(parent_group) = new_parent {
                    editor.open_layer_groups.insert(parent_group);
                }
            }
        }

        self.layer_dnd_source.display(ui, |ui| {
            let Some(selection) = ui.memory().get_dnd_payload::<LayerList>() else {
                ui.memory().clear_dnd_payload();
                return;
            };
            let selection = selection.clone();
            selection.render_contents(ui, &project.client); 
        });

        scroll_response
    }

}
