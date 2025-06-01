use crate::{AppSystems, EditorState, LayerRenderList, ProjectState, RendererState, SceneRenderList};

use super::PANEL_KINDS;

mod serialization;

pub struct PanelContext<'ctx> {
    pub editor: &'ctx mut EditorState,
    pub project: &'ctx ProjectState,
    pub systems: &'ctx mut AppSystems,
    pub renderer: &'ctx mut Option<RendererState>,
    pub layer_render_list: Option<&'ctx LayerRenderList<'ctx>>,
    pub scene_render_list: Option<&'ctx SceneRenderList> 
}

pub trait Panel {

    const NAME: &'static str;

    fn title(&self) -> String;
    fn render<'ctx>(&mut self, ui: &mut pierro::UI, context: &mut PanelContext<'ctx>);

}

trait PanelDyn {
    fn title(&self) -> String;
    fn render<'ctx>(&mut self, ui: &mut pierro::UI, context: &mut PanelContext<'ctx>);
    fn name(&self) -> &'static str;
}

impl<P: Panel> PanelDyn for P {

    fn title(&self) -> String {
        self.title()
    }

    fn render<'ctx>(&mut self, ui: &mut pierro::UI, context: &mut PanelContext<'ctx>) {
        self.render(ui, context);
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
    type Context<'ctx> = PanelContext<'ctx>;

    fn title(&self) -> String {
        self.panel.title() 
    }

    fn render<'ctx>(&mut self, ui: &mut pierro::UI, context: &mut PanelContext<'ctx>) {
        self.panel.render(ui, context); 
    }

    fn add_tab_dropdown<'ctx, F: FnMut(Self)>(ui: &mut pierro::UI, mut add_tab: F, _context: &mut PanelContext<'ctx>) {
        for panel_kind in PANEL_KINDS {
            if pierro::menu_button(ui, panel_kind.name).mouse_clicked() {
                add_tab((panel_kind.make_panel)());
            }
        } 
    }

}
