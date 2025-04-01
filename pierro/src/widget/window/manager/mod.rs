
use crate::UI;

mod instance;
use instance::*;

pub trait Window {

    type Context<'ctx>;

    fn title(&self) -> impl Into<String>;
    fn render<'ctx>(&mut self, ui: &mut UI, close: &mut bool, context: &mut Self::Context<'ctx>);
    fn modal(&self) -> bool { false }
    fn use_margin(&self) -> bool { true }

}

pub struct WindowManager<W: Window> {
    windows: Vec<WindowInstance<W>>
}

impl<W: Window> WindowManager<W> {

    pub fn new() -> Self {
        Self {
            windows: Vec::new()
        }
    }

    pub fn render<'ctx>(&mut self, ui: &mut UI, context: &mut W::Context<'ctx>) {
        let mut to_close = None;
        let mut to_bring_forward = None;
        for (idx, window_instance) in self.windows.iter_mut().enumerate() {
            let (close, bring_forward) = window_instance.render(ui, context); 
            if close {
                to_close = Some(idx);
            }
            if bring_forward {
                to_bring_forward = Some(idx);
            }
        }

        if let Some(to_bring_forward) = to_bring_forward {
            let last_idx = self.windows.len() - 1;
            self.windows.swap(to_bring_forward, last_idx);
        }
        if let Some(to_close) = to_close {
            self.windows.remove(to_close);
        }
    }

    pub fn open_window(&mut self, window: W) {
        self.windows.push(WindowInstance::new(window));
    }

    pub fn iter(&self) -> impl Iterator<Item = &W> {
        self.windows.iter().map(|instance| &instance.window)
    }

}
