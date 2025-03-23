
use crate::{Response, Size, Stroke, UINodeParams, UIRef, UI};
use super::theme::{self, error_label_text_style};

pub fn error_label<S: Into<String>>(ui: &mut UI, label: S) -> Response {
    let text_style = error_label_text_style(ui);

    ui.node(
        UINodeParams::new(Size::text(), Size::text())
            .with_text(label)
            .with_text_style(text_style)
    )

}

pub fn error_outline(ui: &mut UI, target: UIRef) {
    let error_color = ui.style::<theme::ErrorColor>();
    let stroke_width = ui.style::<theme::WidgetStroke>().width;
    ui.set_stroke(target, Stroke::new(error_color, stroke_width));
} 
