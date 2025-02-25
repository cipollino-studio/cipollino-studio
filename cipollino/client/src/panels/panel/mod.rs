use crate::State;

use super::PANEL_KINDS;

mod serialization;

pub trait Panel {

    const NAME: &'static str;

    fn title(&self) -> String;
    fn render(&mut self, ui: &mut pierro::UI, state: &mut State);

}

trait PanelDyn {
    fn title(&self) -> String;
    fn render(&mut self, ui: &mut pierro::UI, state: &mut State);
    fn name(&self) -> &'static str;
}

impl<P: Panel> PanelDyn for P {

    fn title(&self) -> String {
        self.title()
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut State) {
        self.render(ui, state);
    }

    fn name(&self) -> &'static str {
        Self::NAME
    }

}

pub struct PanelKind {
    name: &'static str,
    make_panel: fn() -> EditorPanel
}

impl PanelKind {

    pub const fn of<P: Panel + Default + 'static>() -> Self {
        Self {
            name: P::NAME,
            make_panel: || EditorPanel::new::<P>()
        }
    }

}


pub struct EditorPanel {
    panel: Box<dyn PanelDyn>
}

impl EditorPanel {

    pub fn new<P: Panel + Default + 'static>() -> Self {
        Self {
            panel: Box::new(P::default())
        }
    }

}

impl pierro::DockingTab for EditorPanel {
    type Context = State;

    fn title(&self) -> String {
        self.panel.title() 
    }

    fn render(&mut self, ui: &mut pierro::UI, state: &mut State) {
        self.panel.render(ui, state); 
    }

    fn add_tab_dropdown<F: FnMut(Self)>(ui: &mut pierro::UI, mut add_tab: F, _context: &mut State) {
        for panel_kind in PANEL_KINDS {
            if pierro::menu_button(ui, panel_kind.name).mouse_clicked() {
                add_tab((panel_kind.make_panel)());
            }
        } 
    }

}
