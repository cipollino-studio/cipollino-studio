
use crate::{Color, Response, Size, Texture, UINodeParams, UI};

pub fn sized_image(ui: &mut UI, width: f32, height: f32, texture: Texture) -> Response {
    ui.node(
        UINodeParams::new(Size::px(width), Size::px(height))
            .with_fill(Color::WHITE)
            .with_texture(texture)
    )
}

pub fn scaled_image(ui: &mut UI, scale: f32, texture: Texture) -> Response {
    sized_image(ui, scale * (texture.width() as f32), scale * (texture.height() as f32), texture) 
}

pub fn image(ui: &mut UI, texture: Texture) -> Response {
    scaled_image(ui, 1.0, texture)
}

pub fn image_with_width(ui: &mut UI, width: f32, texture: Texture) -> Response {
    scaled_image(ui, width / (texture.width() as f32), texture)
}
