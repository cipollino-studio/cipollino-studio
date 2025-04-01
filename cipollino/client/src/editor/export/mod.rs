
use progress_modal::ExportProgressModal;

use crate::PanelContext;

use super::Window;

mod progress_modal;

mod video_writer;
use video_writer::*;

pub(super) struct ExportDialog {
    export_path: String,
    scale: f32,
    msaa: u32,
}

impl ExportDialog {

    pub fn new() -> Self {
        Self {
            export_path: String::new(),
            scale: 1.0,
            msaa: 2
        }
    }

}

impl Window for ExportDialog {

    fn title(&self) -> String {
        "Export".to_owned()
    }

    fn render<'ctx>(&mut self, ui: &mut pierro::UI, close: &mut bool, ctx: &mut PanelContext<'ctx>) {
        let Some(clip) = ctx.project.client.get(ctx.editor.open_clip) else {
            pierro::margin(ui, pierro::Margin::same(10.0), |ui| {
                pierro::label(ui, "No clip open.");
            });
            return;
        };
        let Some(clip_inner) = ctx.project.client.get(clip.inner) else {
            pierro::margin(ui, pierro::Margin::same(10.0), |ui| {
                pierro::label(ui, "Clip loading...");
            });
            return;
        };

        let output_w = ((clip_inner.width as f32) * self.scale).round() as u32;
        let output_h = ((clip_inner.height as f32) * self.scale).round() as u32;

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
            builder.labeled("Output Resolution:", |ui| {
                pierro::label(ui, format!("{} x {}", output_w, output_h));
            });
            builder.labeled("Anti Aliasing:", |ui| {
                pierro::dropdown(ui, format!("x{}", self.msaa * self.msaa), |ui| {
                    if pierro::menu_button(ui, "x1").mouse_clicked() {
                        self.msaa = 1;
                    }
                    if pierro::menu_button(ui, "x4").mouse_clicked() {
                        self.msaa = 2;
                    }
                    if pierro::menu_button(ui, "x16").mouse_clicked() {
                        self.msaa = 4;
                    }
                });
            });
        });

        pierro::v_spacing(ui, 5.0);
        pierro::vertical_centered(ui, |ui| {
            if pierro::button(ui, "Export").mouse_clicked() {
                ctx.editor.open_window(
                    ExportProgressModal::new(
                        self.export_path.clone().into(),
                        clip.inner,
                        output_w,
                        output_h,
                        self.msaa,
                        clip_inner.framerate,
                        ui.wgpu_device()
                    )
                );
                *close = true;
            }
        });
    }

    fn unique(&self) -> bool {
        true
    }

}
