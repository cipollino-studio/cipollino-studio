
use crate::{vec2, Axis, Id, Layout, LayoutInfo, Response, Size, TSTransform, UINodeParams, UIRef, Vec2, UI};

use super::{button_fill_animation, v_spacing, Theme};

#[derive(Clone, Copy, Default)]
pub struct ScrollAreaState {
    scroll: Vec2,
    max_scroll: Vec2
}

impl ScrollAreaState {

    pub fn update_ui(&self, ui: &mut UI, response: &ScrollAreaResponse) {
        ui.set_transform(response.content_node_ref, TSTransform::translation(-self.scroll * response.scroll_mask));
        if let Some((min_spacer, max_spacer)) = response.h_scrollbar_spacers {
            ui.set_size(min_spacer, Size::fr(self.scroll.x), Size::fr(1.0));
            ui.set_size(max_spacer, Size::fr(self.max_scroll.x - self.scroll.x), Size::fr(1.0));
        }
        if let Some((min_spacer, max_spacer)) = response.v_scrollbar_spacers {
            ui.set_size(min_spacer, Size::fr(1.0), Size::fr(self.scroll.y));
            ui.set_size(max_spacer, Size::fr(1.0), Size::fr(self.max_scroll.y - self.scroll.y));
        }
    }

}

pub struct ScrollArea<'state> {
    width: Size,
    height: Size,
    layout: Layout,
    show_scroll_bars: bool,
    state: Option<&'state mut ScrollAreaState>,
    scroll_x: bool,
    scroll_y: bool,
    set_max_scroll: bool
}

pub struct ScrollAreaResponse {
    pub scroll_area: Response,
    content_node_ref: UIRef,
    scroll_mask: Vec2,
    h_scrollbar_spacers: Option<(UIRef, UIRef)>,
    v_scrollbar_spacers: Option<(UIRef, UIRef)>,
}

impl<'state> Default for ScrollArea<'state> {

    fn default() -> Self {
        Self {
            width: Size::fr(1.0),
            height: Size::fr(1.0),
            layout: Layout::vertical(),
            show_scroll_bars: true,
            state: None,
            scroll_x: true,
            scroll_y: true,
            set_max_scroll: true 
        }
    }

}

fn handle_scroll_bar_dragging(ui: &mut UI, axis: Axis, thumb: Response, bar_id: Id, max_scroll: Vec2) -> f32 {
    let drag_delta = thumb.drag_delta(ui); 

    if thumb.drag_started() {
        thumb.request_focus(ui);
    }
    if thumb.drag_stopped() {
        thumb.release_focus(ui);
    }

    let theme = ui.style::<Theme>();
    let base_color = theme.bg_button;

    button_fill_animation(ui, thumb.node_ref, &thumb, base_color);

    if thumb.dragging() {
        let thumb_size = ui.memory().get::<LayoutInfo>(thumb.id).rect.size().on_axis(axis);
        let bar_size = ui.memory().get::<LayoutInfo>(bar_id).rect.size().on_axis(axis);
        let drag_scale = max_scroll.on_axis(axis) / (bar_size - thumb_size); 

        drag_delta.on_axis(axis) * drag_scale
    } else {
        0.0
    } 
}

impl<'state> ScrollArea<'state> {

    pub fn with_size(mut self, width: Size, height: Size) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn hide_scroll_bars(mut self) -> Self {
        self.show_scroll_bars = false;
        self
    }

    pub fn with_state(mut self, state: &'state mut ScrollAreaState) -> Self {
        self.state = Some(state);
        self
    }

    pub fn scroll_x(mut self, scroll: bool) -> Self {
        self.scroll_x = scroll;
        self
    }

    pub fn scroll_y(mut self, scroll: bool) -> Self {
        self.scroll_y = scroll;
        self
    }

    pub fn no_set_max_scroll(mut self) -> Self {
        self.set_max_scroll = false;
        self
    }

    fn scroll_mask(&self) -> Vec2 {
        vec2(
            if self.scroll_x { 1.0 } else { 0.0 }, 
            if self.scroll_y { 1.0 } else { 0.0 }, 
        )
    }

