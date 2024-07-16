
use cipollino_project::{crdt::fractional_index::FractionalIndex, project::{action::Action, layer::Layer, obj::{ObjPtr, ObjRef}}};

use crate::{app::AppSystems, editor::EditorState, util::ui::NO_MARGIN};

use super::Panel;

mod layers;
mod frames;
mod header;
mod controls;

#[derive(Default)]
pub struct Timeline {
    x_scroll: f32,
    y_scroll: f32
}

const PLAYBACK_HEAD_STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(0, 50, 255);
const PLAYBACK_HEAD_HIGHLIGHT_COLOR: egui::Color32 = egui::Color32::from_rgba_premultiplied(0, 12, 96, 30);

enum TimelineCommand {
    AddLayer,
    HideLayer(ObjPtr<Layer>),
    LockLayer(ObjPtr<Layer>),
    TransferLayer(ObjPtr<Layer>, FractionalIndex),

    TogglePlaying,
    SetPlaybackFrame(i32),

    SetActiveLayer(ObjPtr<Layer>),

    AddFrame
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

    fn render(&mut self, ui: &mut egui::Ui, state: &mut EditorState) {
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
        let new_layer_insertion_idx = layers.first().map(|layer| layer.clip.1.avg_with_0()).unwrap_or(FractionalIndex::half());

        let len = *clip.length.value();
        let mut render_info = TimelineRenderInfo {
            len, 
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
                self.render_controls(ui, state, &mut render_info);
            });
        
        egui::CentralPanel::default()
            .frame(NO_MARGIN)
            .show_inside(ui, |ui| {
                self.render_timeline(ui, state, &mut render_info);
            });

        if let Some(x) = render_info.set_x_scroll {
            self.x_scroll = x;
        }
        if let Some(y) = render_info.set_y_scroll {
            self.y_scroll = y;
        }
        for command in render_info.commands {
            match command {
                TimelineCommand::AddLayer => {
                    let mut action = Action::new();
                    state.client.add_layer(&mut state.project, state.open_clip, new_layer_insertion_idx.clone(), "Layer".to_owned(), 1.0, false, false, &mut action);
                    state.actions.push_action(action);
                },
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
                TimelineCommand::TogglePlaying => {
                    state.playing = !state.playing;
                },
                TimelineCommand::SetPlaybackFrame(frame) => {
                    state.playback_time = ((frame.max(0).min(len - 1) as f32) / state.project.fps * state.project.sample_rate) as i64 + 10;
                },
                TimelineCommand::SetActiveLayer(layer) => {
                    state.active_layer = layer;
                },
                TimelineCommand::AddFrame => {
                    state.create_frame();
                }
            }
        }
    }

    fn render_timeline(&mut self, ui: &mut egui::Ui, state: &EditorState, info: &mut TimelineRenderInfo) {

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

                        self.render_layers(ui, state, info);
                    })
            });

        egui::CentralPanel::default()
            .frame(NO_MARGIN)
            .show_inside(ui, |ui| {
                egui::TopBottomPanel::top(ui.next_auto_id())
                    .exact_height(header_height)
                    .frame(NO_MARGIN)
                    .show_inside(ui, |ui| {
                        // Fix panel clipping weirdness
                        let mut clip_rect = ui.max_rect();
                        clip_rect.max += egui::Vec2::splat(3.0);
                        ui.set_clip_rect(clip_rect);

                        self.render_header(ui, state, info);
                    });
                
                let (frames_rect, frames_scroll_area) = egui::CentralPanel::default()
                    .frame(NO_MARGIN)
                    .show_inside(ui, |ui| {
                        // Fix panel clipping weirdness
                        let mut clip_rect = ui.max_rect();
                        clip_rect.max += egui::Vec2::splat(3.0);
                        ui.set_clip_rect(clip_rect);

                        self.render_frames(ui, state, info)
                    }).inner;

                // We draw the playback line separately to give it the correct clipping rectangle 
                self.render_playback_line(ui, state, info, frames_rect, frames_scroll_area);
            });
    }

    fn render_playback_line(&mut self, ui: &mut egui::Ui, state: &EditorState, info: &TimelineRenderInfo, frames_rect: egui::Rect, frames_scroll_area: egui::Rect) {
        // Draw playback line
        let painter = ui.painter().with_clip_rect(egui::Rect::from_center_size(frames_scroll_area.center(), egui::vec2(frames_scroll_area.width(), 1000000.0)));
        let frame = state.playback_frame();
        painter.vline(
            frames_rect.left() + (frame as f32 + 0.5) * info.frame_w,
            egui::Rangef::new(frames_scroll_area.top() - 2.0, frames_scroll_area.bottom()),
            egui::Stroke {
                width: 1.0,
                color: PLAYBACK_HEAD_STROKE_COLOR,
            }
        );

    }

}

impl Panel for Timeline {
    
    fn ui(&mut self, ui: &mut egui::Ui, state: &mut EditorState, _systems: &mut AppSystems) {
        let Some(_clip) = state.project.clips.get(state.open_clip) else {
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
        
        self.render(ui, state);
    }

}
