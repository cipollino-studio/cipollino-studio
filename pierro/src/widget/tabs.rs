
use crate::{icons, Layout, Response, Size, UINodeParams, UI};
use super::{button_fill_animation, clickable_icon, h_spacing, label, theme};

pub struct TabResponse {
    pub tab: Response,
    pub close_button: Response
}

/// A selectable tab with a close button. Should be used inside `pierro::menu_bar`
pub fn tab<S: Into<String>>(ui: &mut UI, label_text: S, selected: bool) -> TabResponse {
    let tab_bg = if selected { ui.style::<theme::BgLight>() } else { ui.style::<theme::BgDark>() };
    let widget_margin = ui.style::<theme::WidgetMargin>();

    let (tab, close_button) = ui.with_node(
        UINodeParams::new(Size::fit(), Size::fit())
            .with_layout(Layout::horizontal())
            .with_margin(widget_margin)
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
