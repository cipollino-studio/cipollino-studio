
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

    fn add_tab_dropdown<F: FnMut(Self)>(ui: &mut pierro::UI, add_tab: F, context: &mut EditorState) {

    }

}