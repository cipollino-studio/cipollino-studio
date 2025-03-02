
use project::{Action, Clip, ClipInner, CreateFrame, CreateLayer, FrameTreeData, LayerParent, LayerTreeData, Ptr};

use crate::{EditorState, ProjectState};

use super::TimelinePanel;

impl TimelinePanel {

    pub(super) fn header(&mut self, ui: &mut pierro::UI, project: &ProjectState, editor: &mut EditorState, clip_ptr: Ptr<Clip>, clip: &ClipInner) {
        if pierro::icon_button(ui, pierro::icons::PLUS).mouse_clicked() {
            if let Some(ptr) = project.client.next_ptr() {
                project.client.queue_action(Action::single(CreateFrame {
                    ptr,
                    layer: editor.active_layer,
                    data: FrameTreeData {
                        time: clip.frame_idx(editor.time),
                    },
                }));
            }
        }
        if pierro::icon_button(ui, pierro::icons::FILE_PLUS).mouse_clicked() {
            if let Some(ptr) = project.client.next_ptr() {
                project.client.queue_action(Action::single(CreateLayer {
                    ptr,
                    parent: LayerParent::Clip(clip_ptr),
                    idx: clip.layers.n_children(),
                    data: LayerTreeData {
                        name: "Layer".to_owned(),
                        ..Default::default()
                    },
                }));
                editor.active_layer = ptr;
            }
        }

        pierro::centered_horizontal(ui, |ui| {
            let widget_rounding = ui.style::<pierro::theme::WidgetRounding>();
            let widget_stroke = ui.style::<pierro::theme::WidgetStroke>();
            let divider_color = ui.style::<pierro::theme::BgButton>().darken(0.25);
            ui.with_style::<pierro::theme::WidgetStroke, _, _>(pierro::Stroke::new(divider_color, widget_stroke.width), |ui| {
                ui.with_style::<pierro::theme::WidgetRounding, _, _>(widget_rounding.left_side(), |ui| {
                    if pierro::icon_button(ui, pierro::icons::CARET_DOUBLE_LEFT).mouse_clicked() {
                        editor.jump_to(0.0);
                    }
                });
                pierro::v_line(ui);
                ui.with_style::<pierro::theme::WidgetRounding, _, _>(pierro::Rounding::ZERO, |ui| {
                    pierro::icon_button(ui, pierro::icons::CARET_LINE_LEFT);
                    pierro::v_line(ui);

                    let play_icon = if editor.playing {
                        pierro::icons::PAUSE
                    } else {
                        pierro::icons::PLAY
                    };
                    if pierro::icon_button(ui, play_icon).mouse_clicked() {
                        editor.playing = !editor.playing; 
                    }
                    pierro::v_line(ui);

                    pierro::icon_button(ui, pierro::icons::CARET_LINE_RIGHT);
                });
                pierro::v_line(ui);
                ui.with_style::<pierro::theme::WidgetRounding, _, _>(widget_rounding.right_side(), |ui| {
                    if pierro::icon_button(ui, pierro::icons::CARET_DOUBLE_RIGHT).mouse_clicked() {
                        editor.jump_to(((clip.length - 1) as f32) * clip.frame_len());
                    }
                });
            });
        });
    }

}
