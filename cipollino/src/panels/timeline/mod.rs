
use cipollino_project::{crdt::fractional_index::FractionalIndex, project::{action::Action, layer::Layer, obj::{ObjPtr, ObjRef}}};
use egui::ScrollArea;

use crate::{app::{editor::EditorState, AppSystems}, util::ui::{dnd::{dnd_drop_zone_setup_colors, draggable_widget}, NO_MARGIN}};

use super::Panel;

mod layers;
mod frames;

#[derive(Default)]
pub struct Timeline {
    x_scroll: f32,
    y_scroll: f32
}

enum TimelineCommand {
    HideLayer(ObjPtr<Layer>),
    LockLayer(ObjPtr<Layer>),
    TransferLayer(ObjPtr<Layer>, FractionalIndex)
}

struct TimelineLayer<'a> {
    layer: &'a ObjRef<'a, Layer>,

    drop_top_idx: FractionalIndex,
    drop_bottom_idx: FractionalIndex
}

struct TimelineRenderInfo<'a> {
    len: i32,
    layers: Vec<TimelineLayer<'a>>,
    
    layer_h: f32,
    frame_w: f32,

    x_scroll: f32,
    set_x_scroll: Option<f32>,

    y_scroll: f32,
    set_y_scroll: Option<f32>,

    frame_number_step: usize,

    commands: Vec<TimelineCommand>
}

impl Timeline {

