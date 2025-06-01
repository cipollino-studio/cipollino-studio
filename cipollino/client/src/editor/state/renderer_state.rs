
use crate::BuiltinBrushTextures;

pub struct RendererState {
    pub renderer: malvina::Renderer,
    pub builtin_brushes: BuiltinBrushTextures 
}

impl RendererState {

    pub fn new(device: &pierro::wgpu::Device, queue: &pierro::wgpu::Queue) -> Self {
        let mut renderer = malvina::Renderer::new(device, queue);
        let builtin_brushes = BuiltinBrushTextures::load(device, queue, &mut renderer);
        Self {
            renderer,
            builtin_brushes
        }
    }

}
