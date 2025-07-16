
use frame_area::FrameArea;
use framebar::Framebar;
use layers::{LayerDropLocation, LayerList};
use project::{alisa::AnyPtr, Clip, ClipInner, LayerParent, Ptr};

use crate::{LayerRenderList, RenderLayerKind};

use super::{Panel, PanelContext};

mod header;
mod framebar;
mod layers;
mod frame_area;

pub struct TimelinePanel {
    layers_width: f32,
    scroll_state: pierro::ScrollAreaState,

    clip_length_preview: u32,

    framebar: Framebar,

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
            layers_width: 150.0,
            scroll_state: pierro::ScrollAreaState::default(),

            clip_length_preview: 0,

            framebar: Framebar::new(),

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

    fn render(&mut self, ui: &mut pierro::UI, context: &mut PanelContext) {
        
        let project = &context.project;
        let editor = &mut context.editor;

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
        let Some(render_list) = context.layer_render_list else { return; };

        // Update active layer
        let mut found_active_layer = false;
        let mut first_layer = None;
        for render_layer in render_list.iter() {
            match render_layer.kind {
                RenderLayerKind::Layer(ptr, _) => {
                    if ptr == editor.active_layer {
                        found_active_layer = true;
                    }
                    if first_layer.is_none() {
                        first_layer = Some(ptr);
                    }
                },
                _ => {}
            }
        }
        if !found_active_layer {
            editor.active_layer = first_layer.unwrap_or(Ptr::null());
        }

        pierro::margin_with_size(ui, pierro::Margin::same(3.0), pierro::Size::fr(1.0), pierro::Size::fit(), |ui| {
            pierro::horizontal_centered(ui, |ui| {
                self.header(ui, project, editor, editor.open_clip, clip.inner.ptr(), clip_inner);
            });
        });
        pierro::h_line(ui);

        pierro::horizontal_fill(ui, |ui| {
            let mut layers_width = self.layers_width;
            let layers_scroll_response = pierro::resizable_panel(ui, pierro::Axis::X, &mut layers_width, |ui| {
                pierro::v_spacing(ui, Self::FRAMEBAR_HEIGHT);
                self.layers(ui, project, editor, &render_list, clip_inner)
            });
            let parent_id = ui.get_node_id(ui.curr_parent());
            let parent_width = ui.memory().get_opt::<pierro::LayoutInfo>(parent_id).map(|info| info.screen_rect.width()).unwrap_or(ui.window_size().x);
            if parent_width > 150.0 {
                self.layers_width = layers_width.max(100.0).min(parent_width - 50.0);
            } else {
                self.layers_width = parent_width * 2.0 / 3.0;
            }

            let (frame_container, _) = pierro::vertical_fill(ui, |_| {});
            let frame_container_width = ui.memory().get_opt::<pierro::LayoutInfo>(frame_container.id).map(|info| info.screen_rect.width()).unwrap_or(ui.window_size().x);
            let n_frames = (clip_inner.length + (frame_container_width / Self::FRAME_WIDTH).ceil() as u32).min(50000);

            let (framebar_scroll_response, frame_area_scroll_response) = ui.with_parent(frame_container.node_ref, |ui| {
                let framebar_response = self.framebar.render(ui, editor, context.systems, clip_inner, n_frames, &mut self.scroll_state);
                let frame_area_response = self.frame_area(ui, editor, project, &render_list, clip_inner, n_frames);
                (framebar_response, frame_area_response)
            });

            layers_scroll_response.sync(ui, &mut self.scroll_state);
            framebar_scroll_response.sync(ui, &mut self.scroll_state);
            frame_area_scroll_response.sync(ui, &mut self.scroll_state);
        });

    }

}

impl LayerRenderList<'_> {

    fn get_transfer_location(&self, drop_location: LayerDropLocation, clip_ptr: Ptr<Clip>, clip: &ClipInner) -> (LayerParent, usize) {
        if drop_location.render_list_idx == self.len() {
            return (clip_ptr.into(), clip.layers.as_slice().len()); 
        } 

        let render_layer = &self.layers[drop_location.render_list_idx];
        match &render_layer.kind {
            RenderLayerKind::Layer(_, layer) => {
                (layer.parent, render_layer.idx + if drop_location.above { 0 } else { 1 }) 
            },
            RenderLayerKind::AudioLayer(_, audio) => {
                (audio.parent, render_layer.idx + if drop_location.above { 0 } else { 1 }) 
            }
            RenderLayerKind::LayerGroup(layer_group_ptr, layer_group) => {
                if drop_location.above {
                    (layer_group.parent, render_layer.idx + 1)
                } else {
                    ((*layer_group_ptr).into(), 0)
                }
            }
        }
    }

}
