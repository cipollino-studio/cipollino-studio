
use crate::{vec2, Color, PaintRect, Painter, Rect, Stroke, Vec2};
use super::{ColorSpace, N_CELLS};

pub(super) fn paint_color_area<C: ColorSpace>(painter: &mut Painter, rect: Rect, color: Color) {

    let [c0, c1, c2] = C::from_rgb([color.r, color.g, color.b]); 

    let cell_width = rect.width() / (N_CELLS as f32);
    let cell_height = rect.width() / (N_CELLS as f32);
    let cell_size = vec2(cell_width, cell_height);
    for x in 0..N_CELLS {
        for y in 0..N_CELLS {
            let cell = Rect::min_size(
                rect.tl() + cell_size * vec2(x as f32, y as f32),
                cell_size
            );
            let xt = (x as f32) / (N_CELLS as f32 - 1.0);
            let yt = (y as f32) / (N_CELLS as f32 - 1.0);
            let [r, g, b] = C::to_rgb([c0, xt, 1.0 - yt]);
            let color = Color {
                r,
                g,
                b,
                a: 1.0,
            };
            painter.rect(PaintRect::new(cell, color));
        }
    }

    let color_pos = rect.tl() + vec2(c1, 1.0 - c2) * rect.size(); 
    let color_rect = Rect::center_size(color_pos, Vec2::splat(10.0));
    let outline_value = if (color.r + color.g + color.b) / 3.0 > 0.5 {
        0.0
    } else {
        1.0
    };
    let outline_color = Color::gray(outline_value);
    painter.rect(
        PaintRect::new(color_rect, color)
            .with_stroke(Stroke::new(outline_color, 1.0))
    );
}
