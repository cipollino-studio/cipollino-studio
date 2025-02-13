
struct TestApp {
    n: i32,
    zoom: f32,
    text: String
}

impl pierro::App for TestApp {

    fn window_config() -> pierro::WindowConfig {
        pierro::WindowConfig::default()
            .with_title("Pierro Test")
    }

    fn tick(&mut self, ui: &mut pierro::UI) {

        let bg_color = ui.style::<pierro::Theme>().bg_dark;
        let bg = ui.node(
            pierro::UINodeParams::new(pierro::Size::fr(1.0), pierro::Size::fr(1.0))
                .with_layout(pierro::Layout::vertical().with_vertical_overflow())
                .with_fill(bg_color)
                .with_transform(pierro::TSTransform::scale(self.zoom))
        );
        
        ui.with_parent(bg.node_ref, |ui| {

            pierro::menu_bar(ui, |ui| {
                pierro::menu_bar_item(ui, "File", |ui| {
                    if pierro::menu_button(ui, "New").mouse_clicked() {
                        println!("New!!");
                    }
                    if pierro::menu_button(ui, "Open").mouse_clicked() {
                        println!("Open!!");
                    }
                    pierro::menu_category(ui, "Recent", |ui| {
                        pierro::menu_button(ui, "A.txt");
                        pierro::menu_button(ui, "B.txt");
                        pierro::menu_button(ui, "C.txt");
                        pierro::menu_category(ui, "!!", |ui| {
                            pierro::menu_button(ui, "A.txt");
                            pierro::menu_button(ui, "B.txt");
                            pierro::menu_button(ui, "C.txt");
                        });
                    });
                    pierro::h_line(ui);
                    pierro::menu_button(ui, "XYZ").mouse_clicked();
                    pierro::menu_button(ui, "PQR").mouse_clicked();
                });
                pierro::menu_bar_item(ui, "Edit", |ui| {
                    pierro::menu_button(ui, "Undo").mouse_clicked();
                    pierro::menu_button(ui, "Redo").mouse_clicked();
                });
            });

            pierro::v_spacing(ui, 15.0);
            pierro::label(ui, "Above the scroll area");

            pierro::dnd_source(ui, "Hello".to_string(), |ui| {
                pierro::label(ui, "Drag me! (Hello)");
            });
            pierro::dnd_source(ui, "World".to_string(), |ui| {
                pierro::label(ui, "Drag me! (World)");
            });
            if let Some(message) = pierro::dnd_drop_zone::<String, _>(ui, |ui| {
                pierro::label(ui, "Drop here...");
            }).1 {
                println!("{}", message);
            }

            pierro::horizontal(ui, |ui| {
                if pierro::button(ui, "+").mouse_clicked() {
                    self.zoom *= 1.25;
                }
                if pierro::button(ui, "-").mouse_clicked() {
                    self.zoom /= 1.25;
                }
                
                let button_response = pierro::button(ui, "@");
                pierro::context_menu(ui, &button_response, |ui| {
                    pierro::label(ui, "Truly!!!");
                });
            });
            pierro::v_spacing(ui, 15.0);
            pierro::text_edit(ui, &mut self.text);
            pierro::v_spacing(ui, 15.0);

            pierro::scroll_area(ui, |ui| {

                pierro::label(ui, "Button #1: 🍊");
                if pierro::button(ui, "So long mom, I'm off to drop the bomb").mouse_clicked() {
                    self.n += 1;
                }

                pierro::label(ui, "Button #2:");
                if pierro::button(ui, "So long mom, I'm off to drop the bomb").mouse_clicked() {
                    self.n -= 1;
                }

                pierro::label(ui, "The quick brown fox jumped over the lazy dog. The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.The quick brown fox jumped over the lazy dog.");

                for i in 0..100 {
                    pierro::label(ui, format!("{} {}", i, self.n));
                }

            });

            pierro::v_spacing(ui, 15.0);
            pierro::label(ui, "Below the scroll area");
            pierro::label(ui, "Ниже области прокрутки");
            pierro::label(ui, "أسفل منطقة التمرير");
            pierro::v_spacing(ui, 15.0);

        });

    }

}

fn main() {
    pierro::run(TestApp {
        n: 0,
        zoom: 1.0,
        text: "Hello, World!".to_string() 
    });
}
