
use super::UI;

impl UI<'_, '_> {

    pub fn get_clipboard_text(&mut self) -> Option<String> {
        self.clipboard.as_mut().map(|clipboard| clipboard.get_text().ok()).flatten()
    }

    pub fn set_clipboard_text(&mut self, text: String) {
        if let Some(clipboard) = &mut self.clipboard {
            let _ = clipboard.set_text(text);
        }
    }

}
