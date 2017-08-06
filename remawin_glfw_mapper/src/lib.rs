#[macro_use]
extern crate log;

extern crate remawin;
extern crate glfw;
extern crate serde;

use remawin::raw::{RawInput, RawInputEvent, RawInputAction, RawInputModifiers};
use remawin::types::{DeviceType, WindowData, ActionMetadata, KeyCode};
use remawin::InputReMapper;

use serde::de::DeserializeOwned;

use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::Debug;
use std::clone::Clone;
use std::default::Default;

pub struct GlfwEventMapper<ACTION, ID>
    where ACTION: Hash + Eq + Clone,
          ID: Hash + Eq + Clone + Debug {
    window_data : WindowData,
    input_remapper : InputReMapper<ACTION, ID>
}

impl <ACTION, ID> GlfwEventMapper<ACTION, ID>
    where ACTION: Hash + Eq + Clone + ActionMetadata + Debug + DeserializeOwned,
          ID: Hash + Eq + Clone + Debug + DeserializeOwned {
    pub fn new(current_size : (f64, f64)) -> GlfwEventMapper<ACTION, ID> {
        GlfwEventMapper {
            window_data : WindowData {
                size : current_size,
                cursor_position : None
            },
            input_remapper : InputReMapper::new()
        }
    }

    pub fn process_events(&mut self, events : &Vec<(f64, glfw::WindowEvent)>) -> Vec<RawInput> {
        let mut next = self.window_data.clone();
        let raw = events.iter().flat_map(|e| process_event(e, &mut next)).collect();
        self.window_data = next;
        raw
    }

    pub fn process(&mut self, events : &Vec<(f64, glfw::WindowEvent)>) -> Vec<remawin::Event<ACTION, ID>> {
        let raw_input = self.process_events(events);
        self.input_remapper.process_raw_input(&raw_input)
    }

    pub fn remapper_mut(&mut self) -> &mut InputReMapper<ACTION, ID> {
        &mut self.input_remapper
    }

    pub fn remapper(&self) -> &InputReMapper<ACTION, ID> {
        &self.input_remapper
    }

}

