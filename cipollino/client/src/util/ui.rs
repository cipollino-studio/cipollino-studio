
pub fn color_picker(ui: &mut pierro::UI, color: &mut elic::Color) {
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
            pierro::icon_gap(ui);
        }
    );

    pierro::left_click_context_menu(ui, &picker_button, |ui| {
        pierro::horizontal_fit(ui, |ui| {
            pierro::color_picker::<pierro::HSVColorSpace>(ui, color);
            
            pierro::h_spacing(ui, 5.0);
            pierro::vertical_fit(ui, |ui| {
                pierro::key_value_layout(ui, |builder| {
                    builder.labeled("R:", |ui| {
                        pierro::DragValue::new(&mut color.r)
                            .with_min(0.0)
                            .with_max(1.0)
                            .render(ui);
                    });
                    builder.labeled("G:", |ui| {
                        pierro::DragValue::new(&mut color.g)
                            .with_min(0.0)
                            .with_max(1.0)
                            .render(ui);
                    });
                    builder.labeled("B:", |ui| {
                        pierro::DragValue::new(&mut color.b)
                            .with_min(0.0)
                            .with_max(1.0)
                            .render(ui);
                    });
                });
            });
        }); 
    });
}