use frame_area::FrameArea;
use layers::{LayerDropLocation, LayerList};
use project::{alisa::AnyPtr, Ptr};
use render_list::RenderList;

use crate::State;

use super::Panel;

mod render_list;
mod header;
mod framebar;
mod layers;
mod frame_area;

pub struct TimelinePanel {
    layers_width: f32,
    scroll_state: pierro::ScrollAreaState,

    clip_length_preview: u32,

    renaming_state: Option<(AnyPtr, String)>,
    started_renaming: bool,

    layer_dnd_source: pierro::DndSource,
    layer_dnd_hover_pos: Option<LayerDropLocation>,
    layer_dnd_dropped_payload: Option<LayerList>,

    frame_area: FrameArea
}

impl Default for TimelinePanel {

    fn default() -> Self {
        Self {
            layers_width: 100.0,
            scroll_state: pierro::ScrollAreaState::default(),

            clip_length_preview: 0,

            renaming_state: None,
            started_renaming: false,

            layer_dnd_source: pierro::DndSource::new(),
            layer_dnd_hover_pos: None,
            layer_dnd_dropped_payload: None,

            frame_area: FrameArea::new()
        }
    }

}

impl Panel for TimelinePanel {

    const NAME: &'static str = "Timeline";

    fn title(&self) -> String {
        "Timeline".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut State) {
        
        let project = &state.project;
        let editor = &mut state.editor;

        let Some(clip) = project.client.get(editor.open_clip) else {
            pierro::centered(ui, |ui| {
                pierro::label(ui, "No clip open.");
            });
            return;
        };
        let Some(clip_inner) = project.client.get(clip.inner) else {
            pierro::centered(ui, |ui| {
                pierro::label(ui, "Clip loading...");
            });
            return;
        };

        let render_list = RenderList::make(&project.client, clip_inner);

        // Update active layer
        let mut found_active_layer = false;
        let mut first_layer = None;
        for render_layer in render_list.iter() {
            match render_layer.kind {
                render_list::RenderLayerKind::Layer(ptr, _) => {
                    if ptr == editor.active_layer {
                        found_active_layer = true;
                    }
                    if first_layer.is_none() {
                        first_layer = Some(ptr);
                    }
                },
            }
        }
        if !found_active_layer {
            editor.active_layer = first_layer.unwrap_or(Ptr::null());
        }

        pierro::margin_with_size(ui, pierro::Margin::same(3.0), pierro::Size::fr(1.0), pierro::Size::fit(), |ui| {
            pierro::horizontal_centered(ui, |ui| {
                self.header(ui, project, editor, editor.open_clip, clip.inner, clip_inner);
            });
        });
        pierro::h_line(ui);

        pierro::horizontal_fill(ui, |ui| {
            let mut layers_width = self.layers_width;
            let layers_scroll_response = pierro::resizable_panel(ui, pierro::Axis::X, &mut layers_width, |ui| {
                pierro::v_spacing(ui, Self::FRAMEBAR_HEIGHT);
                self.layers(ui, project, editor, &render_list)
            });
            self.layers_width = layers_width;

            let (frame_container, _) = pierro::vertical_fill(ui, |_| {});
            let frame_container_width = ui.memory().get::<pierro::LayoutInfo>(frame_container.id).screen_rect.width();
            let n_frames = clip_inner.length + (frame_container_width / Self::FRAME_WIDTH).ceil() as u32;

            let (framebar_scroll_response, frame_area_scroll_response) = ui.with_parent(frame_container.node_ref, |ui| {
                let framebar_response = self.framebar(ui, editor, clip_inner, n_frames);
                let frame_area_response = self.frame_area(ui, editor, project, &render_list, clip_inner, n_frames);
                (framebar_response, frame_area_response)
            });

            layers_scroll_response.sync(ui, &mut self.scroll_state);
            framebar_scroll_response.sync(ui, &mut self.scroll_state);
            frame_area_scroll_response.sync(ui, &mut self.scroll_state);
        });

    }

}