    pub fn render<F: FnOnce(&mut UI)>(self, ui: &mut UI, contents: F) -> ScrollAreaResponse {

        let scroll_area = ui.node(
            UINodeParams::new(self.width, self.height)
                .sense_scroll()
                .with_layout(Layout::horizontal())
        );

        let theme = ui.style::<Theme>();
        let scroll_thumb_color = theme.bg_button;
        let scroll_bar_size = 10.0;
        let scroll_thumb_size = 500.0;

        let mut max_scroll = Vec2::ZERO;
        let mut content_node_ref = UIRef::Null;
        let mut show_h_scrollbar = false;
        let mut show_v_scrollbar = false;
        let mut h_scrollbar = None;
        let mut v_scrollbar = None;
        let mut h_scrollbar_spacers = None;
        let mut v_scrollbar_spacers = None;

        // Construct the UI tree of the scroll area
        ui.with_parent(scroll_area.node_ref, |ui| {

            let inner = ui.node(
                UINodeParams::new(self.width, self.height)
                    .with_layout(Layout::vertical())
            );

            ui.with_parent(inner.node_ref, |ui| {

                let content_response = ui.node(
                    UINodeParams::new(Size::fit(), Size::fr(1.0))
                        .with_layout(self.layout.with_vertical_overflow().with_horizontal_overflow())
                );
                ui.with_parent(content_response.node_ref, contents);

                let layout_info = ui.memory().get::<LayoutInfo>(content_response.id);
                max_scroll = (layout_info.children_base_size - layout_info.rect.size()).max(Vec2::ZERO);
                
                content_node_ref = content_response.node_ref;

                show_h_scrollbar = max_scroll.x > 0.5 && self.show_scroll_bars;
                show_v_scrollbar = max_scroll.y > 0.5 && self.show_scroll_bars;

                if show_h_scrollbar {
                    let scroll_bar = ui.node(
                        UINodeParams::new(Size::fr(1.0), Size::px(scroll_bar_size))
                            .with_layout(Layout::horizontal())
                    );
                    let scroll_thumb = ui.with_parent(scroll_bar.node_ref, |ui| {
                        let min_spacer = ui.node(UINodeParams::new(Size::fr(1.0), Size::fr(1.0))).node_ref;
                        let h_scroll_thumb = ui.node(
                            UINodeParams::new(Size::fr(scroll_thumb_size), Size::fr(1.0))
                                .sense_mouse()
                                .with_fill(scroll_thumb_color)
                        );
                        let max_spacer = ui.node(UINodeParams::new(Size::fr(1.0), Size::fr(1.0))).node_ref;
                        h_scrollbar_spacers = Some((min_spacer, max_spacer));
                        h_scroll_thumb
                    });
                    h_scrollbar = Some((scroll_bar, scroll_thumb)); 
                }

            });

            if show_v_scrollbar {
                let scroll_bar = ui.node(UINodeParams::new(Size::px(scroll_bar_size), Size::fr(1.0)));
                let scroll_thumb = ui.with_parent(scroll_bar.node_ref, |ui| {
                    let min_spacer = ui.node(UINodeParams::new(Size::fr(1.0), Size::fr(1.0))).node_ref;
                    let v_scroll_thumb = ui.node(
                        UINodeParams::new(Size::fr(1.0), Size::fr(scroll_thumb_size))
                            .sense_mouse()
                            .with_fill(scroll_thumb_color)
                    );
                    let max_spacer = ui.node(UINodeParams::new(Size::fr(1.0), Size::fr(1.0))).node_ref;
                    v_scrollbar_spacers = Some((min_spacer, max_spacer));
                    if show_h_scrollbar {
                        v_spacing(ui, scroll_bar_size);
                    }
                    v_scroll_thumb
                });
                v_scrollbar = Some((scroll_bar, scroll_thumb));
            } 
        });

        let drag_delta_x = if let Some((bar, thumb)) = h_scrollbar {
            handle_scroll_bar_dragging(ui, Axis::X, thumb, bar.id, max_scroll)
        } else {
            0.0
        };
        let drag_delta_y = if let Some((bar, thumb)) = v_scrollbar {
            handle_scroll_bar_dragging(ui, Axis::Y, thumb, bar.id, max_scroll)
        } else {
            0.0
        };

        // Update the state
        let scroll_mask = self.scroll_mask();
        let state = self.state.unwrap_or_else(|| ui.memory().get::<ScrollAreaState>(scroll_area.id));
        state.scroll -= (scroll_area.scroll - vec2(drag_delta_x, drag_delta_y)) * scroll_mask;
        if self.set_max_scroll {
            state.max_scroll = max_scroll;
        }
        if self.scroll_x {
            state.scroll.x = state.scroll.x.min(state.max_scroll.x).max(0.0);
        }
        if self.scroll_y { 
            state.scroll.y = state.scroll.y.min(state.max_scroll.y).max(0.0);
        }

        let response = ScrollAreaResponse {
            scroll_area,
            content_node_ref,
            scroll_mask,
            h_scrollbar_spacers,
            v_scrollbar_spacers,
        };

        let state = *state;
        state.update_ui(ui, &response);

        response
    }

}

pub fn scroll_area<F: FnOnce(&mut UI)>(ui: &mut UI, contents: F) {
    ScrollArea::default().render(ui, contents);
}
