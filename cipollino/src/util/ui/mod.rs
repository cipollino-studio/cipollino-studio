
pub mod keybind;
pub mod dnd;

pub fn clickable_label<T>(ui: &mut egui::Ui, msg: T) -> egui::Response where T: Into<egui::WidgetText> {
    ui.add(egui::Label::new(msg).selectable(false).sense(egui::Sense::click()))
}

pub fn error_label<T>(ui: &egui::Ui, text: T) -> egui::Label where T: Into<String> {
    egui::Label::new(egui::RichText::new(text.into()).color(ui.style().visuals.error_fg_color))
}

pub fn key_value_layout<F>(ui: &mut egui::Ui, contents: F) where F: FnOnce(&mut egui::Ui) {
    egui::Grid::new(ui.next_auto_id()).num_columns(2).show(ui, |ui| {
        contents(ui);
    });
}

pub fn key_value_row<T, F>(ui: &mut egui::Ui, label: T, value: F) where T: Into<egui::WidgetText>, F: FnOnce(&mut egui::Ui) {
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(label);
    });
    value(ui);
    ui.end_row();
}

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch = "wasm32"))]
pub fn path_selector<F>(ui: &mut egui::Ui, path: &mut PathBuf, pick_folder: bool, correct_path: F) where F: Fn(&mut PathBuf) {
    use std::str::FromStr;

    ui.horizontal(|ui| {
        let mut path_as_string = path.to_str().unwrap().to_owned();
        let text_edit_resp = ui.text_edit_singleline(&mut path_as_string);
        *path = PathBuf::from_str(&path_as_string).unwrap();
        if text_edit_resp.lost_focus() {
            correct_path(path);
        }

        if ui.button(egui_phosphor::regular::FOLDER).clicked() {
            if pick_folder {
                if let Some(new_path) = tinyfiledialogs::select_folder_dialog("", "") { 
                    if let Ok(mut new_path) = PathBuf::from_str(&new_path) {
                        correct_path(&mut new_path);
                        *path = new_path;
                    }
                }
            } else if let Some(new_path) = tinyfiledialogs::save_file_dialog("", "") {
                if let Ok(mut new_path) = PathBuf::from_str(&new_path) {
                    correct_path(&mut new_path);
                    *path = new_path;
                }
            }
        }
    });
}