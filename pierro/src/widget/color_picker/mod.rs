
use crate::{Color, LayoutInfo, Size, UINodeParams, UI};

use super::{h_spacing, horizontal_fit, theme};

mod color_space;
pub use color_space::*;

mod color_area;
use color_area::*;

mod color_bar;
use color_bar::*;

const N_CELLS: i32 = 100;
const TOLERANCE: f32 = 0.005;

pub fn color_picker<C: ColorSpace>(ui: &mut UI, color: &mut Color) {
    let stroke = ui.style::<theme::WidgetStroke>();
    horizontal_fit(ui, |ui| {
        let size = 150.0;
        let color_area = ui.node(
            UINodeParams::new(Size::px(size), Size::px(size))
                .with_stroke(stroke)
                .sense_mouse()
        );
        h_spacing(ui, 3.0);
        let color_bar = ui.node(
            UINodeParams::new(Size::px(20.0), Size::px(size))
                .with_stroke(stroke)
                .sense_mouse()
        );

        if color_area.drag_started() {
            color_area.request_focus(ui);
        }
        if color_area.mouse_released() {
            color_area.release_focus(ui);
        }
        if color_bar.drag_started() {
            color_bar.request_focus(ui);
        }
        if color_bar.mouse_released() {
            color_bar.release_focus(ui);
        }

        let [mut c0, mut c1, mut c2] = C::from_rgb([color.r, color.g, color.b]);
        if color_area.mouse_clicked() || color_area.dragging() {
            if let Some(mouse_pos) = color_area.mouse_pos(ui) {
                let color_area_size = ui.memory().get::<LayoutInfo>(color_area.id).screen_rect.size(); 
                let mouse_pos = mouse_pos / color_area_size;
                c1 = mouse_pos.x;
                c2 = 1.0 - mouse_pos.y;
            }
        }
        if color_bar.mouse_clicked() || color_bar.dragging() {
            if let Some(mouse_pos) = color_bar.mouse_pos(ui) {
                let hue_bar_height = ui.memory().get::<LayoutInfo>(color_bar.id).screen_rect.height();
                c0 = mouse_pos.y / hue_bar_height; 
            }
        }
        c0 = c0.clamp(TOLERANCE, 1.0 - TOLERANCE);
        c1 = c1.clamp(TOLERANCE, 1.0 - TOLERANCE);
        c2 = c2.clamp(TOLERANCE, 1.0 - TOLERANCE);
        let [r, g, b] = C::to_rgb([c0, c1, c2]);
        *color = Color::rgba(r, g, b, color.a);

        let color = *color;
        ui.set_on_paint(color_area.node_ref, move |painter, rect| {
            paint_color_area::<C>(painter, rect, color); 
        });
        ui.set_on_paint(color_bar.node_ref, move |painter, rect| {
            paint_color_bar::<C>(painter, rect, color);
        });
    });

}
