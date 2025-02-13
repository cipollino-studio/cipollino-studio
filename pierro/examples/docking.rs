
#[derive(Clone)]
struct DockingTab {
    text: &'static str,
    n: i32
}

impl pierro::DockingTab for DockingTab {

    type Context = ();

    fn title(&self) -> String {
        self.text.to_owned()
    }

    fn render(&mut self, ui: &mut pierro::UI, _context: &mut ()) {
        pierro::label(ui, format!("{}: {}", self.text, self.n));
        if pierro::button(ui, "Add 100!").mouse_clicked() {
            self.n += 100;
        }
    }
    
    fn add_tab_dropdown<F: FnMut(Self)>(ui: &mut pierro::UI, mut add_tab: F, _context: &mut ()) {
        if pierro::menu_button(ui, "Hey").mouse_clicked() {
            add_tab(Self {
                text: "Hey",
                n: 0,
            });
        }
        if pierro::menu_button(ui, "Goodbye").mouse_clicked() {
            add_tab(Self {
                text: "Goodbye",
                n: 123,
            });
        }
    }

}

struct DockingApp {
    docking_state: pierro::DockingState<DockingTab>,
}

impl pierro::App for DockingApp {

    fn window_config() -> pierro::WindowConfig {
        pierro::WindowConfig::default()
            .with_title("Pierro Docking")
    }

    fn tick(&mut self, ui: &mut pierro::UI) {
        pierro::menu_bar(ui, |ui| {
            pierro::menu_bar_item(ui, "Menu", |ui| {
                pierro::menu_button(ui, "Some");
                pierro::menu_button(ui, "Stuff");
                pierro::menu_button(ui, "Here");
            });
        });
        self.docking_state.render(ui, &mut ());
    }

}

fn main() {
    pierro::run(DockingApp {
        docking_state: pierro::DockingState::new(vec![
            DockingTab { text: "Hello", n: 0 },
            DockingTab { text: "World", n: 0 },
        ]),
    });
}
