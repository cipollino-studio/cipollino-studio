
use std::any::{Any, TypeId};

use crate::{clickable_icon, h_line, h_spacing, horizontal_centered, icons, margin, theme, widget::label, window, Layout, LayoutInfo, Margin, Size, TSTransform, UINodeParams, Vec2, UI};
use super::{Window, WindowDyn};

pub(super) struct WindowInstance<C> {
    window: Box<dyn WindowDyn<Context = C>>,
    pos: Vec2,
    opened_on_last_frame: bool
}

impl<C: 'static> WindowInstance<C> {

    pub fn new<W: Window<Context = C>>(window: W) -> Self {
        Self::new_dyn(Box::new(window))
    }

    pub fn new_dyn(window: Box<dyn WindowDyn<Context = C>>) -> Self {
        WindowInstance {
            window,
            // Hack to get around the fact we don't know the window's size on the frame it first opens
            // Once we know the size, this will be set to position the window at the center of the screen
            // Until then, we set the position far outside the app's window so it won't be seen
            pos: Vec2::splat(100000.0),
            opened_on_last_frame: false
        }
    }

    fn render_contents(&mut self, ui: &mut UI, context: &mut C) -> (bool, bool) {
        let window = ui.curr_parent();
        let window_id = ui.get_node_id(window);
        let just_opened = !ui.memory().has::<LayoutInfo>(window_id);
        let window_size = ui.memory().get::<LayoutInfo>(window_id).rect.size();

        let window_margin = ui.style::<theme::WindowMargin>();
        let window_bar_bg = ui.style::<theme::BgDark>();

        let (window_bar, close_window) = ui.with_node(
            UINodeParams::new(Size::fr(1.0), Size::fit())
                .with_layout(Layout::horizontal())
                .with_margin(Margin::same(4.0))
                .with_fill(window_bar_bg)
                .sense_mouse(),
            |ui| {
                let close_window = clickable_icon(ui, icons::X).mouse_clicked();
                h_spacing(ui, 3.0);
                horizontal_centered(ui, |ui| {
                    label(ui, self.window.title());
                });
                h_spacing(ui, 3.0);

                close_window
            }
        );
        h_line(ui);

        if window_bar.drag_started() {
            window_bar.request_focus(ui);
        }
        if window_bar.drag_stopped() {
            window_bar.release_focus(ui);
        }
        self.pos += window_bar.drag_delta(ui);
        if window_size.x > 0.0 {
            self.pos = self.pos.max(Vec2::ZERO);
            self.pos = self.pos.min(ui.window_size() - window_size);
        }
        if self.opened_on_last_frame {
            self.pos = (ui.window_size() - window_size) / 2.0;
        }

        let mut window_wants_close = false; 
        margin(ui, window_margin, |ui| {
            self.window.render(ui, &mut window_wants_close, context);
        });

        if just_opened {
            ui.request_redraw();
        }
        self.opened_on_last_frame = just_opened;

        (close_window || window_wants_close, window_bar.mouse_pressed())
    }

    pub fn render(&mut self, ui: &mut UI, context: &mut C) -> (bool, bool) {
        let (layer, (close, bring_forward)) = ui.layer(|ui| {
            let (window_response, (close, bring_forward)) = window(ui, |ui| {
                let (close, bring_forward) = self.render_contents(ui, context);
                (close, bring_forward)
            });
            (close, bring_forward || window_response.mouse_pressed())
        });

        ui.set_transform(layer, TSTransform::translation(self.pos));

        (close, bring_forward)
    }

    pub fn type_id(&self) -> TypeId {
        self.window.type_id()
    }

    pub fn unique(&self) -> bool {
        self.window.unique()
    }

}
