
use project::{Client, SceneObjectColor};

pub fn get_color_value(color: &SceneObjectColor, client: &Client) -> elic::Color {
    match client.get(color.color.ptr()) {
        Some(color) => color.color.into(),
        None => color.backup.into()
    }
}
