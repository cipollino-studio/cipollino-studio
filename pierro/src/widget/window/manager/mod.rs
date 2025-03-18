
use std::any::{Any, TypeId};

use crate::UI;

mod instance;
use instance::*;

pub trait Window: Any {

    type Context;

    /// True if there can be at most one of this kind of window open at once.
    const UNIQUE: bool = false;

    fn title(&self) -> impl Into<String>;
    fn render(&mut self, ui: &mut UI, close: &mut bool, context: &mut Self::Context);

}

pub trait WindowDyn {

    type Context;    

    fn title(&self) -> String;
    fn render(&mut self, ui: &mut UI, close: &mut bool, context: &mut Self::Context);
    fn unique(&self) -> bool;

}

impl<W: Window> WindowDyn for W {
    type Context = <Self as Window>::Context;

    fn title(&self) -> String {
        self.title().into()
    }

    fn render(&mut self, ui: &mut UI, close: &mut bool, context: &mut Self::Context) {
        self.render(ui, close, context);
    }

    fn unique(&self) -> bool {
        Self::UNIQUE
    }

}

pub struct WindowManager<C> {
    windows: Vec<WindowInstance<C>>
}

impl<C: 'static> WindowManager<C> {

    pub fn new() -> Self {
        Self {
            windows: Vec::new()
        }
    }

    pub fn render(&mut self, ui: &mut UI, context: &mut C) {
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

    pub fn open_window<W: Window<Context = C>>(&mut self, window: W) {
        if W::UNIQUE {
            for open_window in &self.windows {
                if open_window.type_id() == TypeId::of::<W>() {
                    return;
                }
            }
        }
        self.windows.push(WindowInstance::new(window));
    }

    pub fn open_window_dyn(&mut self, window_dyn: Box<dyn WindowDyn<Context = C>>) {
        let instance = WindowInstance::new_dyn(window_dyn); 
        if instance.unique() {
            for open_window in &self.windows {
                if open_window.type_id() == instance.type_id() {
                    return;
                }
            }
        }
        self.windows.push(instance);
    }

}
