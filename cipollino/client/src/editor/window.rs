
use std::any::{Any, TypeId};

use crate::PanelContext;

pub trait Window: Any {

    fn title(&self) -> String;
    fn render<'ctx>(&mut self, ui: &mut pierro::UI, close: &mut bool, context: &mut PanelContext<'ctx>);
    fn modal(&self) -> bool { false }
    fn unique(&self) -> bool { false }
    fn use_margin(&self) -> bool { true }
    
}

pub struct WindowInstance {
    window: Box<dyn Window>,
    type_id: TypeId
}

impl WindowInstance {

    pub fn new<W: Window + 'static>(window: W) -> Self {
        Self {
            window: Box::new(window),
            type_id: TypeId::of::<W>()
        }
    }

    pub fn unique(&self) -> bool {
        self.window.unique()
    }

    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

}

impl pierro::Window for WindowInstance {
    type Context<'ctx> = PanelContext<'ctx>;

    fn title(&self) -> impl Into<String> {
        self.window.title()
    }

    fn render<'ctx>(&mut self, ui: &mut pierro::UI, close: &mut bool, context: &mut PanelContext<'ctx>) {
        self.window.render(ui, close, context);
    }

    fn modal(&self) -> bool {
        self.window.modal()
    }

    fn use_margin(&self) -> bool {
        self.window.use_margin() 
    }

}