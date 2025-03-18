
use std::collections::HashMap;

use winit::{
    application::ApplicationHandler, dpi::{LogicalPosition, LogicalSize, Position, Size}, event::*, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::{Cursor, WindowId}
};

use crate::{vec2, Input, Memory, Painter, RawInput, Rect, RenderResources, UITree, Vec2, WindowConfig, UI};

use super::{CursorIcon, LayoutMemory, TextRenderCache, Texture};

mod input;

pub trait App {

    fn window_config() -> WindowConfig {
        WindowConfig::default()
    }

    fn tick(&mut self, ui: &mut UI);

}

struct AppHandler<'a, T: App> {
    app: T,

    render_resources: Option<RenderResources<'a>>,
    clipboard: Option<arboard::Clipboard>,
    textures: HashMap<String, Texture>,
    raw_input: RawInput,
    input: Input,
    memory: Memory,

    prev_redraw_time: std::time::Instant,
    redraw_counter: i32
}

impl<T: App> AppHandler<'_, T> {

    pub fn tick(app: &mut T, render_resources: &mut RenderResources<'_>, clipboard: Option<&mut arboard::Clipboard>, textures: &mut HashMap<String, Texture>, raw_input: &mut RawInput, input: &mut Input, memory: &mut Memory) {
        let physical_size = vec2(render_resources.window.surface_size().width as f32, render_resources.window.surface_size().height as f32);
        let scale_factor = render_resources.window.scale_factor() as f32;
        let size = physical_size / scale_factor;
        
        let mut tree = UITree::new();
        let layer = tree.add_layer(size); 

        // distribute input
        input.update(raw_input, scale_factor);
        input.distribute(memory);

        // ui generation
        let mut ui = UI::new(input, memory, render_resources, clipboard, textures, size, tree, layer);
        app.tick(&mut ui);

        let cursor = ui.cursor;
        let request_redraw = ui.request_redraw; 
        let request_ime = ui.request_ime;

        let mut tree = ui.tree();
        if request_redraw {
            render_resources.request_redraw();
        }

        memory.garbage_collect(&tree);

        // ui layout
        tree.layout(Rect::min_size(Vec2::ZERO, size), memory, &mut render_resources.text_resources);
        tree.remember_layout(memory);

        // ui rendering
        let Ok(output) = render_resources.surface.get_current_texture() else { return; }; 
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        render_resources.begin_frame(size);

        let mut encoder = render_resources.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("pierro_command_encoder"),
        });

        let mut next_text_render_cache = TextRenderCache::new();
        let mut painter = Painter::new(
            &render_resources.device,
            &render_resources.queue,
            &mut encoder,
            &view,
            
            &mut render_resources.paint_resources,
            &mut render_resources.text_resources, 

            size,
            scale_factor,

            &mut render_resources.text_render_cache,
            &mut next_text_render_cache
        );

        tree.paint(&mut painter);

        painter.finish();
        render_resources.text_render_cache = next_text_render_cache;

        render_resources.queue.submit([encoder.finish()]);
        output.present();

        // other ui output
        render_resources.window.set_cursor(Cursor::Icon(pierro_to_winit_cursor(cursor)));
        render_resources.window.set_ime_allowed(request_ime.is_some()); 
        if let Some(ime_node) = request_ime {
            let id = tree.get(ime_node).id;
            let rect = memory.get::<LayoutMemory>(id).screen_rect;
            let logical_position = Position::Logical(LogicalPosition::new(rect.left() as f64, rect.top() as f64));
            let logical_size = Size::Logical(LogicalSize::new(rect.width() as f64, rect.height() as f64));
            render_resources.window.set_ime_cursor_area(logical_position, logical_size);
        }

    }

}



fn pierro_to_winit_cursor(cursor: CursorIcon) -> winit::window::CursorIcon {
    match cursor {
        CursorIcon::Default => winit::window::CursorIcon::Default,
        CursorIcon::Crosshair => winit::window::CursorIcon::Crosshair,
        CursorIcon::Move => winit::window::CursorIcon::Move,
        CursorIcon::Text => winit::window::CursorIcon::Text,
        CursorIcon::Wait => winit::window::CursorIcon::Wait,
        CursorIcon::Help => winit::window::CursorIcon::Help,
        CursorIcon::Progress => winit::window::CursorIcon::Progress,
        CursorIcon::NotAllowed => winit::window::CursorIcon::NotAllowed,
        CursorIcon::ContextMenu => winit::window::CursorIcon::ContextMenu,
        CursorIcon::Cell => winit::window::CursorIcon::Cell,
        CursorIcon::VerticalText => winit::window::CursorIcon::VerticalText,
        CursorIcon::Alias => winit::window::CursorIcon::Alias,
        CursorIcon::Copy => winit::window::CursorIcon::Copy,
        CursorIcon::NoDrop => winit::window::CursorIcon::NoDrop,
        CursorIcon::Grab => winit::window::CursorIcon::Grab,
        CursorIcon::Grabbing => winit::window::CursorIcon::Grabbing,
        CursorIcon::AllScroll => winit::window::CursorIcon::AllScroll,
        CursorIcon::ZoomIn => winit::window::CursorIcon::ZoomIn,
        CursorIcon::ZoomOut => winit::window::CursorIcon::ZoomOut,
        CursorIcon::EResize => winit::window::CursorIcon::EResize,
        CursorIcon::NResize => winit::window::CursorIcon::NResize,
        CursorIcon::NeResize => winit::window::CursorIcon::NeResize,
        CursorIcon::NwResize => winit::window::CursorIcon::NwResize,
        CursorIcon::SResize => winit::window::CursorIcon::SResize,
        CursorIcon::SeResize => winit::window::CursorIcon::SeResize,
        CursorIcon::SwResize => winit::window::CursorIcon::SwResize,
        CursorIcon::WResize => winit::window::CursorIcon::WResize,
        CursorIcon::EwResize => winit::window::CursorIcon::EwResize,
        CursorIcon::NsResize => winit::window::CursorIcon::NsResize,
        CursorIcon::NeswResize => winit::window::CursorIcon::NeswResize,
        CursorIcon::NwseResize => winit::window::CursorIcon::NwseResize,
        CursorIcon::ColResize => winit::window::CursorIcon::ColResize,
        CursorIcon::RowResize => winit::window::CursorIcon::RowResize,
    }
}

impl<T: App> ApplicationHandler for AppHandler<'_, T> {

    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {
        self.render_resources = pollster::block_on(RenderResources::new(event_loop, T::window_config()));
    }

    fn device_event(
        &mut self,
        _event_loop: &dyn ActiveEventLoop,
        _device_id: Option<DeviceId>,
        event: DeviceEvent,
    ) {
        self.handle_device_event(event);
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.handle_window_event(event_loop, event);        
    }

}

pub fn run<T: App>(app: T) {

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    event_loop.run_app(&mut AppHandler {
        app,
        render_resources: None,
        clipboard: arboard::Clipboard::new().ok(),
        textures: HashMap::new(),
        raw_input: RawInput::new(),
        input: Input::new(),
        memory: Memory::new(),
        prev_redraw_time: std::time::Instant::now(),
        redraw_counter: 0
    }).unwrap();

}
