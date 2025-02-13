
use crate::{Axis, Id, Layout, LayoutInfo, Response, Size, TSTransform, UINodeParams, Vec2, UI};

use super::{button_fill_animation, v_spacing, Theme};

#[derive(Default)]
struct ScrollAreaMemory {
    scroll: Vec2,
    max_scroll: Vec2
}

pub struct ScrollArea {
    width: Size,
    height: Size,
    show_scroll_bars: bool
}

impl Default for ScrollArea {

    fn default() -> Self {
        Self {
            width: Size::fr(1.0),
            height: Size::fr(1.0),
            show_scroll_bars: true
        }
    }

}

fn handle_scroll_bar_dragging(ui: &mut UI, axis: Axis, thumb: Response, bar_id: Id, scroll_area_id: Id, max_scroll: Vec2) {
    let drag_delta = thumb.drag_delta(ui); 

    if thumb.drag_started() {
        thumb.request_focus(ui);
    }
    if thumb.drag_stopped() {
        thumb.release_focus(ui);
    }

    let theme = ui.style::<Theme>();
    let base_color = theme.bg_button;

    if thumb.dragging() {
        let thumb_size = ui.memory().get::<LayoutInfo>(thumb.id).rect.size().on_axis(axis);
        let bar_size = ui.memory().get::<LayoutInfo>(bar_id).rect.size().on_axis(axis);
        let drag_scale = max_scroll.on_axis(axis) / (bar_size - thumb_size); 

        let memory = ui.memory().get::<ScrollAreaMemory>(scroll_area_id);
        *memory.scroll.on_axis_mut(axis) += drag_delta.on_axis(axis) * drag_scale; 
    } 

    button_fill_animation(ui, thumb.node_ref, &thumb, base_color);
}

impl ScrollArea {

    pub fn with_size(mut self, width: Size, height: Size) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn hide_scroll_bars(mut self) -> Self {
        self.show_scroll_bars = false;
        self
    }

    pub fn render<F: FnOnce(&mut UI)>(self, ui: &mut UI, contents: F) -> Response {

        let scroll_area = ui.node(
            UINodeParams::new(self.width, self.height)
                .sense_scroll()
                .with_layout(Layout::horizontal())
        );

        let theme = ui.style::<Theme>();
        let scroll_thumb_color = theme.bg_button;
        let scroll_bar_size = 10.0;
        let scroll_thumb_size = 500.0;

        let (h_scroll_bar, v_scroll_bar) = ui.with_parent(scroll_area.node_ref, |ui| {

            let inner = ui.node(
                UINodeParams::new(self.width, self.height)
                    .with_layout(Layout::vertical())
            );

            let (scroll, max_scroll, show_h_scroll_bar, show_v_scroll_bar, h_scroll_bar) = ui.with_parent(inner.node_ref, |ui| {

                let content_response = ui.node(
                    UINodeParams::new(Size::fit(), Size::fr(1.0))
                        .with_layout(Layout::vertical().with_vertical_overflow().with_horizontal_overflow())
                );
                ui.with_parent(content_response.node_ref, contents);

                let layout_info = ui.memory().get::<LayoutInfo>(content_response.id);
                let max_scroll = (layout_info.children_base_size - layout_info.rect.size()).max(Vec2::ZERO);

                let memory = ui.memory().get::<ScrollAreaMemory>(scroll_area.id);
                memory.scroll -= scroll_area.scroll;
                memory.scroll = memory.scroll.min(max_scroll).max(Vec2::ZERO);
                memory.max_scroll = max_scroll;
                let scroll = memory.scroll;
                ui.set_transform(content_response.node_ref, TSTransform::translation(-scroll));

                let show_h_scroll_bar = max_scroll.x > 0.0 && self.show_scroll_bars;
                let show_v_scroll_bar = max_scroll.y > 0.0 && self.show_scroll_bars;

                let h_scroll_bar = if show_h_scroll_bar {
                    let scroll_bar = ui.node(
                        UINodeParams::new(Size::fr(1.0), Size::px(scroll_bar_size))
                            .with_layout(Layout::horizontal())
                    );
                    Some((scroll_bar, ui.with_parent(scroll_bar.node_ref, |ui| {
                        ui.node(UINodeParams::new(Size::fr(scroll.x), Size::fr(1.0)));
                        let h_scroll_thumb = ui.node(
                            UINodeParams::new(Size::fr(scroll_thumb_size), Size::fr(1.0))
                                .sense_mouse()
                                .with_fill(scroll_thumb_color)
                        );
                        ui.node(UINodeParams::new(Size::fr(max_scroll.x - scroll.x), Size::fr(1.0)));
                        h_scroll_thumb
                    })))
                } else {
                    None
                };

                (scroll, max_scroll, show_h_scroll_bar, show_v_scroll_bar, h_scroll_bar)
            });

            let v_scroll_bar = if show_v_scroll_bar {
                let scroll_bar = ui.node(UINodeParams::new(Size::px(scroll_bar_size), Size::fr(1.0)));
                Some((scroll_bar, ui.with_parent(scroll_bar.node_ref, |ui| {
                    ui.node(UINodeParams::new(Size::fr(1.0), Size::fr(scroll.y)));
                    let v_scroll_thumb = ui.node(
                        UINodeParams::new(Size::fr(1.0), Size::fr(scroll_thumb_size))
                            .sense_mouse()
                            .with_fill(scroll_thumb_color)
                    );
                    ui.node(UINodeParams::new(Size::fr(1.0), Size::fr(max_scroll.y - scroll.y)));
                    if show_h_scroll_bar {
                        v_spacing(ui, scroll_bar_size);
                    }
                    v_scroll_thumb
                })))
            } else { 
                None
            };

            (h_scroll_bar, v_scroll_bar)
        });

        let max_scroll = ui.memory().get::<ScrollAreaMemory>(scroll_area.id).max_scroll;

        if let Some((bar, thumb)) = h_scroll_bar {
            handle_scroll_bar_dragging(ui, Axis::X, thumb, bar.id, scroll_area.id, max_scroll);
        }
        if let Some((bar, thumb)) = v_scroll_bar {
            handle_scroll_bar_dragging(ui, Axis::Y, thumb, bar.id, scroll_area.id, max_scroll);
        }

        scroll_area
    }

}

pub fn scroll_area<F: FnOnce(&mut UI)>(ui: &mut UI, contents: F) {
    ScrollArea::default().render(ui, contents);
}
