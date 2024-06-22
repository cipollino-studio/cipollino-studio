
pub fn centered_fixed_window(title: &'static str) -> egui::Window {
    egui::Window::new(title)
        .title_bar(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .collapsible(false)
        .movable(false)
        .resizable(false)
}