    fn render(&mut self, ui: &mut egui::Ui, state: &mut EditorState, systems: &mut AppSystems) {
        let Some(clip) = state.project.clips.get(state.open_clip) else { return; };

        let layers: Vec<ObjRef<Layer>> = clip.layers.iter_ref(&state.project.layers).collect();
        let timeline_layers = (0..layers.len()).map(|i| TimelineLayer {
            layer: &layers[i],
            drop_top_idx: if i > 0 {
                    FractionalIndex::avg(&layers[i].clip.1, &layers[i - 1].clip.1)
                } else {
                    layers[i].clip.1.avg_with_0()
                },
            drop_bottom_idx: if i < layers.len() - 1 {
                    FractionalIndex::avg(&layers[i].clip.1, &layers[i + 1].clip.1)
                } else {
                    layers[i].clip.1.avg_with_1()
                }
        }).collect();

        let mut render_info = TimelineRenderInfo {
            len: /* *clip.length.value() */ 1000, 
            layers: timeline_layers,

            layer_h: 20.0,
            frame_w: 10.0,
            
            x_scroll: self.x_scroll,
            set_x_scroll: None,
            y_scroll: self.y_scroll,
            set_y_scroll: None,

            frame_number_step: 5,

            commands: Vec::new()
        };

        egui::TopBottomPanel::top(ui.next_auto_id())
            .resizable(false)
            .exact_height(22.)
            .frame(NO_MARGIN)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.button("1");
                    ui.button("2");
                    ui.button("3");
                });
            });
        
        egui::CentralPanel::default()
            .frame(NO_MARGIN)
            .show_inside(ui, |ui| {
                self.render_timeline(ui, state, systems, &mut render_info);
            });

        if let Some(x) = render_info.set_x_scroll {
            self.x_scroll = x;
        }
        if let Some(y) = render_info.set_y_scroll {
            self.y_scroll = y;
        }
        for command in render_info.commands {
            match command {
                TimelineCommand::HideLayer(ptr) => {
                    let mut action = Action::new();
                    let Some(layer) = state.project.layers.get(ptr) else { continue; };
                    let hide = *layer.hide.value();
                    state.client.set_layer_hide(&mut state.project, ptr, !hide, &mut action);
                    state.actions.push_action(action);
                },
                TimelineCommand::LockLayer(ptr) => {
                    let mut action = Action::new();
                    let Some(layer) = state.project.layers.get(ptr) else { continue; };
                    let lock = *layer.lock.value();
                    state.client.set_layer_lock(&mut state.project, ptr, !lock, &mut action);
                    state.actions.push_action(action);
                },
                TimelineCommand::TransferLayer(layer, new_idx) => {
                    let mut action = Action::new();
                    state.client.transfer_layer(&mut state.project, layer, state.open_clip, new_idx, &mut action);
                    state.actions.push_action(action);
                },
            }
        }
    }

    fn render_timeline(&mut self, ui: &mut egui::Ui, state: &EditorState, systems: &mut AppSystems, info: &mut TimelineRenderInfo) {

        let header_height = 22.0;

        egui::SidePanel::left(ui.next_auto_id())
            .frame(NO_MARGIN)
            .resizable(true)
            .show_separator_line(true)
            .default_width(150.0)
            .min_width(150.0)
            .show_inside(ui, |ui| {
                egui::TopBottomPanel::top(ui.next_auto_id())
                    .exact_height(header_height)
                    .frame(NO_MARGIN)
                    .show_inside(ui, |_ui| {

                    });
                egui::CentralPanel::default()
                    .frame(NO_MARGIN)
                    .show_inside(ui, |ui| {
                        // Fix panel clipping weirdness
                        let mut clip_rect = ui.max_rect();
                        clip_rect.max.x -= 3.0;
                        ui.set_clip_rect(clip_rect);

                        self.render_layers(ui, state, systems, info);
                    })
            });

        egui::CentralPanel::default()
            .frame(NO_MARGIN)
            .show_inside(ui, |ui| {
                egui::TopBottomPanel::top(ui.next_auto_id())
                    .exact_height(header_height)
                    .frame(NO_MARGIN)
                    .show_inside(ui, |ui| {
                        self.render_header(ui, state, systems, info);
                    });
                egui::CentralPanel::default()
                    .frame(NO_MARGIN)
                    .show_inside(ui, |ui| {
                        // Fix panel clipping weirdness
                        let mut clip_rect = ui.max_rect();
                        clip_rect.max += egui::Vec2::splat(3.0);
                        ui.set_clip_rect(clip_rect);

                        self.render_frames(ui, state, systems, info);
                    })
            });
    }

    fn render_header(&mut self, ui: &mut egui::Ui, state: &EditorState, systems: &mut AppSystems, info: &mut TimelineRenderInfo) {
        let scroll_resp = ScrollArea::horizontal()
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .scroll_offset(egui::vec2(info.x_scroll, 0.0)) // Always scroll to the current scroll x to sync with other scroll areas
            .show(ui, |ui| {
                let (rect, resp) = ui.allocate_exact_size(egui::vec2(info.frame_w * (info.len as f32), ui.available_height()), egui::Sense::click_and_drag());
                for i in (0..info.len).skip(info.frame_number_step - 1).step_by(info.frame_number_step) {
                    let number_center = rect.min + egui::vec2((i as f32 + 0.5) * info.frame_w, ui.available_height() / 2.0);
                    let number_rect = egui::Rect::from_center_size(number_center, egui::vec2(0.0, ui.available_height()));
                    ui.put(number_rect, egui::Label::new(format!("{}", i + 1)).selectable(false).wrap(false));
                }
            });

        // If the user hovered over the header scroll area, update the scroll x based on the scroll area
        if ui.input(|i| i.pointer.hover_pos().map(|pos| scroll_resp.inner_rect.contains(pos)).unwrap_or(false)) {
            info.set_x_scroll = Some(scroll_resp.state.offset.x);
        }
    }

}

impl Panel for Timeline {
    
    fn ui(&mut self, ui: &mut egui::Ui, state: &mut EditorState, systems: &mut AppSystems) {
        let Some(clip) = state.project.clips.get(state.open_clip) else {
            ui.centered_and_justified(|ui| {
                ui.label("No clip open");
            });
            return;
        };
        if !state.project.clips.is_loaded(state.open_clip) {
            ui.centered_and_justified(|ui| {
                ui.label("Clip not loaded");
            });
            return;
        }
        
        self.render(ui, state, systems);
    }

}
