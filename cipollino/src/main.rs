
pub mod app;
pub mod editor;
pub mod util;
pub mod panels;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_maximized(true)
            .with_min_inner_size([600.0, 400.0])
            .with_title("Cipollino")
            .with_icon(eframe::icon_data::from_png_bytes(&include_bytes!("../res/icon256x256.png")[..]).expect("failed to load icon")), 
        ..Default::default()
    };

    eframe::run_native(
        "Cipollino",
        native_options,
        Box::new(|cc| Box::new(app::App::new(cc))))
}

#[cfg(target_arch = "wasm32")]
fn main() {
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "canvas",
                web_options,
                Box::new(|cc| Box::new(app::App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
