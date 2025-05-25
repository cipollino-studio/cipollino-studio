
pub fn color_picker_context_menu(ui: &mut pierro::UI, color: &mut elic::Color, button: &pierro::Response) -> pierro::ColorPickerResponse {
    let mut editing = false;
    let mut done_editing = false;

    pierro::left_click_context_menu(ui, button, |ui| {
        pierro::horizontal_fit(ui, |ui| {
            let color_response = pierro::color_picker::<pierro::HSVColorSpace>(ui, color);
            editing |= color_response.editing;
            done_editing |= color_response.done_editing;
            
            pierro::h_spacing(ui, 5.0);
            pierro::vertical_fit(ui, |ui| {
                pierro::key_value_layout(ui, |builder| {
                    builder.labeled("R:", |ui| {
                        let resp = pierro::DragValue::new(&mut color.r)
                            .with_min(0.0)
                            .with_max(1.0)
                            .render(ui);
                        editing |= resp.editing;
                        done_editing |= resp.done_editing;
                    });
                    builder.labeled("G:", |ui| {
                        let resp = pierro::DragValue::new(&mut color.g)
                            .with_min(0.0)
                            .with_max(1.0)
                            .render(ui);
                        editing |= resp.editing;
                        done_editing |= resp.done_editing;
                    });
                    builder.labeled("B:", |ui| {
                        let resp = pierro::DragValue::new(&mut color.b)
                            .with_min(0.0)
                            .with_max(1.0)
                            .render(ui);
                        editing |= resp.editing;
                        done_editing |= resp.done_editing;
                    });
                });
            });
        }); 
    });

    pierro::ColorPickerResponse {
        editing,
        done_editing
    }
}

pub fn color_picker_with_icon(ui: &mut pierro::UI, color: &mut elic::Color, icon: Option<&'static str>) -> pierro::ColorPickerResponse {
    let margin = ui.style::<pierro::theme::WidgetMargin>();
    let rounding = ui.style::<pierro::theme::WidgetRounding>();
    let stroke = ui.style::<pierro::theme::WidgetStroke>();
    let (picker_button, _) = ui.with_node(
        pierro::UINodeParams::new(pierro::Size::fit(), pierro::Size::fit())
            .with_margin(margin)
            .with_rounding(rounding)
            .with_stroke(stroke)
            .with_fill(*color)
            .sense_mouse(),
        |ui| {
            if let Some(icon) = icon {
                ui.with_style::<pierro::theme::TextColor, _, _>(color.contrasting_color(), |ui| {
                    pierro::icon(ui, icon);
                });
            } else {
                pierro::icon_gap(ui);
            }
        }
    );

    color_picker_context_menu(ui, color, &picker_button) 
}

pub fn color_picker(ui: &mut pierro::UI, color: &mut elic::Color) -> pierro::ColorPickerResponse {
    color_picker_with_icon(ui, color, None) 
}
