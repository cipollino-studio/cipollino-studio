
use std::sync::Arc;

use winit::{
    dpi::{LogicalSize, PhysicalSize, Size}, event_loop::ActiveEventLoop, window::{Icon, Window, WindowAttributes}
};

use crate::{text::TextResources, PainterResources, WindowConfig};

use super::{TextRenderCache, Vec2};

pub(crate) struct RenderResources<'a> {
    pub(crate) window: Arc<Window>,

    pub(crate) surface: wgpu::Surface<'a>,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) config: wgpu::SurfaceConfiguration,
    pub(crate) physical_size: PhysicalSize<u32>,

    pub(crate) text_resources: TextResources,
    pub(crate) paint_resources: PainterResources,

    pub(crate) text_render_cache: TextRenderCache
}

impl RenderResources<'_> {

    pub(crate) async fn new(event_loop: &ActiveEventLoop, config: WindowConfig) -> Option<Self> {

        let icon = Icon::from_rgba(config.icon.rgba, config.icon.width, config.icon.height).ok();

        let window_attributes = WindowAttributes::default()
            .with_min_inner_size(Size::Logical(LogicalSize::new(config.min_size.x as f64, config.min_size.y as f64)))
            .with_window_icon(icon)
            .with_title(config.title);
        let window = Arc::new(event_loop.create_window(window_attributes).ok()?);
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).ok()?;
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface) 
        }).await?;
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
            },
            None
        ).await.ok()?;
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|format| !format.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: if surface_caps.present_modes.contains(&wgpu::PresentMode::Fifo) {
                wgpu::PresentMode::Fifo
            } else {
                surface_caps.present_modes[0] 
            },
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let paint_resources = PainterResources::new(&device, config.format);

        let text_resources = TextResources::new(&device);
        let text_render_cache = TextRenderCache::new();

        Some(Self {
            window,
            surface,
            device,
            queue,
            config,
            physical_size: size,
            paint_resources,
            text_resources,
            text_render_cache
        })
    }

    pub(crate) fn begin_frame(&mut self, screen_size: Vec2) {
        self.paint_resources.begin_frame(&self.queue, screen_size);
    }

    pub(crate) fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.physical_size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub(crate) fn request_redraw(&self) {
        self.window.request_redraw();
    }

}
