
mod toolbar;
mod canvas;

use crate::State;
use super::Panel;

pub struct ScenePanel {
    cam_pos: malvina::Vec2,
    cam_size: f32
}

impl Default for ScenePanel {

    fn default() -> Self {
        Self {
            cam_pos: malvina::Vec2::ZERO,
            cam_size: 2.0
        }
    }

}

impl Panel for ScenePanel {

    const NAME: &'static str = "Scene";

    fn title(&self) -> String {
        "Scene".to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut State) {

        let project = &state.project;
        let editor = &mut state.editor;
        
        if state.renderer.is_none() {
            state.renderer = Some(malvina::Renderer::new(ui.wgpu_device()));
        }
        let Some(renderer) = state.renderer.as_mut() else {
            return;
        };

        let Some(clip) = state.project.client.get(editor.open_clip) else {
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

        pierro::horizontal_fill(ui, |ui| {
            self.toolbar(ui, editor);
            pierro::v_line(ui);
            self.canvas(ui, project, editor, renderer, clip_inner); 
        });
        
    }

}
