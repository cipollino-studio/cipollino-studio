
use progress_modal::ExportProgressModal;

use super::State;

mod progress_modal;

mod video_writer;
use video_writer::*;

pub(super) struct ExportDialog {
    export_path: String,
    scale: f32
}

impl ExportDialog {

    pub fn new() -> Self {
        Self {
            export_path: String::new(),
            scale: 1.0
        }
    }

}

impl pierro::Window for ExportDialog {

    type Context = State;

    const UNIQUE: bool = true;

    fn title(&self) -> impl Into<String> {
        "Export"
    }

    fn render(&mut self, ui: &mut pierro::UI, close: &mut bool, state: &mut State) {
        let Some(clip) = state.project.client.get(state.editor.open_clip) else {
            pierro::margin(ui, pierro::Margin::same(10.0), |ui| {
                pierro::label(ui, "No clip open.");
            });
            return;
        };
        let Some(clip_inner) = state.project.client.get(clip.inner) else {
            pierro::margin(ui, pierro::Margin::same(10.0), |ui| {
                pierro::label(ui, "Clip loading...");
            });
            return;
        };

        pierro::key_value_layout(ui, |builder| {
            builder.labeled("Export Path:", |ui| {
                pierro::text_edit(ui, &mut self.export_path);
                if pierro::icon_button(ui, pierro::icons::FOLDER).mouse_clicked() {
                    if let Some(path) = rfd::FileDialog::new().save_file() {
                        self.export_path = path.with_extension("mp4").to_string_lossy().to_string();
                    }
                }
            });
            builder.labeled("Scale:", |ui| {
                pierro::DragValue::new(&mut self.scale)
                    .with_min(0.01)
                    .with_max(10.0)
                    .render(ui);
            });
        });

        pierro::v_spacing(ui, 5.0);
        pierro::vertical_centered(ui, |ui| {
            if pierro::button(ui, "Export").mouse_clicked() {
                state.editor.open_window(
                    ExportProgressModal::new(
                        self.export_path.clone().into(),
                        clip.inner,
                        clip_inner.width,
                        clip_inner.height,
                        clip_inner.framerate,
                        ui.wgpu_device()
                    )
                );
                *close = true;
            }
        });
    }

}
