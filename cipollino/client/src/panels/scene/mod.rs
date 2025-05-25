
mod toolbar;
mod canvas;

use crate::{keyboard_shortcut, Shortcut};
use super::{Panel, PanelContext};

use std::cell::RefCell;
use std::rc::Rc;

mod onion_skin_colors;
pub use onion_skin_colors::*;

keyboard_shortcut!(RecenterSceneShortcut, G, pierro::KeyModifiers::CONTROL);
keyboard_shortcut!(MirrorSceneShortcut, M, pierro::KeyModifiers::CONTROL);

pub struct ScenePanel {
    cam_pos: malvina::Vec2,
    cam_size: f32,
    mirror: bool,

    picking_buffer: Rc<RefCell<malvina::PickingBuffer>>,
}

impl Default for ScenePanel {

    fn default() -> Self {
        Self {
            cam_pos: malvina::Vec2::ZERO,
            cam_size: 2.0,
            mirror: false,
            picking_buffer: Rc::new(RefCell::new(malvina::PickingBuffer::new())),
        }
    }

}

impl Panel for ScenePanel {

    const NAME: &'static str = "Scene";

    fn title(&self) -> String {
        "Scene".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, context: &mut PanelContext) {

        let project = &context.project;
        let editor = &mut context.editor;
        
        if context.renderer.is_none() {
            *context.renderer = Some(malvina::Renderer::new(ui.wgpu_device(), ui.wgpu_queue()));
        }
        let Some(renderer) = context.renderer.as_mut() else {
            return;
        };

        let Some(clip) = context.project.client.get(editor.open_clip) else {
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
        let Some(scene_render_list) = context.scene_render_list else { return; }; 

        pierro::horizontal_fill(ui, |ui| {
            self.toolbar(ui, &context.project.client, editor, context.systems);
            pierro::v_line(ui);
            self.canvas(ui, project, editor, context.systems, renderer, clip_inner, scene_render_list); 
        });

        if RecenterSceneShortcut::used_globally(ui, context.systems) {
            self.cam_pos = elic::Vec2::ZERO;
            self.cam_size = 2.0;
        }
        if MirrorSceneShortcut::used_globally(ui, context.systems) {
            self.mirror = !self.mirror;
            self.cam_pos.x *= -1.0;
        }
        
    }

}
