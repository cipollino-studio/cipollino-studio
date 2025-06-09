
use crate::{theme, PaintRect, Size, UINodeParams, UI};

pub fn progress_bar(ui: &mut UI, progress: f32) {

    let accent_color = ui.style::<theme::AccentColor>();
    let bg_color = ui.style::<theme::BgTextField>();

    ui.node(UINodeParams::new(Size::px(200.0).with_grow(1.0), Size::px(20.0)).on_paint(move |painter, rect| {
        painter.rect(PaintRect::new(rect.left_frac(progress), accent_color));
        painter.rect(PaintRect::new(rect.right_frac(progress), bg_color));
    }));

}
