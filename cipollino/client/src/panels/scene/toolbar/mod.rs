
use crate::{color_picker, AppSystems, BucketTool, ColorPicker, EditorState, PaintBrushTool, PencilTool, SelectTool, Tool};

use super::ScenePanel;
use crate::Shortcut;

impl ScenePanel {

    const GAP: f32 = 3.0;

    fn tool_button<T: Tool + 'static>(&mut self, ui: &mut pierro::UI, editor: &mut EditorState, systems: &mut AppSystems) {
        let response = pierro::icon_button(ui, T::ICON);
        pierro::v_spacing(ui, Self::GAP);

        // Minor hack: tool icons are unique, so we can compare them to see if T is the tool selected
        if editor.curr_tool.borrow().icon() == T::ICON { 
            ui.set_sense_mouse(response.node_ref, false);
            let color = pierro::theme::pressed_color(ui.style::<pierro::theme::BgButton>());
            ui.set_fill(response.node_ref, color);
        } else {
            if response.mouse_clicked() {
                *editor.curr_tool.borrow_mut() = Box::new(T::default());
            }
        }

        if T::Shortcut::used_globally(ui, systems) {
            *editor.curr_tool.borrow_mut() = Box::new(T::default());
        }
    }

    fn mirror_button(&mut self, ui: &mut pierro::UI) {
        let mirror = self.mirror;

        if mirror {
            let bg = pierro::theme::pressed_color(ui.style::<pierro::theme::BgButton>());
            ui.push_style::<pierro::theme::BgButton>(bg);
        }
        if pierro::icon_button(ui, pierro::icons::FLIP_HORIZONTAL).mouse_clicked() {
            self.mirror = !self.mirror;
            self.cam_pos *= elic::vec2(-1.0, 1.0);
        }
        if mirror {
            ui.pop_style();
        }

    }

    pub(super) fn toolbar(&mut self, ui: &mut pierro::UI, editor: &mut EditorState, systems: &mut AppSystems) {
        let bg = ui.style::<pierro::theme::BgLight>();
        let margin = pierro::Margin::same(Self::GAP);
        ui.with_style::<pierro::theme::WidgetMargin, _, _>(pierro::Margin::same(3.0), |ui| {
            ui.with_style::<pierro::theme::BgButton, _, _>(bg, |ui| {
                ui.with_style::<pierro::theme::LabelFontSize, _, _>(20.0, |ui| {
                    ui.with_node(
                        pierro::UINodeParams::new(pierro::Size::fit(), pierro::Size::fr(1.0))
                            .with_layout(pierro::Layout::vertical().with_vertical_overflow())
                            .with_fill(bg)
                            .with_margin(margin),
                        |ui| {
                            self.tool_button::<SelectTool>(ui, editor, systems);
                            self.tool_button::<PencilTool>(ui, editor, systems);
                            self.tool_button::<PaintBrushTool>(ui, editor, systems);
                            self.tool_button::<BucketTool>(ui, editor, systems);
                            self.tool_button::<ColorPicker>(ui, editor, systems);
                            color_picker(ui, &mut editor.color);

                            // Spacer
                            ui.node(pierro::UINodeParams::new(pierro::Size::px(0.0), pierro::Size::px(0.0).with_grow(1.0)));

                            self.mirror_button(ui);
                        }
                    );
                });
            });
        });
        
    }

}
