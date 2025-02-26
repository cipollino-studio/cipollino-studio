
use crate::{icons, vec2, Color, Id, Layout, LayoutInfo, PerAxis, Response, Size, UINodeParams, UI};

use super::{close_context_menu, h_line, horizontal, icon, is_context_menu_open, label, open_context_menu, render_context_menu, theme::{self, label_text_style}};

#[derive(Default)]
struct MenuMemory {
    open_submenu_id: Option<Id>
}

pub fn menu_bar<F: FnOnce(&mut UI)>(ui: &mut UI, contents: F) {
    let fill = ui.style::<theme::BgDark>(); 
    ui.with_node(
        UINodeParams::new(Size::fr(1.0), Size::fit())
            .with_fill(fill),
        |ui| {
            horizontal(ui, contents);
            h_line(ui);
    });
}

pub fn menu_bar_item<S: Into<String>, F: FnOnce(&mut UI)>(ui: &mut UI, label: S, contents: F) {
    let margin = ui.style::<theme::WidgetMargin>();
    let rounding = ui.style::<theme::WidgetRounding>(); 
    let open_fill = ui.style::<theme::BgLight>();
    let text_style = label_text_style(ui);

    let response = ui.node(
        UINodeParams::new(Size::text(), Size::text())
            .with_fill(Color::TRANSPARENT)
            .with_margin(margin)
            .with_rounding(rounding)
            .with_text_style(text_style)
            .with_text(label)
            .sense_mouse()
    );

    if is_context_menu_open(ui, response.id) {
        ui.set_fill(response.node_ref, open_fill);
    }

    let parent_id = ui.get_parent_id(response.node_ref);
    let open_menu_id = ui.memory().get::<MenuMemory>(parent_id).open_submenu_id;
    
    render_context_menu(ui, response.id, contents);
    if open_menu_id != Some(response.id) && (response.mouse_pressed() || (response.hovered && open_menu_id.is_some())) {
        let button_rect = ui.memory().get::<LayoutInfo>(response.id).screen_rect;
        let position = button_rect.bl();
        open_context_menu(ui, response.id, position, PerAxis::splat(None));

        if let Some(open_menu_id) = open_menu_id {
            close_context_menu(ui, open_menu_id);
        }
        ui.memory().get::<MenuMemory>(parent_id).open_submenu_id = Some(response.id);
    }
    if open_menu_id == Some(response.id) && !is_context_menu_open(ui, response.id) {
        ui.memory().get::<MenuMemory>(parent_id).open_submenu_id = None;
    }
     
}

fn menu_button_common<S: Into<String>, F: FnOnce(&mut UI)>(ui: &mut UI, label_text: S, contents: F) -> Response {
    let bg_hover = theme::hovered_color(ui.style::<theme::BgLight>()); 
    let margin = ui.style::<theme::WidgetMargin>() / 2.0;
    let rounding = ui.style::<theme::WidgetRounding>();

    let (response, _) = ui.with_node(
        UINodeParams::new(Size::fr(1.0), Size::fit())
            .with_layout(Layout::horizontal())
            .with_margin(margin)
            .with_rounding(rounding)
            .sense_mouse(),
        |ui| {
            label(ui, label_text);
            ui.node(UINodeParams::new(Size::fr(1.0), Size::fr(1.0)));
            contents(ui);
        }
    );

    if response.hovered {
        ui.set_fill(response.node_ref, bg_hover);
    }

    response
}

pub fn menu_button<S: Into<String>>(ui: &mut UI, label_text: S) -> Response {
    let response = menu_button_common(ui, label_text, |_| {});

    let parent_id = ui.get_parent_id(response.node_ref);
    let open_menu_id = ui.memory().get::<MenuMemory>(parent_id).open_submenu_id;
    
    if open_menu_id != Some(response.id) && response.hovered { 
        if let Some(open_menu_id) = open_menu_id {
            close_context_menu(ui, open_menu_id);
        }
    }

    response
}

pub fn menu_category<S: Into<String>, F: FnOnce(&mut UI)>(ui: &mut UI, label_text: S, contents: F) {
    let response = menu_button_common(ui, label_text, |ui| {
        icon(ui, icons::CARET_RIGHT);
    });

    let parent_id = ui.get_parent_id(response.node_ref);
    let open_menu_id = ui.memory().get::<MenuMemory>(parent_id).open_submenu_id;
    
    render_context_menu(ui, response.id, contents);
    if open_menu_id != Some(response.id) && response.hovered {
        let stroke_width = ui.style::<theme::WidgetStroke>().width;
        let button_rect = ui.memory().get::<LayoutInfo>(response.id).screen_rect;
        let parent_rect = ui.memory().get::<LayoutInfo>(parent_id).screen_rect;
        let position = vec2(parent_rect.right() - stroke_width, button_rect.top()); 
        open_context_menu(ui, response.id, position, PerAxis::splat(None));

        if let Some(open_menu_id) = open_menu_id {
            close_context_menu(ui, open_menu_id);
        }
        ui.memory().get::<MenuMemory>(parent_id).open_submenu_id = Some(response.id);
    }
    if open_menu_id == Some(response.id) && !is_context_menu_open(ui, response.id) {
        ui.memory().get::<MenuMemory>(parent_id).open_submenu_id = None;
    }

}
