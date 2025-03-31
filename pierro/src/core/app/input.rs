
use crate::{vec2, Key, KeyModifiers};

use super::{App, AppHandler};

fn winit_to_pierro_key(key: winit::keyboard::Key) -> Option<Key> {

    macro_rules! handle_named_key {
        ($key: ident) => {
            if key == winit::keyboard::Key::Named(winit::keyboard::NamedKey::$key) {
                return Some(Key::$key);
            } 
        };
    }

    handle_named_key!(ArrowDown);
    handle_named_key!(ArrowLeft);
    handle_named_key!(ArrowRight);
    handle_named_key!(ArrowUp);

    handle_named_key!(Escape);
    handle_named_key!(Tab);
    handle_named_key!(Backspace);
    handle_named_key!(Enter);
    handle_named_key!(Space);

    handle_named_key!(Insert);
    handle_named_key!(Delete);
    handle_named_key!(Home);
    handle_named_key!(End);
    handle_named_key!(PageUp);
    handle_named_key!(PageDown);

    handle_named_key!(F1);
    handle_named_key!(F2);
    handle_named_key!(F3);
    handle_named_key!(F4);
    handle_named_key!(F5);
    handle_named_key!(F6);
    handle_named_key!(F7);
    handle_named_key!(F8);
    handle_named_key!(F9);
    handle_named_key!(F10);
    handle_named_key!(F11);
    handle_named_key!(F12);
    handle_named_key!(F13);
    handle_named_key!(F14);
    handle_named_key!(F15);
    handle_named_key!(F16);
    handle_named_key!(F17);
    handle_named_key!(F18);
    handle_named_key!(F19);
    handle_named_key!(F20);
    handle_named_key!(F21);
    handle_named_key!(F22);
    handle_named_key!(F23);
    handle_named_key!(F24);
    handle_named_key!(F25);
    handle_named_key!(F26);
    handle_named_key!(F27);
    handle_named_key!(F28);
    handle_named_key!(F29);
    handle_named_key!(F30);
    handle_named_key!(F31);
    handle_named_key!(F32);
    handle_named_key!(F33);
    handle_named_key!(F34);
    handle_named_key!(F35);

    macro_rules! handle_str_key {
        ($key: ident, $str: literal) => {
            if key == winit::keyboard::Key::Character($str.into()) {
                return Some(Key::$key);
            }
        };
    }

    handle_str_key!(Colon, ":");
    handle_str_key!(Comma, ",");
    handle_str_key!(Backslash, "\\");
    handle_str_key!(Slash, "/");
    handle_str_key!(Pipe, "|");
    handle_str_key!(QuestionMark, "?");
    handle_str_key!(OpenBracket, "(");
    handle_str_key!(CloseBracket, ")");
    handle_str_key!(Backtick, "`");
    handle_str_key!(Minus, "-");
    handle_str_key!(Period, ".");
    handle_str_key!(Plus, "+");
    handle_str_key!(Equals, "=");
    handle_str_key!(Semicolon, ";");
    handle_str_key!(Quote, "'");

    handle_str_key!(Num0, "0");
    handle_str_key!(Num1, "1");
    handle_str_key!(Num2, "2");
    handle_str_key!(Num3, "3");
    handle_str_key!(Num4, "4");
    handle_str_key!(Num5, "5");
    handle_str_key!(Num6, "6");
    handle_str_key!(Num7, "7");
    handle_str_key!(Num8, "8");
    handle_str_key!(Num9, "9");

    handle_str_key!(A, "A"); 
    handle_str_key!(B, "B");
    handle_str_key!(C, "C"); 
    handle_str_key!(D, "D"); 
    handle_str_key!(E, "E"); 
    handle_str_key!(F, "F"); 
    handle_str_key!(G, "G"); 
    handle_str_key!(H, "H"); 
    handle_str_key!(I, "I"); 
    handle_str_key!(J, "J"); 
    handle_str_key!(K, "K"); 
    handle_str_key!(L, "L");
    handle_str_key!(M, "M");
    handle_str_key!(N, "N");
    handle_str_key!(O, "O"); 
    handle_str_key!(P, "P"); 
    handle_str_key!(Q, "Q");
    handle_str_key!(R, "R"); 
    handle_str_key!(S, "S"); 
    handle_str_key!(T, "T"); 
    handle_str_key!(U, "U"); 
    handle_str_key!(V, "V"); 
    handle_str_key!(W, "W"); 
    handle_str_key!(X, "X"); 
    handle_str_key!(Y, "Y");
    handle_str_key!(Z, "Z"); 

    handle_str_key!(A, "a"); 
    handle_str_key!(B, "b");
    handle_str_key!(C, "c"); 
    handle_str_key!(D, "d"); 
    handle_str_key!(E, "e"); 
    handle_str_key!(F, "f"); 
    handle_str_key!(G, "g"); 
    handle_str_key!(H, "h"); 
    handle_str_key!(I, "i"); 
    handle_str_key!(J, "j"); 
    handle_str_key!(K, "k"); 
    handle_str_key!(L, "l");
    handle_str_key!(M, "m");
    handle_str_key!(N, "n");
    handle_str_key!(O, "o"); 
    handle_str_key!(P, "p"); 
    handle_str_key!(Q, "q");
    handle_str_key!(R, "r"); 
    handle_str_key!(S, "s"); 
    handle_str_key!(T, "t"); 
    handle_str_key!(U, "u"); 
    handle_str_key!(V, "v"); 
    handle_str_key!(W, "w"); 
    handle_str_key!(X, "x"); 
    handle_str_key!(Y, "y");
    handle_str_key!(Z, "z");

    None
}

