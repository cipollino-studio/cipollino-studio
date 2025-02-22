use crate::{icons, Layout, Margin, Response, Size, UINodeParams, UI};

use super::{button_fill_animation, clickable_icon, h_spacing, label, Theme};

pub struct TabResponse {
    pub tab: Response,
    pub close_button: Response
}

/// A selectable tab with a close button. Should be used inside `pierro::menu_bar`
pub fn tab<S: Into<String>>(ui: &mut UI, label_text: S, selected: bool) -> TabResponse {
    let theme = ui.style::<Theme>();
    let tab_bg = if selected { theme.bg_light } else { theme.bg_dark };
    let widget_margin = theme.widget_margin;

    let (tab, close_button) = ui.with_node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_layout(Layout::horizontal())
            .with_margin(Margin::same(widget_margin))
            .with_fill(tab_bg)
            .sense_mouse(),
        |ui| {
            label(ui, label_text);
            h_spacing(ui, 6.0);

            clickable_icon(ui, icons::X)
        }
    );

    button_fill_animation(ui, tab.node_ref, &tab, tab_bg);

    TabResponse {
        tab,
        close_button
    }
}
