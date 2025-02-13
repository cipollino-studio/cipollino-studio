
mod editor;
pub use editor::*;

mod panels;
pub use panels::*;

struct App {
    editor: Editor
}

impl pierro::App for App {

    fn tick(&mut self, ui: &mut pierro::UI) {
        self.editor.tick(ui);
    }

}

fn main() {
    pierro::run(App {
        editor: Editor::new(project::Client::local("test.cip").unwrap()),
    });
}