impl<T: App> AppHandler<'_, T> {

    pub(super) fn handle_device_event(&mut self, event: winit::event::DeviceEvent) {
        match event {
            winit::event::DeviceEvent::TabletPressure(pressure) => {
                self.raw_input.pressure = pressure;
            },
            _ => {}
        }
    } 

    pub(super) fn handle_window_event(&mut self, event_loop: &dyn winit::event_loop::ActiveEventLoop, event: winit::event::WindowEvent) {
        let Some(render_resources) = &mut self.render_resources else { return; };
        if event != winit::event::WindowEvent::RedrawRequested {
            self.redraw_counter = 2;
            render_resources.request_redraw();
        }
        match event {
            winit::event::WindowEvent::SurfaceResized(new_size) => {
                render_resources.resize(new_size);
            },
            winit::event::WindowEvent::RedrawRequested => {
                let delta_time = self.prev_redraw_time.elapsed().as_secs_f32();
                self.prev_redraw_time = std::time::Instant::now();
                self.raw_input.delta_time = delta_time;
                Self::tick(&mut self.app, render_resources, self.clipboard.as_mut(), &mut self.textures, &mut self.raw_input, &mut self.input, &mut self.memory);
                if self.redraw_counter > 0 {
                    self.redraw_counter -= 1;
                    render_resources.request_redraw();
                }
            },

            winit::event::WindowEvent::PointerButton { device_id: _, state, button, .. } => {
                match button {
                    winit::event::ButtonSource::Mouse(winit::event::MouseButton::Left) => {
                        self.raw_input.l_mouse_down = state.is_pressed();
                    },
                    winit::event::ButtonSource::Mouse(winit::event::MouseButton::Right) => {
                        self.raw_input.r_mouse_down = state.is_pressed();
                    },
                    _ => {}
                }
            },
            winit::event::WindowEvent::PointerLeft { .. } => {
                self.raw_input.mouse_pos = None;
            },
            winit::event::WindowEvent::PointerMoved { device_id: _, position, .. } => {
                self.raw_input.mouse_pos = Some(vec2(position.x as f32, position.y as f32))
            },
            winit::event::WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
                match delta {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => {
                        self.raw_input.scroll += vec2(x, y) * 5.0;
                    },
                    winit::event::MouseScrollDelta::PixelDelta(physical_position) => {
                        self.raw_input.scroll += vec2(physical_position.x as f32, physical_position.y as f32);
                    },
                }
            },

            winit::event::WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                if let Some(key) = winit_to_pierro_key(event.logical_key.clone()) {
                    if event.state.is_pressed() {
                        self.raw_input.keys_pressed.push(key);
                    } else {
                        self.raw_input.keys_released.push(key);
                    }
                }

                if event.state.is_pressed() {
                    if let winit::keyboard::Key::Character(str) = event.logical_key {
                        self.raw_input.text += &str; 
                    }
                }
            },
            winit::event::WindowEvent::ModifiersChanged(modifiers) => {
                self.raw_input.key_modifiers = KeyModifiers::empty();
                if modifiers.state().shift_key() {
                    self.raw_input.key_modifiers |= KeyModifiers::SHIFT;
                }
                if modifiers.state().alt_key() {
                    self.raw_input.key_modifiers |= KeyModifiers::OPTION;
                }
                #[cfg(target_os = "macos")]
                if modifiers.state().super_key() {
                    self.raw_input.key_modifiers |= KeyModifiers::CONTROL;
                }
                #[cfg(not(target_os = "macos"))]
                if modifiers.state().control_key() {
                    self.raw_input.key_modifiers |= KeyModifiers::CONTROL;
                }

            },

            winit::event::WindowEvent::Focused(focused) => {
                if !focused {
                    self.raw_input.lost_focus = true;
                } 
            }
            winit::event::WindowEvent::Ime(winit::event::Ime::Preedit(preedit, _)) => {
                self.raw_input.ime_preedit = preedit;
            },
            winit::event::WindowEvent::Ime(winit::event::Ime::Commit(text)) => {
                self.raw_input.ime_commit = Some(text);
            },

            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => {} 
        }
    }

}
