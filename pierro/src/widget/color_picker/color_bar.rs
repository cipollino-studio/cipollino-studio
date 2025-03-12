use crate::{vec2, Color, PaintRect, Painter, Rect, Stroke};

use super::{ColorSpace, N_CELLS};

pub(super) fn paint_color_bar<C: ColorSpace>(painter: &mut Painter, rect: Rect, color: Color) {

    let [c0, _c1, _c2] = C::from_rgb([color.r, color.g, color.b]); 

    let cell_height = rect.height() / (N_CELLS as f32);
    let cell_size = vec2(rect.width(), cell_height);
    for y in 0..N_CELLS {
        let cell = Rect::min_size(
            rect.tl() + cell_size * vec2(0.0, y as f32),
            cell_size
        );
        let yt = (y as f32) / (N_CELLS as f32 - 1.0);
        let [r, g, b] = C::to_rgb([yt, 1.0, 1.0]);
        let color = Color {
            r,
            g,
            b,
            a: 1.0,
        };
        painter.rect(PaintRect::new(cell, color));
    }

    let color_pos = rect.tl() + vec2(0.5, c0) * rect.size(); 
    let color_rect = Rect::center_size(color_pos, vec2(rect.width(), 10.0));
    let [r, g, b] = C::to_rgb([c0, 1.0, 1.0]);
    let fill_color = Color::rgb(r, g, b);
    let outline_value = if (fill_color.r + fill_color.g + fill_color.b) / 3.0 > 0.5 {
        0.0
    } else {
        1.0
    };
    let outline_color = Color::gray(outline_value);   
    painter.rect(
        PaintRect::new(color_rect, fill_color)
            .with_stroke(Stroke::new(outline_color, 1.0))
    );

}
