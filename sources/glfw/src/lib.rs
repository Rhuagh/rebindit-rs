#[macro_use]
extern crate log;

extern crate input;
extern crate glfw;

use glfw::Glfw;
use input::raw::{RawInputSource, RawInput, RawInputEvent, RawInputAction, RawInputModifiers};
use input::types::{DeviceType, WindowPosition};

use std::sync::mpsc::Receiver;

pub struct GlfwInputSource {
    glfw : Glfw,
    events : Receiver<(f64, glfw::WindowEvent)>,
    last_cursor_position : Option<WindowPosition>,
    current_size : (f64, f64),
}

impl GlfwInputSource {
    pub fn new(glfw: Glfw,
               events: Receiver<(f64, glfw::WindowEvent)>,
               current_size : (f64, f64)) -> GlfwInputSource {
        GlfwInputSource {
            glfw: glfw,
            events: events,
            last_cursor_position : None,
            current_size: current_size
        }
    }
}

impl RawInputSource for GlfwInputSource {

    fn process(&mut self) -> Vec<RawInput> {
        self.glfw.poll_events();
        let mut raw = vec![];
        for (time, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(keycode, _, action, modifiers) => {
                    raw.push(RawInput::new(time, DeviceType::Keyboard, 0,
                                           RawInputEvent::Key(map_keycode(keycode),
                                                              map_action(action),
                                                              map_modifiers(modifiers))));
                },
                glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                    raw.push(RawInput::new(time, DeviceType::Mouse, 0,
                                           RawInputEvent::Button(map_mouse_button(button),
                                                                 match self.last_cursor_position {
                                                                     Some(position) => position,
                                                                     None => (0.0, 0.0)
                                                                 },
                                                                 map_action(action),
                                                                 map_modifiers(modifiers))));
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    raw.push(RawInput::new(time, DeviceType::Window, 0,
                                           RawInputEvent::Scroll(x/self.current_size.0,
                                                                 y/self.current_size.1)));
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    raw.push(RawInput::new(time, DeviceType::Mouse, 0,
                                           RawInputEvent::CursorPosition(x/self.current_size.0,
                                                                         y/self.current_size.1)));
                    match self.last_cursor_position {
                        Some((px, py)) => raw.push(RawInput::new(time, DeviceType::Mouse, 0,
                            RawInputEvent::Motion((x-px)/self.current_size.0,
                                                (y-py)/self.current_size.1))),
                        None => ()
                    };
                    self.last_cursor_position = Some((x, y));
                },
                glfw::WindowEvent::Close => {
                    raw.push(RawInput::new(time, DeviceType::Window, 0,
                                           RawInputEvent::Close));
                },
                glfw::WindowEvent::Focus(b) => {
                    raw.push(RawInput::new(time, DeviceType::Window, 0,
                                           RawInputEvent::Focus(b)));
                },
                glfw::WindowEvent::Char(ch) => {
                    raw.push(RawInput::new(time, DeviceType::Keyboard, 0,
                                           RawInputEvent::Char(ch)));
                },
                glfw::WindowEvent::Size(x, y) => {
                    raw.push(RawInput::new(time, DeviceType::Window, 0,
                                           RawInputEvent::Resize(x as u32, y as u32)));
                    self.current_size = (x as f64, y as f64);
                }
                _ => {
                    debug!("{:?}", event);
                }
            }
        }
        raw
    }
}

fn map_mouse_button(button : glfw::MouseButton) -> u32 {
    match button {
        glfw::MouseButton::Button1 => 1,
        glfw::MouseButton::Button2 => 2,
        glfw::MouseButton::Button3 => 3,
        glfw::MouseButton::Button4 => 4,
        glfw::MouseButton::Button5 => 5,
        glfw::MouseButton::Button6 => 6,
        glfw::MouseButton::Button7 => 7,
        glfw::MouseButton::Button8 => 8
    }
}

fn map_modifiers(modifiers : glfw::modifiers::Modifiers) -> RawInputModifiers {
    RawInputModifiers::from_bits_truncate(modifiers.bits() as u32)
}

fn map_action(action: glfw::Action) -> RawInputAction {
    match action {
        glfw::Action::Press => RawInputAction::Press,
        glfw::Action::Release => RawInputAction::Release,
        glfw::Action::Repeat => RawInputAction::Repeat
    }
}

