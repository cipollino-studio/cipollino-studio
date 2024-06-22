
use crate::app::{editor::EditorState, AppSystems};

mod assets;
mod scene;

pub trait Panel {

    fn ui(&mut self, ui: &mut egui::Ui, state: &mut EditorState, systems: &mut AppSystems);

}

// Used for serialization
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub enum PanelKind {
    Assets,
    Scene
}

pub const PANEL_KINDS: [PanelKind; 2] = [PanelKind::Assets, PanelKind::Scene];

impl PanelKind {

    fn make_panel(&self) -> Box<dyn Panel> {
        match self {
            PanelKind::Assets => Box::new(assets::Assets::default()),
            PanelKind::Scene => Box::new(scene::Scene::default()),
        }
    }

    pub fn title(&self) -> &str {
        match self {
            PanelKind::Assets => "Assets",
            PanelKind::Scene => "Scene",
        }
    }

}

pub struct PanelViewer<'a, 'b> {
    state: &'a mut EditorState,
    systems: &'a mut AppSystems<'b>,
    disabled: bool
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Tab {
    id: u64,
    kind: PanelKind,

    // Wrapped in an option to make it possible to deserialize with serde
    #[serde(skip)]
    panel: Option<Box<dyn Panel>>
}

impl Tab {
    
    fn ensure_panel_exists(&mut self) {
        if self.panel.is_none() {
            self.panel = Some(self.kind.make_panel());
        }
    }

    fn get_panel(&mut self) -> &mut Box<dyn Panel> {
        self.ensure_panel_exists();
        return self.panel.as_mut().unwrap();
    }

}

impl egui_dock::TabViewer for PanelViewer<'_, '_> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.kind.title().into()
    }

    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(tab.id)
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        ui.set_enabled(!self.disabled);
        tab.get_panel().ui(ui, self.state, self.systems);
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PanelManager {
    tree: egui_dock::DockState<Tab>,
    curr_id: u64
}

impl PanelManager {

    pub fn new() -> Self {
        let tree = egui_dock::DockState::new(vec![]);
        Self {
            tree,
            curr_id: 0
        }
    }

    pub fn add_panel(&mut self, panel: PanelKind) {
        self.tree.add_window(vec![
            Tab { id: self.curr_id, kind: panel, panel: Some(panel.make_panel()) }
        ]);
        self.curr_id += 1;
    }

    pub fn update(&mut self, ctx: &egui::Context, state: &mut EditorState, systems: &mut AppSystems, disabled: bool) {
        egui_dock::DockArea::new(&mut self.tree)
            .style(egui_dock::Style::from_egui(ctx.style().as_ref()))
            .draggable_tabs(!disabled)
            .show_close_buttons(!disabled)
            .show(ctx, &mut PanelViewer {
                state,
                systems,
                disabled
            });
    }

}
