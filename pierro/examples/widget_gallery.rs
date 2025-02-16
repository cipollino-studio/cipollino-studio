
struct Gallery {
    docking_state: pierro::DockingState<GalleryTab>
}

struct Basic {
    text_edit_text: String,
    checkbox_state: bool,
    dropdown_state: String
}

impl Basic {

    fn new() -> Self {
        Self {
            text_edit_text: "A text edit box".to_owned(),
            checkbox_state: true,
            dropdown_state: "Option A".to_owned()
        }
    }

    fn ui(&mut self, ui: &mut pierro::UI) {
        pierro::scroll_area(ui, |ui| {
            pierro::margin(ui, |ui| { 
                pierro::label(ui, "A label");
                pierro::v_spacing(ui, 10.0);

                pierro::button(ui, "A button");
                pierro::v_spacing(ui, 10.0);

                pierro::text_edit(ui, &mut self.text_edit_text);
                pierro::v_spacing(ui, 10.0);

                pierro::checkbox_labeled(ui, "A checkbox", &mut self.checkbox_state);
                pierro::v_spacing(ui, 10.0);

                let context_menu_response = pierro::label(ui, "Context menu (right click me!)");
                pierro::context_menu(ui, &context_menu_response, |ui| {
                    pierro::label(ui, "Inside the context menu");
                });
                pierro::v_spacing(ui, 10.0);

                pierro::collapsing_label(ui, "A collapsing header", |ui| {
                    for i in 0..50 {
                        pierro::label(ui, i.to_string());
                    }
                });
                pierro::v_spacing(ui, 10.0);

                pierro::dropdown(ui, self.dropdown_state.clone(), |ui| {
                    for option in ["Option A", "Option B", "Option C"] {
                        if pierro::menu_button(ui, option).mouse_clicked() {
                            self.dropdown_state = option.to_owned();
                        }
                    }
                });

            });
        });
    }

}

struct Layout {
    axis: pierro::Axis,
    justify: pierro::Justify,
    align: pierro::Align
}

impl Layout {
    
    fn new() -> Self {
        Self {
            axis: pierro::Axis::X,
            justify: pierro::Justify::Center,
            align: pierro::Align::Center
        }
    }

    fn node(&self, ui: &mut pierro::UI, color: pierro::Color) {
        ui.node(
            pierro::UINodeParams::new(pierro::Size::px(100.0), pierro::Size::px(100.0))
                .with_fill(color)
        );
    }

    fn ui(&mut self, ui: &mut pierro::UI) {

        pierro::margin(ui, |ui| {
            pierro::dropdown_labeled(ui, "Axis: ", match self.axis {
                pierro::Axis::X => "X",
                pierro::Axis::Y => "Y",
            }, |ui| {
                if pierro::menu_button(ui, "X").mouse_clicked() {
                    self.axis = pierro::Axis::X;
                }
                if pierro::menu_button(ui, "Y").mouse_clicked() {
                    self.axis = pierro::Axis::Y;
                }
            });
            pierro::v_spacing(ui, 5.0);

            pierro::dropdown_labeled(ui, "Justify: ", match self.justify {
                pierro::Justify::Min => "Min",
                pierro::Justify::Center => "Center",
                pierro::Justify::Max => "Max",
            }, |ui| {
                if pierro::menu_button(ui, "Min").mouse_clicked() {
                    self.justify = pierro::Justify::Min;
                }
                if pierro::menu_button(ui, "Center").mouse_clicked() {
                    self.justify = pierro::Justify::Center;
                }
                if pierro::menu_button(ui, "Max").mouse_clicked() {
                    self.justify = pierro::Justify::Max;
                }
            });
            pierro::v_spacing(ui, 5.0);

            pierro::dropdown_labeled(ui, "Align: ", match self.align {
                pierro::Align::Min => "Min",
                pierro::Align::Center => "Center",
                pierro::Align::Max => "Max",
            }, |ui| {
                if pierro::menu_button(ui, "Min").mouse_clicked() {
                    self.align = pierro::Align::Min;
                }
                if pierro::menu_button(ui, "Center").mouse_clicked() {
                    self.align = pierro::Align::Center;
                }
                if pierro::menu_button(ui, "Max").mouse_clicked() {
                    self.align = pierro::Align::Max;
                }
            });
        });

        pierro::h_divider(ui);
        pierro::container(ui,
            pierro::Size::fr(1.0),
            pierro::Size::fr(1.0),
            pierro::Layout::new(self.axis).with_justify(self.justify).with_align(self.align),
            |ui| {
                self.node(ui, pierro::Color::RED);   
                self.node(ui, pierro::Color::GREEN);   
                self.node(ui, pierro::Color::BLUE);
            });
    }

}

enum GalleryTab {
    Basic(Basic),
    Layout(Layout)
}

impl pierro::DockingTab for GalleryTab {

    type Context = ();

    fn title(&self) -> String {
        match self {
            GalleryTab::Basic(..) => "Basic Widgets".to_owned(),
            GalleryTab::Layout(..) => "Layout".to_owned()
        }
    }

    fn render(&mut self, ui: &mut pierro::UI, _context: &mut ()) {
            match self {
                GalleryTab::Basic(basic) => basic.ui(ui),
                GalleryTab::Layout(layout) => layout.ui(ui)
            }
    }

    fn add_tab_dropdown<F: FnMut(Self)>(ui: &mut pierro::UI, mut add_tab: F, _context: &mut ()) {
        if pierro::menu_button(ui, "Basic Widgets").mouse_clicked() {
            add_tab(Self::Basic(Basic::new()));
        }
        if pierro::menu_button(ui, "Layout").mouse_clicked() {
            add_tab(Self::Layout(Layout::new()));
        }
    }

}

impl pierro::App for Gallery {

    fn window_config() -> pierro::WindowConfig {
        pierro::WindowConfig::default()
            .with_title("Pierro Widget Gallery")
    }

    fn tick(&mut self, ui: &mut pierro::UI) {
        pierro::menu_bar(ui, |ui| {
            pierro::menu_bar_item(ui, "Menubar", |ui| {
                pierro::menu_button(ui, "Button A");
                pierro::menu_button(ui, "Button B");
                pierro::menu_button(ui, "Button C");
                pierro::menu_category(ui, "Category", |ui| {
                    pierro::menu_button(ui, "Button X");
                    pierro::menu_button(ui, "Button Y");
                });
            });
        });
        self.docking_state.render(ui, &mut ());
    }

}

fn main() {
    pierro::run(Gallery {
        docking_state: pierro::DockingState::new(vec![
            GalleryTab::Basic(Basic::new())
        ])
    });
}