
mod assets;
pub use assets::*;

mod scene;
pub use scene::*;

use crate::EditorState;

pub trait Panel {

    fn title(&self) -> String;
    fn render(&mut self, ui: &mut pierro::UI, state: &mut EditorState);

}

pub struct EditorPanel {
    panel: Box<dyn Panel>
}

impl EditorPanel {

    pub fn new<P: Panel + Default + 'static>() -> Self {
        Self {
            panel: Box::new(P::default())
        }
    }

}

impl pierro::DockingTab for EditorPanel {
    type Context = EditorState;

    fn title(&self) -> String {
        self.panel.title() 
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut EditorState) {
        self.panel.render(ui, state); 
    }

    fn add_tab_dropdown<F: FnMut(Self)>(ui: &mut pierro::UI, mut add_tab: F, _context: &mut EditorState) {
        if pierro::menu_button(ui, "Assets").mouse_clicked() {
            add_tab(EditorPanel::new::<AssetsPanel>());
        }
        if pierro::menu_button(ui, "Scene").mouse_clicked() {
            add_tab(EditorPanel::new::<ScenePanel>());
        }
    }

}