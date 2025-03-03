
use crate::{Color, LayoutInfo, Response, Size, Texture, TextureMapMode, UINodeParams, UI};

struct CanvasMemory {
    texture: Texture
}

pub fn canvas<F: FnOnce(&mut UI, &Texture, &Response)>(ui: &mut UI, render: F) -> Response {
    let response = ui.node(
        UINodeParams::new(Size::fr(1.0), Size::fr(1.0))
            .with_fill(Color::WHITE)
            .with_texture_map(TextureMapMode::Cover)
    );

    let size = ui.memory().get::<LayoutInfo>(response.id).screen_rect.size();
    let width = size.x.ceil() as u32 + 100;
    let height = size.y.ceil() as u32 + 100;

    let create_texture = if let Some(memory) = ui.memory().get_opt::<CanvasMemory>(response.id) {
        memory.texture.width() != width || memory.texture.height() != height
    } else {
        true
    };

    if create_texture {
        let texture = Texture::create_empty(ui.wgpu_device(), width, height);
        ui.memory().insert(response.id, CanvasMemory {
            texture,
        });
    }

    if let Some(memory) = ui.memory().get_opt::<CanvasMemory>(response.id) {
        let texture = memory.texture.clone();
        ui.with_parent(response.node_ref, |ui| {
            render(ui, &texture, &response);
        });
        ui.set_texture(response.node_ref, texture);
    }

    response
}
