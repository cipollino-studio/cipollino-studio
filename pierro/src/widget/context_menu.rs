
use crate::{Id, LayoutInfo, Margin, PerAxis, Response, Size, TSTransform, UINodeParams, Vec2, UI};

use super::Theme;

struct ContextMenuMemory {
    position: Vec2,
    size: PerAxis<Option<f32>> 
}

pub fn context_menu<F: FnOnce(&mut UI)>(ui: &mut UI, response: &Response, body: F) {
    ui.set_sense_mouse(response.node_ref, true);

    if response.right_mouse_clicked() {
        let position = ui.input().mouse_pos.unwrap_or(Vec2::ZERO);
        open_context_menu(ui, response.id, position, PerAxis::splat(None));        
    }

    render_context_menu(ui, response.id, body);
}

pub fn left_click_context_menu<F: FnOnce(&mut UI)>(ui: &mut UI, response: &Response, body: F) {
    ui.set_sense_mouse(response.node_ref, true);

    if response.mouse_clicked() {
        let position = ui.input().mouse_pos.unwrap_or(Vec2::ZERO);
        open_context_menu(ui, response.id, position, PerAxis::splat(None));        
    }

    render_context_menu(ui, response.id, body);
}

pub fn open_context_menu(ui: &mut UI, id: Id, position: Vec2, size: PerAxis<Option<f32>>) {
    ui.memory().insert(id, ContextMenuMemory {
        position,
        size
    });
}

pub fn close_context_menu(ui: &mut UI, id: Id) {
    ui.memory().remove::<ContextMenuMemory>(id);
}

pub fn is_context_menu_open(ui: &mut UI, id: Id) -> bool {
    ui.memory().has::<ContextMenuMemory>(id)
}

pub fn render_context_menu<F: FnOnce(&mut UI)>(ui: &mut UI, id: Id, body: F) {
    if let Some(context_menu_memory) = ui.memory().get_opt::<ContextMenuMemory>(id) {
        let position = context_menu_memory.position;
        let size = context_menu_memory.size;
        let theme = ui.style::<Theme>();
        let fill = theme.bg_light;
        let stroke = theme.widget_stroke();
        let margin = theme.widget_margin;

        let width = size.x.map(|size| Size::px(size - 2.0 * margin)).unwrap_or(Size::fit());
        let height = size.y.map(|size| Size::px(size - 2.0 * margin)).unwrap_or(Size::fit());

        let (layer, menu) = ui.layer(|ui| {
            let (menu, _) = ui.with_node(
                UINodeParams::new(width, height)
                    .with_fill(fill)
                    .with_stroke(stroke)
                    .with_margin(Margin::same(margin)),
                body
            );

            if menu.mouse_pressed_outside(ui) {
                close_context_menu(ui, id);
            }

            menu
        });

        let menu_size = ui.memory().get::<LayoutInfo>(menu.id).rect.size();
        let max_position = ui.window_size() - menu_size;
        let position = position.min(max_position).max(Vec2::ZERO);

        ui.set_transform(layer, TSTransform::translation(position));
    }
}
