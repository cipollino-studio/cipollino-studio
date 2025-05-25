
use alisa::Children;
use project::{Action, Client, Color, ColorParent, ColorTreeData, CreateColor, DeleteColor, Ptr, SceneObjectColor, SetColorColor, SetColorName};

use crate::{color_picker_context_menu, get_color_value, EditorState};

use super::{Panel, PanelContext};
 
#[derive(Default)]
pub struct ColorsPanel {

}

impl ColorsPanel {

    fn color_context_menu(ui: &mut pierro::UI, client: &Client, editor: &mut EditorState, color_ptr: Ptr<Color>) {
        if pierro::menu_button(ui, "Delete").mouse_clicked() {
            client.queue_action(Action::single(editor.action_context("Delete Color"), DeleteColor {
                ptr: color_ptr
            }));
        }
    }

    fn render_color(ui: &mut pierro::UI, client: &Client, color_ptr: Ptr<Color>, color: &Color, editor: &mut EditorState) {
        ui.push_id_seed(&color_ptr);

        let selected = editor.color.color.ptr() == color_ptr;
        let (color_resp, label_resp) = pierro::horizontal_centered(ui, |ui| {
            let id = ui.get_node_id(ui.curr_parent());

            pierro::h_spacing(ui, 5.0);
            let mut c = *ui.memory().get(id);
            let rounding = ui.style::<pierro::theme::WidgetRounding>();
            let stroke = ui.style::<pierro::theme::WidgetStroke>();
            let color_button = ui.node(
                pierro::UINodeParams::new(pierro::Size::px(30.0), pierro::Size::px(20.0).with_grow(1.0))
                    .with_fill(c)
                    .with_rounding(rounding)
                    .with_stroke(stroke)
                    .sense_mouse()
            );
            let color_picker_resp = color_picker_context_menu(ui, &mut c, &color_button);
            *ui.memory().get::<elic::Color>(id) = if color_picker_resp.done_editing {
                client.queue_action(Action::single(editor.action_context("Set Color"), SetColorColor {
                    ptr: color_ptr,
                    color_value: c.into(),
                }));
                c
            } else if !color_picker_resp.editing {
                color.color.into()
            } else {
                c
            };

            pierro::h_spacing(ui, 5.0);
            if selected {
                let active_text_color = ui.style::<pierro::theme::ActiveTextColor>();
                ui.push_style::<pierro::theme::TextColor>(active_text_color);
            }
            let mut name = color.name.clone();
            let label_resp = pierro::editable_label(ui, &mut name);
            if label_resp.done_editing {
                client.queue_action(Action::single(editor.action_context("Rename Color"), SetColorName {
                    ptr: color_ptr,
                    name_value: name
                }));
            }
            if selected {
                ui.pop_style();
            }

            label_resp.response
        });
        if selected {
            let fill = ui.style::<pierro::theme::BgButton>();
            ui.set_fill(color_resp.node_ref, fill);
        }

        ui.set_sense_mouse(color_resp.node_ref, true);
        if color_resp.mouse_clicked() || label_resp.mouse_clicked() {
            editor.color = SceneObjectColor {
                color: color_ptr.into(),
                backup: color.color
            };
        }
        pierro::context_menu(ui, &color_resp, |ui| {
            Self::color_context_menu(ui, client, editor, color_ptr); 
        });
        pierro::context_menu(ui, &label_resp, |ui| {
            Self::color_context_menu(ui, client, editor, color_ptr); 
        });
    }

    fn render_color_list(ui: &mut pierro::UI, client: &Client, editor: &mut EditorState, color_list: &alisa::UnorderedChildList<alisa::LoadingPtr<Color>>, label: &str, color_parent: ColorParent) {
        let mut colors = Vec::new();
        for color_ptr in color_list.iter() {
            let Some(color) = client.get(color_ptr.ptr()) else {
                continue;
            };
            colors.push((color_ptr.ptr(), color));
        }
        colors.sort_by_key(|(_, color)| &color.name);

        // Header
        let margin = ui.style::<pierro::theme::WidgetMargin>();
        let color = ui.style::<pierro::theme::BgDark>();
        ui.push_id_seed(&color_parent);
        let (header, _) = ui.with_node(
            pierro::UINodeParams::new(pierro::Size::fit().with_grow(1.0), pierro::Size::fit())
                .with_margin(margin)
                .with_fill(color)
                .with_layout(pierro::Layout::horizontal().align_center())
                .sense_mouse(),
            |ui| {
                let id = ui.get_node_id(ui.curr_parent());
                let closed = *ui.memory().get::<bool>(id);
                pierro::icon(ui, if closed {
                    pierro::icons::CARET_RIGHT
                } else {
                    pierro::icons::CARET_DOWN
                });
                pierro::h_spacing(ui, 3.0);
                pierro::label(ui, label); 

                ui.node(pierro::UINodeParams::new(pierro::Size::px(0.0).with_grow(1.0), pierro::Size::px(0.0)));

                if pierro::clickable_icon(ui, pierro::icons::PLUS).mouse_clicked() {
                    *ui.memory().get(id) = false;
                    let color = get_color_value(&editor.color, client);
                    let new_color_ptr = client.next_ptr();
                    client.queue_action(Action::single(editor.action_context("Create Color"), CreateColor {
                        ptr: new_color_ptr,
                        parent: color_parent,
                        idx: (),
                        data: ColorTreeData {
                            color: color.into(),
                            name: format!("Color {}", color_list.n_children() + 1),
                        }
                    }));
                    editor.color = SceneObjectColor {
                        color: new_color_ptr.into(),
                        backup: color.into(),
                    };
                }
            }
        );
        pierro::button_fill_animation(ui, header.node_ref, &header, color); 

        let closed = *ui.memory().get::<bool>(header.id);
        if header.mouse_clicked() {
            *ui.memory().get::<bool>(header.id) = !closed;
        }

        // Colors
        if !closed {
            pierro::v_spacing(ui, 2.0);
            for (color_ptr, color) in colors {
                Self::render_color(ui, client, color_ptr, color, editor);
                pierro::v_spacing(ui, 2.0);
            }
        }
    }

}

impl Panel for ColorsPanel {
    const NAME: &'static str = "Colors";

    fn title(&self) -> String {
        "Colors".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, context: &mut PanelContext) {
        let project = &context.project;
        let editor = &mut context.editor;

        let Some(clip) = project.client.get(editor.open_clip) else {
            pierro::centered(ui, |ui| {
                pierro::label(ui, "No clip open.");
            });
            return;
        };
        let Some(clip_inner) = project.client.get(clip.inner) else {
            pierro::centered(ui, |ui| {
                pierro::label(ui, "Clip loading...");
            });
            return;
        };

        pierro::scroll_area(ui, |ui| {
            Self::render_color_list(ui, &context.project.client, editor, &clip_inner.colors, "Clip Colors", ColorParent::Clip(editor.open_clip));
        });
    }
}
