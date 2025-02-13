
use crate::{icons, Layout, LayoutInfo, Margin, PerAxis, Size, UINodeParams, UI};

use super::{button_fill_animation, close_context_menu, h_spacing, horizontal_fit_centered, icon_text_style, is_context_menu_open, label, label_text_style, open_context_menu, render_context_menu, Theme};

pub fn dropdown<S: Into<String>, F: FnOnce(&mut UI)>(ui: &mut UI, dropdown_text: S, contents: F) {
    let theme = ui.style::<Theme>(); 
    let bg = theme.bg_text_field;
    let rounding = theme.widget_rounding;
    let widget_margin = theme.widget_margin;
    let label_text_style = label_text_style(ui);
    let icon_text_style = icon_text_style(ui);

    let (response, _) = ui.with_node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_layout(Layout::horizontal())
            .with_margin(Margin::same(widget_margin))
            .with_rounding(rounding)
            .sense_mouse(),
        |ui| {
            ui.node(
                UINodeParams::new(Size::px(80.0), Size::text())
                    .with_text(dropdown_text)
                    .with_text_style(label_text_style)
            );
            ui.node(
                UINodeParams::new(Size::text().no_shrink(), Size::text())
                    .with_text(icons::CARET_DOWN)
                    .with_text_style(icon_text_style)
            );
        }
    );

    button_fill_animation(ui, response.node_ref, &response, bg);

    render_context_menu(ui, response.id, contents);
    if is_context_menu_open(ui, response.id) && (ui.input().l_mouse.released() || ui.input().r_mouse.released()) {
        close_context_menu(ui, response.id);
    }
    if response.mouse_clicked() {
        let rect = ui.memory().get::<LayoutInfo>(response.id).screen_rect;
        open_context_menu(ui, response.id, rect.bl(), PerAxis::new(Some(rect.width()), None));
    }

}

pub fn dropdown_labeled<L: Into<String>, S: Into<String>, F: FnOnce(&mut UI)>(ui: &mut UI, label_text: L, dropdown_text: S, contents: F) {
    horizontal_fit_centered(ui, |ui| {
        label(ui, label_text);
        h_spacing(ui, 5.0);
        dropdown(ui, dropdown_text, contents);
    });
}