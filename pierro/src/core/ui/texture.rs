
use crate::Texture;

use super::UI;

impl UI<'_, '_> {

    pub fn load_texture<F: FnOnce(&wgpu::Device, &wgpu::Queue) -> Texture>(&mut self, name: &str, loader: F) -> Texture {
        if let Some(texture) = self.textures.get(name) {
            return texture.clone();
        }
        let texture = loader(self.wgpu_device(), self.wgpu_queue()); 
        self.textures.insert(name.to_owned(), texture.clone());
        texture
    }

}

#[macro_export]
macro_rules! include_image {
    ($ui: ident, $path: literal) => {
        $ui.load_texture($path, |device, queue| {
            let bytes = include_bytes!($path);
            let image = ::pierro::image::load_from_memory(bytes).unwrap();
            let data = image.to_rgba8();
            ::pierro::Texture::create(device, queue, image.width(), image.height(), &data)
        }) 
    };
}