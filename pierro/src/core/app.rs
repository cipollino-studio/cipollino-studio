
use std::collections::HashMap;

use winit::{
    application::ApplicationHandler, dpi::{LogicalPosition, LogicalSize, Position, Size}, event::*, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, window::{Cursor, WindowId}
};

use crate::{vec2, Input, Memory, Painter, RawInput, Rect, RenderResources, UITree, Vec2, WindowConfig, UI};

use super::{CursorIcon, Key, LayoutMemory, LogicalKey, TextRenderCache, Texture};

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

fn winit_to_pierro_key(key: winit::keyboard::Key) -> Option<Key> {

    macro_rules! handle_logical_key {
        ($key: ident) => {
            if key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::$key) {
                return Some(Key {
                    text: None,
                    logical_key: Some(LogicalKey::$key)
                });
            }   
        };
    }

    handle_logical_key!(Alt);
    handle_logical_key!(CapsLock);
    handle_logical_key!(Control);
    handle_logical_key!(Fn);
    handle_logical_key!(Shift);
    if key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Super) {
        return Some(Key {
            text: None,
            logical_key: Some(LogicalKey::Command)
        });
    }
    handle_logical_key!(Enter);
    handle_logical_key!(Tab);
    handle_logical_key!(Space);
    handle_logical_key!(ArrowDown);
    handle_logical_key!(ArrowLeft);
    handle_logical_key!(ArrowRight);
    handle_logical_key!(ArrowUp);
    handle_logical_key!(Backspace);
    handle_logical_key!(Delete);
    handle_logical_key!(Escape);
    handle_logical_key!(Home);
    handle_logical_key!(End);
    handle_logical_key!(F1);
    handle_logical_key!(F2);
    handle_logical_key!(F3);
    handle_logical_key!(F4);
    handle_logical_key!(F5);
    handle_logical_key!(F6);
    handle_logical_key!(F7);
    handle_logical_key!(F8);
    handle_logical_key!(F9);
    handle_logical_key!(F10);
    handle_logical_key!(F11);
    handle_logical_key!(F12);

    if let winit::keyboard::Key::Character(text) = key {
        return Some(Key {
            text: Some(text.into()),
            logical_key: None,
        });
    }

    None
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
        match event {
            DeviceEvent::TabletPressure(pressure) => {
                self.raw_input.pressure = pressure;
            },
            _ => {}
        }
    }

    fn window_event(
        &mut self,
        event_loop: &dyn ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(render_resources) = &mut self.render_resources else { return; };
        if event != WindowEvent::RedrawRequested {
            self.redraw_counter = 2;
            render_resources.request_redraw();
        }
        match event {
            WindowEvent::SurfaceResized(new_size) => {
                render_resources.resize(new_size);
            },
            WindowEvent::RedrawRequested => {
                let delta_time = self.prev_redraw_time.elapsed().as_secs_f32();
                self.prev_redraw_time = std::time::Instant::now();
                self.raw_input.delta_time = delta_time;
                Self::tick(&mut self.app, render_resources, self.clipboard.as_mut(), &mut self.textures, &mut self.raw_input, &mut self.input, &mut self.memory);
                if self.redraw_counter > 0 {
                    self.redraw_counter -= 1;
                    render_resources.request_redraw();
                }
            },

            WindowEvent::PointerButton { device_id: _, state, button, .. } => {
                match button {
                    ButtonSource::Mouse(MouseButton::Left) => {
                        self.raw_input.l_mouse_down = state.is_pressed();
                    },
                    ButtonSource::Mouse(MouseButton::Right) => {
                        self.raw_input.r_mouse_down = state.is_pressed();
                    },
                    _ => {}
                }
            },
            WindowEvent::PointerLeft { .. } => {
                self.raw_input.mouse_pos = None;
            },
            WindowEvent::PointerMoved { device_id: _, position, .. } => {
                self.raw_input.mouse_pos = Some(vec2(position.x as f32, position.y as f32))
            },
            WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        self.raw_input.scroll += vec2(x, y) * 5.0;
                    },
                    MouseScrollDelta::PixelDelta(physical_position) => {
                        self.raw_input.scroll += vec2(physical_position.x as f32, physical_position.y as f32);
                    },
                }
            },

            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                if let Some(key) = winit_to_pierro_key(event.logical_key) {
                    if event.state.is_pressed() {
                        self.raw_input.keys_pressed.push(key);
                    } else {
                        self.raw_input.keys_released.push(key);
                    }
                }
            },

            WindowEvent::Focused(focused) => {
                if !focused {
                    self.raw_input.lost_focus = true;
                } 
            }

            WindowEvent::Ime(Ime::Preedit(preedit, _)) => {
                self.raw_input.ime_preedit = preedit;
            },
            WindowEvent::Ime(Ime::Commit(text)) => {
                self.raw_input.ime_commit = Some(text);
            },

            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => {} 
        }
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
