
mod toolbar;
mod canvas;

use project::{Action, DeleteStroke};

use crate::{EditorState, ProjectState};
use super::{Panel, PanelContext};

use std::cell::RefCell;
use std::rc::Rc;

pub const ONION_SKIN_PREV_COLOR: pierro::Color = pierro::Color::rgb(0.8588, 0.3764, 0.8196);
pub const ONION_SKIN_NEXT_COLOR: pierro::Color = pierro::Color::rgb(0.4666, 0.8588, 0.3764);

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

impl ScenePanel {

    fn delete_scene_selection(project: &ProjectState, editor: &mut EditorState) {
        let mut action = Action::new(editor.action_context("Delete Strokes"));
        for stroke in editor.selection.iter() {
            action.push(DeleteStroke {
                ptr: stroke
            });
        }
        project.client.queue_action(action);
        editor.selection.clear();
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

        pierro::horizontal_fill(ui, |ui| {
            self.toolbar(ui, editor);
            pierro::v_line(ui);
            self.canvas(ui, project, editor, context.systems, renderer, clip_inner); 
        });

        // Delete scene selection
        let delete_shortcut = pierro::KeyboardShortcut::new(pierro::KeyModifiers::empty(), pierro::Key::Backspace);
        if delete_shortcut.used_globally(ui) {
            Self::delete_scene_selection(project, editor);
        }
        
    }

}
