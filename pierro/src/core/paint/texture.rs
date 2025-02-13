
use std::sync::Arc;

#[derive(Clone)]
pub struct Texture {
    tex: Arc<(wgpu::Texture, wgpu::TextureView)> 
}

impl PartialEq for Texture {

    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.tex, &other.tex)
    }

}

impl Eq for Texture {}

impl Texture {

    pub fn new(texture: wgpu::Texture, texture_view: wgpu::TextureView) -> Self {
        Self {
            tex: Arc::new((texture, texture_view)),
        }
    }

    pub fn texture(&self) -> &wgpu::Texture {
        &self.tex.0
    }
    
    pub fn texture_view(&self) -> &wgpu::TextureView {
        &self.tex.1
    }

}