fn process_event(&(time, ref event) : &(f64, glfw::WindowEvent), next : &mut WindowData) -> Vec<RawInput> {
    let mut raw = Vec::default();
    match *event {
        glfw::WindowEvent::Key(keycode, _, action, modifiers) => {
            raw.push(RawInput::new(time, DeviceType::Keyboard, 0,
                                   RawInputEvent::Key(map_keycode(keycode),
                                                      map_action(action),
                                                      map_modifiers(modifiers))));
        },
        glfw::WindowEvent::MouseButton(button, action, modifiers) => {
            raw.push(RawInput::new(time, DeviceType::Mouse, 0,
                                   RawInputEvent::Button(map_mouse_button(button),
                                                         match next.cursor_position {
                                                             Some(position) => position,
                                                             None => (0.0, 0.0)
                                                         },
                                                         map_action(action),
                                                         map_modifiers(modifiers))));
        },
        glfw::WindowEvent::Scroll(x, y) => {
            raw.push(RawInput::new(time, DeviceType::Window, 0,
                                   RawInputEvent::Scroll(x/next.size.0,
                                                         y/next.size.1)));
        },
        glfw::WindowEvent::CursorPos(x, y) => {
            raw.push(RawInput::new(time, DeviceType::Mouse, 0,
                                   RawInputEvent::CursorPosition(x/next.size.0,
                                                                 y/next.size.1)));
            match next.cursor_position {
                Some((px, py)) => raw.push(RawInput::new(time, DeviceType::Mouse, 0,
                                                         RawInputEvent::Motion((x-px)/next.size.0,
                                                                               (y-py)/next.size.1))),
                None => ()
            };
            next.cursor_position = Some((x, y));
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
            next.size = (x as f64, y as f64);
        }
        _ => {
            debug!("{:?}", event);
        }
    };
    raw
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

fn map_keycode(keycode: glfw::Key) -> remawin::types::KeyCode {
    match keycode {
        glfw::Key::Space => KeyCode::Space,
        glfw::Key::Apostrophe => KeyCode::Apostrophe,
        glfw::Key::Comma => KeyCode::Comma,
        glfw::Key::Minus => KeyCode::Minus,
        glfw::Key::Period => KeyCode::Period,
        glfw::Key::Slash => KeyCode::Slash,
        glfw::Key::Num0 => KeyCode::Key0,
        glfw::Key::Num1 => KeyCode::Key1,
        glfw::Key::Num2 => KeyCode::Key2,
        glfw::Key::Num3 => KeyCode::Key3,
        glfw::Key::Num4 => KeyCode::Key4,
        glfw::Key::Num5 => KeyCode::Key5,
        glfw::Key::Num6 => KeyCode::Key6,
        glfw::Key::Num7 => KeyCode::Key7,
        glfw::Key::Num8 => KeyCode::Key8,
        glfw::Key::Num9 => KeyCode::Key9,
        glfw::Key::Semicolon => KeyCode::Semicolon,
        glfw::Key::Equal => KeyCode::Equals,
        glfw::Key::A => KeyCode::A,
        glfw::Key::B => KeyCode::B,
        glfw::Key::C => KeyCode::C,
        glfw::Key::D => KeyCode::D,
        glfw::Key::E => KeyCode::E,
        glfw::Key::F => KeyCode::F,
        glfw::Key::G => KeyCode::G,
        glfw::Key::H => KeyCode::H,
        glfw::Key::I => KeyCode::I,
        glfw::Key::J => KeyCode::J,
        glfw::Key::K => KeyCode::K,
        glfw::Key::L => KeyCode::L,
        glfw::Key::M => KeyCode::M,
        glfw::Key::N => KeyCode::N,
        glfw::Key::O => KeyCode::O,
        glfw::Key::P => KeyCode::P,
        glfw::Key::Q => KeyCode::Q,
        glfw::Key::R => KeyCode::R,
        glfw::Key::S => KeyCode::S,
        glfw::Key::T => KeyCode::T,
        glfw::Key::U => KeyCode::U,
        glfw::Key::V => KeyCode::V,
        glfw::Key::W => KeyCode::W,
        glfw::Key::X => KeyCode::X,
        glfw::Key::Y => KeyCode::Y,
        glfw::Key::Z => KeyCode::Z,
        glfw::Key::LeftBracket => KeyCode::LBracket,
        glfw::Key::Backslash => KeyCode::Backslash,
        glfw::Key::RightBracket => KeyCode::RBracket,
        glfw::Key::GraveAccent => KeyCode::Grave,
        glfw::Key::World1 => KeyCode::None,
        glfw::Key::World2 => KeyCode::None,
        glfw::Key::Escape => KeyCode::Escape,
        glfw::Key::Enter => KeyCode::Return,
        glfw::Key::Tab => KeyCode::Tab,
        glfw::Key::Backspace => KeyCode::Back,
        glfw::Key::Insert => KeyCode::Insert,
        glfw::Key::Delete => KeyCode::Delete,
        glfw::Key::Right => KeyCode::Right,
        glfw::Key::Left => KeyCode::Left,
        glfw::Key::Down => KeyCode::Down,
        glfw::Key::Up => KeyCode::Up,
        glfw::Key::PageUp => KeyCode::PageUp,
        glfw::Key::PageDown => KeyCode::PageDown,
        glfw::Key::Home => KeyCode::Home,
        glfw::Key::End => KeyCode::End,
        glfw::Key::CapsLock => KeyCode::Capital,
        glfw::Key::ScrollLock => KeyCode::Scroll,
        glfw::Key::NumLock => KeyCode::Numlock,
        glfw::Key::PrintScreen => KeyCode::Snapshot,
        glfw::Key::Pause => KeyCode::Pause,
        glfw::Key::F1 => KeyCode::F1,
        glfw::Key::F2 => KeyCode::F2,
        glfw::Key::F3 => KeyCode::F3,
        glfw::Key::F4 => KeyCode::F4,
        glfw::Key::F5 => KeyCode::F5,
        glfw::Key::F6 => KeyCode::F6,
        glfw::Key::F7 => KeyCode::F7,
        glfw::Key::F8 => KeyCode::F8,
        glfw::Key::F9 => KeyCode::F9,
        glfw::Key::F10 => KeyCode::F10,
        glfw::Key::F11 => KeyCode::F11,
        glfw::Key::F12 => KeyCode::F12,
        glfw::Key::F13 => KeyCode::F13,
        glfw::Key::F14 => KeyCode::F14,
        glfw::Key::F15 => KeyCode::F15,
        glfw::Key::F16 => KeyCode::None,
        glfw::Key::F17 => KeyCode::None,
        glfw::Key::F18 => KeyCode::None,
        glfw::Key::F19 => KeyCode::None,
        glfw::Key::F20 => KeyCode::None,
        glfw::Key::F21 => KeyCode::None,
        glfw::Key::F22 => KeyCode::None,
        glfw::Key::F23 => KeyCode::None,
        glfw::Key::F24 => KeyCode::None,
        glfw::Key::F25 => KeyCode::None,
        glfw::Key::Kp0 => KeyCode::Numpad0,
        glfw::Key::Kp1 => KeyCode::Numpad1,
        glfw::Key::Kp2 => KeyCode::Numpad2,
        glfw::Key::Kp3 => KeyCode::Numpad3,
        glfw::Key::Kp4 => KeyCode::Numpad4,
        glfw::Key::Kp5 => KeyCode::Numpad5,
        glfw::Key::Kp6 => KeyCode::Numpad6,
        glfw::Key::Kp7 => KeyCode::Numpad7,
        glfw::Key::Kp8 => KeyCode::Numpad8,
        glfw::Key::Kp9 => KeyCode::Numpad9,
        glfw::Key::KpDecimal => KeyCode::Decimal,
        glfw::Key::KpDivide => KeyCode::Divide,
        glfw::Key::KpMultiply => KeyCode::Multiply,
        glfw::Key::KpSubtract => KeyCode::Subtract,
        glfw::Key::KpAdd => KeyCode::Add,
        glfw::Key::KpEnter => KeyCode::NumpadEnter,
        glfw::Key::KpEqual => KeyCode::NumpadEquals,
        glfw::Key::LeftShift => KeyCode::LShift,
        glfw::Key::LeftControl => KeyCode::LControl,
        glfw::Key::LeftAlt => KeyCode::LAlt,
        glfw::Key::LeftSuper => KeyCode::LWin,
        glfw::Key::RightShift => KeyCode::RShift,
        glfw::Key::RightControl => KeyCode::RControl,
        glfw::Key::RightAlt => KeyCode::RAlt,
        glfw::Key::RightSuper => KeyCode::RWin,
        glfw::Key::Menu => KeyCode::LMenu,
        glfw::Key::Unknown => KeyCode::None,
    }
}
