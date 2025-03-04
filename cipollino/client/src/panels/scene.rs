
use crate::State;

use super::Panel;

#[derive(Default)]
pub struct ScenePanel {

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
        let Some(_clip_inner) = project.client.get(clip.inner) else {
            pierro::centered(ui, |ui| {
                pierro::label(ui, "Clip loading...");
            });
            return;
        };

        pierro::canvas(ui, |ui, texture, _response| {
            let camera = malvina::Camera::new(0.0, 0.0, ui.scale_factor());
            renderer.render(ui.wgpu_device(), ui.wgpu_queue(), texture.texture(), camera);
        });
    }

}
