
use crate::{EditorState, PencilTool, SelectTool, Tool};

use super::ScenePanel;

impl ScenePanel {

    const GAP: f32 = 3.0;

    fn tool_button<T: Tool + 'static>(&mut self, ui: &mut pierro::UI, editor: &mut EditorState) {
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
    }

    pub(super) fn toolbar(&mut self, ui: &mut pierro::UI, editor: &mut EditorState) {
        let bg = ui.style::<pierro::theme::BgLight>();
        let margin = pierro::Margin::same(Self::GAP);
        ui.with_style::<pierro::theme::WidgetMargin, _, _>(pierro::Margin::same(3.0), |ui| {
            ui.with_style::<pierro::theme::BgButton, _, _>(bg, |ui| {
                ui.with_style::<pierro::theme::LabelFontSize, _, _>(20.0, |ui| {
                    ui.with_node(
                        pierro::UINodeParams::new(pierro::Size::fit(), pierro::Size::fr(1.0))
                            .with_fill(bg)
                            .with_margin(margin),
                        |ui| {
                            self.tool_button::<SelectTool>(ui, editor);
                            self.tool_button::<PencilTool>(ui, editor);
                        }
                    );
                });
            });
        });
        
    }

}
