
use crate::{clickable_icon, h_line, h_spacing, icon_gap, icons, margin, modal, theme, vertical_centered, widget::label, window, Layout, LayoutInfo, Margin, Response, Size, TSTransform, UINodeParams, Vec2, UI};
use super::Window;

pub(super) struct WindowInstance<W: Window> {
    pub(super) window: W,
    pos: Vec2,
    just_opened: bool
}

impl<W: Window> WindowInstance<W> {

    pub fn new(window: W) -> Self {
        Self {
            window,
            // Hack to get around the fact we don't know the window's size on the frame it first opens
            // Once we know the size, this will be set to position the window at the center of the screen
            // Until then, we set the position far outside the app's window so it won't be seen
            pos: Vec2::splat(100000.0),
            just_opened: true
        }
    }

    fn render_window_header(&mut self, ui: &mut UI) -> (Response, bool) {
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
                vertical_centered(ui, |ui| {
                    label(ui, self.window.title());
                });
                h_spacing(ui, 3.0);
                icon_gap(ui);

                close_window
            }
        );
        h_line(ui);
        (window_bar, close_window)
    }

    fn render_window_contents<'ctx>(&mut self, ui: &mut UI, context: &mut W::Context<'ctx>) -> (bool, bool) {
        let window = ui.curr_parent();
        let window_id = ui.get_node_id(window);
        let window_size = ui.memory().get::<LayoutInfo>(window_id).rect.size();
        let window_margin = if self.window.use_margin() { ui.style::<theme::WindowMargin>() } else { Margin::ZERO };
        
        let (window_bar, close_window) = self.render_window_header(ui);

        if window_bar.drag_started() {
            window_bar.request_focus(ui);
        }
        if window_bar.drag_stopped() {
            window_bar.release_focus(ui);
        }
        self.pos += window_bar.drag_delta(ui);
        if window_size.x > 0.0 && !self.just_opened {
            if self.pos.x > 10000.0 {
                self.pos = (ui.window_size() - window_size) / 2.0;
            }
            self.pos = self.pos.max(Vec2::ZERO);
            self.pos = self.pos.min(ui.window_size() - window_size);
        }

        let mut window_wants_close = false; 
        margin(ui, window_margin, |ui| {
            self.window.render(ui, &mut window_wants_close, context);
        });

        if self.just_opened {
            ui.request_redraw();
        }
        self.just_opened = false;

        (close_window || window_wants_close, window_bar.mouse_pressed())
    }

    fn render_window<'ctx>(&mut self, ui: &mut UI, context: &mut W::Context<'ctx>) -> (bool, bool) {
        let (layer, (close, bring_forward)) = ui.layer(|ui| {
            let (window_response, (close, bring_forward)) = window(ui, |ui| {
                let (close, bring_forward) = self.render_window_contents(ui, context);
                (close, bring_forward)
            });
            (close, bring_forward || window_response.mouse_pressed())
        });

        ui.set_transform(layer, TSTransform::translation(self.pos));

        (close, bring_forward)
    }

    fn render_modal<'ctx>(&mut self, ui: &mut UI, context: &mut W::Context<'ctx>) -> (bool, bool) {
        let window_margin = if self.window.use_margin() { ui.style::<theme::WindowMargin>() } else { Margin::ZERO };

        let (_, result) = modal(ui, |ui| {
            let (window_bar, close_window) = self.render_window_header(ui);
            let mut window_wants_close = false;
            margin(ui, window_margin, |ui| {
                self.window.render(ui, &mut window_wants_close, context);
            });
            (close_window || window_wants_close, window_bar.mouse_clicked())
        });

        result
    }

    pub fn render<'ctx>(&mut self, ui: &mut UI, context: &mut W::Context<'ctx>) -> (bool, bool) {
        if self.window.modal() {
            self.render_modal(ui, context)
        } else {
            self.render_window(ui, context)
        }
    }

}