fn map_keycode(keycode: glfw::Key) -> input::types::KeyCode {
    match keycode {
        glfw::Key::Space => input::types::KeyCode::Space,
        glfw::Key::Apostrophe => input::types::KeyCode::Apostrophe,
        glfw::Key::Comma => input::types::KeyCode::Comma,
        glfw::Key::Minus => input::types::KeyCode::Minus,
        glfw::Key::Period => input::types::KeyCode::Period,
        glfw::Key::Slash => input::types::KeyCode::Slash,
        glfw::Key::Num0 => input::types::KeyCode::Key0,
        glfw::Key::Num1 => input::types::KeyCode::Key1,
        glfw::Key::Num2 => input::types::KeyCode::Key2,
        glfw::Key::Num3 => input::types::KeyCode::Key3,
        glfw::Key::Num4 => input::types::KeyCode::Key4,
        glfw::Key::Num5 => input::types::KeyCode::Key5,
        glfw::Key::Num6 => input::types::KeyCode::Key6,
        glfw::Key::Num7 => input::types::KeyCode::Key7,
        glfw::Key::Num8 => input::types::KeyCode::Key8,
        glfw::Key::Num9 => input::types::KeyCode::Key9,
        glfw::Key::Semicolon => input::types::KeyCode::Semicolon,
        glfw::Key::Equal => input::types::KeyCode::Equals,
        glfw::Key::A => input::types::KeyCode::A,
        glfw::Key::B => input::types::KeyCode::B,
        glfw::Key::C => input::types::KeyCode::C,
        glfw::Key::D => input::types::KeyCode::D,
        glfw::Key::E => input::types::KeyCode::E,
        glfw::Key::F => input::types::KeyCode::F,
        glfw::Key::G => input::types::KeyCode::G,
        glfw::Key::H => input::types::KeyCode::H,
        glfw::Key::I => input::types::KeyCode::I,
        glfw::Key::J => input::types::KeyCode::J,
        glfw::Key::K => input::types::KeyCode::K,
        glfw::Key::L => input::types::KeyCode::L,
        glfw::Key::M => input::types::KeyCode::M,
        glfw::Key::N => input::types::KeyCode::N,
        glfw::Key::O => input::types::KeyCode::O,
        glfw::Key::P => input::types::KeyCode::P,
        glfw::Key::Q => input::types::KeyCode::Q,
        glfw::Key::R => input::types::KeyCode::R,
        glfw::Key::S => input::types::KeyCode::S,
        glfw::Key::T => input::types::KeyCode::T,
        glfw::Key::U => input::types::KeyCode::U,
        glfw::Key::V => input::types::KeyCode::V,
        glfw::Key::W => input::types::KeyCode::W,
        glfw::Key::X => input::types::KeyCode::X,
        glfw::Key::Y => input::types::KeyCode::Y,
        glfw::Key::Z => input::types::KeyCode::Z,
        glfw::Key::LeftBracket => input::types::KeyCode::LBracket,
        glfw::Key::Backslash => input::types::KeyCode::Backslash,
        glfw::Key::RightBracket => input::types::KeyCode::RBracket,
        glfw::Key::GraveAccent => input::types::KeyCode::Grave,
        glfw::Key::World1 => input::types::KeyCode::None,
        glfw::Key::World2 => input::types::KeyCode::None,
        glfw::Key::Escape => input::types::KeyCode::Escape,
        glfw::Key::Enter => input::types::KeyCode::Return,
        glfw::Key::Tab => input::types::KeyCode::Tab,
        glfw::Key::Backspace => input::types::KeyCode::Back,
        glfw::Key::Insert => input::types::KeyCode::Insert,
        glfw::Key::Delete => input::types::KeyCode::Delete,
        glfw::Key::Right => input::types::KeyCode::Right,
        glfw::Key::Left => input::types::KeyCode::Left,
        glfw::Key::Down => input::types::KeyCode::Down,
        glfw::Key::Up => input::types::KeyCode::Up,
        glfw::Key::PageUp => input::types::KeyCode::PageUp,
        glfw::Key::PageDown => input::types::KeyCode::PageDown,
        glfw::Key::Home => input::types::KeyCode::Home,
        glfw::Key::End => input::types::KeyCode::End,
        glfw::Key::CapsLock => input::types::KeyCode::Capital,
        glfw::Key::ScrollLock => input::types::KeyCode::Scroll,
        glfw::Key::NumLock => input::types::KeyCode::Numlock,
        glfw::Key::PrintScreen => input::types::KeyCode::Snapshot,
        glfw::Key::Pause => input::types::KeyCode::Pause,
        glfw::Key::F1 => input::types::KeyCode::F1,
        glfw::Key::F2 => input::types::KeyCode::F2,
        glfw::Key::F3 => input::types::KeyCode::F3,
        glfw::Key::F4 => input::types::KeyCode::F4,
        glfw::Key::F5 => input::types::KeyCode::F5,
        glfw::Key::F6 => input::types::KeyCode::F6,
        glfw::Key::F7 => input::types::KeyCode::F7,
        glfw::Key::F8 => input::types::KeyCode::F8,
        glfw::Key::F9 => input::types::KeyCode::F9,
        glfw::Key::F10 => input::types::KeyCode::F10,
        glfw::Key::F11 => input::types::KeyCode::F11,
        glfw::Key::F12 => input::types::KeyCode::F12,
        glfw::Key::F13 => input::types::KeyCode::F13,
        glfw::Key::F14 => input::types::KeyCode::F14,
        glfw::Key::F15 => input::types::KeyCode::F15,
        glfw::Key::F16 => input::types::KeyCode::None,
        glfw::Key::F17 => input::types::KeyCode::None,
        glfw::Key::F18 => input::types::KeyCode::None,
        glfw::Key::F19 => input::types::KeyCode::None,
        glfw::Key::F20 => input::types::KeyCode::None,
        glfw::Key::F21 => input::types::KeyCode::None,
        glfw::Key::F22 => input::types::KeyCode::None,
        glfw::Key::F23 => input::types::KeyCode::None,
        glfw::Key::F24 => input::types::KeyCode::None,
        glfw::Key::F25 => input::types::KeyCode::None,
        glfw::Key::Kp0 => input::types::KeyCode::Numpad0,
        glfw::Key::Kp1 => input::types::KeyCode::Numpad1,
        glfw::Key::Kp2 => input::types::KeyCode::Numpad2,
        glfw::Key::Kp3 => input::types::KeyCode::Numpad3,
        glfw::Key::Kp4 => input::types::KeyCode::Numpad4,
        glfw::Key::Kp5 => input::types::KeyCode::Numpad5,
        glfw::Key::Kp6 => input::types::KeyCode::Numpad6,
        glfw::Key::Kp7 => input::types::KeyCode::Numpad7,
        glfw::Key::Kp8 => input::types::KeyCode::Numpad8,
        glfw::Key::Kp9 => input::types::KeyCode::Numpad9,
        glfw::Key::KpDecimal => input::types::KeyCode::Decimal,
        glfw::Key::KpDivide => input::types::KeyCode::Divide,
        glfw::Key::KpMultiply => input::types::KeyCode::Multiply,
        glfw::Key::KpSubtract => input::types::KeyCode::Subtract,
        glfw::Key::KpAdd => input::types::KeyCode::Add,
        glfw::Key::KpEnter => input::types::KeyCode::NumpadEnter,
        glfw::Key::KpEqual => input::types::KeyCode::NumpadEquals,
        glfw::Key::LeftShift => input::types::KeyCode::LShift,
        glfw::Key::LeftControl => input::types::KeyCode::LControl,
        glfw::Key::LeftAlt => input::types::KeyCode::LAlt,
        glfw::Key::LeftSuper => input::types::KeyCode::LWin,
        glfw::Key::RightShift => input::types::KeyCode::RShift,
        glfw::Key::RightControl => input::types::KeyCode::RControl,
        glfw::Key::RightAlt => input::types::KeyCode::RAlt,
        glfw::Key::RightSuper => input::types::KeyCode::RWin,
        glfw::Key::Menu => input::types::KeyCode::LMenu,
        glfw::Key::Unknown => input::types::KeyCode::None,
    }
}
