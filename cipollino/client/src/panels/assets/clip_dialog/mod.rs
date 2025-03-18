

mod create_clip;
pub use create_clip::*;

mod properties;
pub use properties::*;

const FPS_OPTIONS: &[f32] = &[12.0, 18.0, 24.0, 30.0, 48.0, 60.0];

pub struct ClipProperties {
    pub name: String,
    pub length: u32,
    pub framerate: f32,
    pub width: u32,
    pub height: u32
}

pub struct ClipPropertiesResponse {
    pub name_response: pierro::TextEditResponse,
    pub length_response: pierro::DragValueResponse,
    pub framerate_changed: bool,
    pub width_response: pierro::DragValueResponse,
    pub height_response: pierro::DragValueResponse 
}

fn labeled<R, F: FnOnce(&mut pierro::UI) -> R>(ui: &mut pierro::UI, label: &str, contents: F) -> R {
    let (_, result) = pierro::horizontal_fit_centered(ui, |ui| {
        pierro::container(ui, pierro::Size::px(60.0), pierro::Size::fit(), pierro::Layout::horizontal().justify_max(), |ui| {
            pierro::label(ui, label);
        });

        pierro::h_spacing(ui, 4.0);

        contents(ui)
    });
    pierro::v_spacing(ui, 1.5);
    result
}

impl ClipProperties {

    pub fn new() -> Self {
        Self {
            name: "Clip".to_owned(),
            length: 100,
            framerate: 24.0,
            width: 1920,
            height: 1080
        }
    }

    pub fn render_ui(&mut self, ui: &mut pierro::UI) -> ClipPropertiesResponse {
        
        let name_response = labeled(ui, "Name:", |ui| {
            pierro::text_edit(ui, &mut self.name)
        });
        let (width_response, height_response) = labeled(ui, "Size:", |ui| {
            let width_changed = pierro::drag_value(ui, &mut self.width);
            pierro::h_spacing(ui, 3.0);
            let height_changed = pierro::drag_value(ui, &mut self.height);
            (width_changed, height_changed)
        });
        let length_response = labeled(ui, "Length:", |ui| {
            pierro::drag_value(ui, &mut self.length)
        });
        let framerate_changed = labeled(ui, "FPS:", |ui| {
            let mut framerate_changed = false;
            pierro::dropdown(ui, format!("{}", self.framerate), |ui| {
                for fps_option in FPS_OPTIONS {
                    if pierro::menu_button(ui, format!("{}", fps_option)).mouse_clicked() {
                        self.framerate = *fps_option;
                        framerate_changed = true;
                    }
                }
            });
            framerate_changed
        });

        ClipPropertiesResponse {
            name_response,
            length_response,
            framerate_changed,
            width_response,
            height_response,
        }
    }

}
