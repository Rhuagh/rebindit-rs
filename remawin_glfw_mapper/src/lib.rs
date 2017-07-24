#[macro_use]
extern crate log;

extern crate remawin;
extern crate glfw;

use remawin::raw::{RawInput, RawInputEvent, RawInputAction, RawInputModifiers};
use remawin::types::{DeviceType, WindowData};
use remawin::InputReMapper;

pub struct GlfwEventMapper<C, I>
    where C: std::hash::Hash + std::cmp::Eq + std::str::FromStr +
             std::fmt::Debug + std::clone::Clone + remawin::types::ActionMetadata,
          I: std::hash::Hash + std::cmp::Eq + std::str::FromStr +
             std::fmt::Debug + std::clone::Clone {
    window_data : WindowData,
    input_remapper : InputReMapper<C, I>
}

impl <C, I> GlfwEventMapper<C, I>
    where C: std::hash::Hash + std::cmp::Eq + std::str::FromStr +
             std::fmt::Debug + std::clone::Clone + remawin::types::ActionMetadata,
          I: std::hash::Hash + std::cmp::Eq + std::str::FromStr +
             std::fmt::Debug + std::clone::Clone {
    pub fn new(current_size : (f64, f64), input_remapper: InputReMapper<C, I>) -> GlfwEventMapper<C, I> {
        GlfwEventMapper {
            window_data : WindowData {
                size : current_size,
                cursor_position : None
            },
            input_remapper : input_remapper
        }
    }

    pub fn process_events(&mut self, events : &Vec<(f64, glfw::WindowEvent)>) -> Vec<RawInput> {
        let mut next = self.window_data.clone();
        let raw = events.iter().flat_map(|e| process_event(e, &mut next)).collect();
        self.window_data = next;
        raw
    }

    pub fn process(&mut self, events : &Vec<(f64, glfw::WindowEvent)>) -> Vec<remawin::Event<C, I>> {
        let raw_input = self.process_events(events);
        self.input_remapper.process_raw_input(&raw_input)
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
        glfw::Key::Space => remawin::types::KeyCode::Space,
        glfw::Key::Apostrophe => remawin::types::KeyCode::Apostrophe,
        glfw::Key::Comma => remawin::types::KeyCode::Comma,
        glfw::Key::Minus => remawin::types::KeyCode::Minus,
        glfw::Key::Period => remawin::types::KeyCode::Period,
        glfw::Key::Slash => remawin::types::KeyCode::Slash,
        glfw::Key::Num0 => remawin::types::KeyCode::Key0,
        glfw::Key::Num1 => remawin::types::KeyCode::Key1,
        glfw::Key::Num2 => remawin::types::KeyCode::Key2,
        glfw::Key::Num3 => remawin::types::KeyCode::Key3,
        glfw::Key::Num4 => remawin::types::KeyCode::Key4,
        glfw::Key::Num5 => remawin::types::KeyCode::Key5,
        glfw::Key::Num6 => remawin::types::KeyCode::Key6,
        glfw::Key::Num7 => remawin::types::KeyCode::Key7,
        glfw::Key::Num8 => remawin::types::KeyCode::Key8,
        glfw::Key::Num9 => remawin::types::KeyCode::Key9,
        glfw::Key::Semicolon => remawin::types::KeyCode::Semicolon,
        glfw::Key::Equal => remawin::types::KeyCode::Equals,
        glfw::Key::A => remawin::types::KeyCode::A,
        glfw::Key::B => remawin::types::KeyCode::B,
        glfw::Key::C => remawin::types::KeyCode::C,
        glfw::Key::D => remawin::types::KeyCode::D,
        glfw::Key::E => remawin::types::KeyCode::E,
        glfw::Key::F => remawin::types::KeyCode::F,
        glfw::Key::G => remawin::types::KeyCode::G,
        glfw::Key::H => remawin::types::KeyCode::H,
        glfw::Key::I => remawin::types::KeyCode::I,
        glfw::Key::J => remawin::types::KeyCode::J,
        glfw::Key::K => remawin::types::KeyCode::K,
        glfw::Key::L => remawin::types::KeyCode::L,
        glfw::Key::M => remawin::types::KeyCode::M,
        glfw::Key::N => remawin::types::KeyCode::N,
        glfw::Key::O => remawin::types::KeyCode::O,
        glfw::Key::P => remawin::types::KeyCode::P,
        glfw::Key::Q => remawin::types::KeyCode::Q,
        glfw::Key::R => remawin::types::KeyCode::R,
        glfw::Key::S => remawin::types::KeyCode::S,
        glfw::Key::T => remawin::types::KeyCode::T,
        glfw::Key::U => remawin::types::KeyCode::U,
        glfw::Key::V => remawin::types::KeyCode::V,
        glfw::Key::W => remawin::types::KeyCode::W,
        glfw::Key::X => remawin::types::KeyCode::X,
        glfw::Key::Y => remawin::types::KeyCode::Y,
        glfw::Key::Z => remawin::types::KeyCode::Z,
        glfw::Key::LeftBracket => remawin::types::KeyCode::LBracket,
        glfw::Key::Backslash => remawin::types::KeyCode::Backslash,
        glfw::Key::RightBracket => remawin::types::KeyCode::RBracket,
        glfw::Key::GraveAccent => remawin::types::KeyCode::Grave,
        glfw::Key::World1 => remawin::types::KeyCode::None,
        glfw::Key::World2 => remawin::types::KeyCode::None,
        glfw::Key::Escape => remawin::types::KeyCode::Escape,
        glfw::Key::Enter => remawin::types::KeyCode::Return,
        glfw::Key::Tab => remawin::types::KeyCode::Tab,
        glfw::Key::Backspace => remawin::types::KeyCode::Back,
        glfw::Key::Insert => remawin::types::KeyCode::Insert,
        glfw::Key::Delete => remawin::types::KeyCode::Delete,
        glfw::Key::Right => remawin::types::KeyCode::Right,
        glfw::Key::Left => remawin::types::KeyCode::Left,
        glfw::Key::Down => remawin::types::KeyCode::Down,
        glfw::Key::Up => remawin::types::KeyCode::Up,
        glfw::Key::PageUp => remawin::types::KeyCode::PageUp,
        glfw::Key::PageDown => remawin::types::KeyCode::PageDown,
        glfw::Key::Home => remawin::types::KeyCode::Home,
        glfw::Key::End => remawin::types::KeyCode::End,
        glfw::Key::CapsLock => remawin::types::KeyCode::Capital,
        glfw::Key::ScrollLock => remawin::types::KeyCode::Scroll,
        glfw::Key::NumLock => remawin::types::KeyCode::Numlock,
        glfw::Key::PrintScreen => remawin::types::KeyCode::Snapshot,
        glfw::Key::Pause => remawin::types::KeyCode::Pause,
        glfw::Key::F1 => remawin::types::KeyCode::F1,
        glfw::Key::F2 => remawin::types::KeyCode::F2,
        glfw::Key::F3 => remawin::types::KeyCode::F3,
        glfw::Key::F4 => remawin::types::KeyCode::F4,
        glfw::Key::F5 => remawin::types::KeyCode::F5,
        glfw::Key::F6 => remawin::types::KeyCode::F6,
        glfw::Key::F7 => remawin::types::KeyCode::F7,
        glfw::Key::F8 => remawin::types::KeyCode::F8,
        glfw::Key::F9 => remawin::types::KeyCode::F9,
        glfw::Key::F10 => remawin::types::KeyCode::F10,
        glfw::Key::F11 => remawin::types::KeyCode::F11,
        glfw::Key::F12 => remawin::types::KeyCode::F12,
        glfw::Key::F13 => remawin::types::KeyCode::F13,
        glfw::Key::F14 => remawin::types::KeyCode::F14,
        glfw::Key::F15 => remawin::types::KeyCode::F15,
        glfw::Key::F16 => remawin::types::KeyCode::None,
        glfw::Key::F17 => remawin::types::KeyCode::None,
        glfw::Key::F18 => remawin::types::KeyCode::None,
        glfw::Key::F19 => remawin::types::KeyCode::None,
        glfw::Key::F20 => remawin::types::KeyCode::None,
        glfw::Key::F21 => remawin::types::KeyCode::None,
        glfw::Key::F22 => remawin::types::KeyCode::None,
        glfw::Key::F23 => remawin::types::KeyCode::None,
        glfw::Key::F24 => remawin::types::KeyCode::None,
        glfw::Key::F25 => remawin::types::KeyCode::None,
        glfw::Key::Kp0 => remawin::types::KeyCode::Numpad0,
        glfw::Key::Kp1 => remawin::types::KeyCode::Numpad1,
        glfw::Key::Kp2 => remawin::types::KeyCode::Numpad2,
        glfw::Key::Kp3 => remawin::types::KeyCode::Numpad3,
        glfw::Key::Kp4 => remawin::types::KeyCode::Numpad4,
        glfw::Key::Kp5 => remawin::types::KeyCode::Numpad5,
        glfw::Key::Kp6 => remawin::types::KeyCode::Numpad6,
        glfw::Key::Kp7 => remawin::types::KeyCode::Numpad7,
        glfw::Key::Kp8 => remawin::types::KeyCode::Numpad8,
        glfw::Key::Kp9 => remawin::types::KeyCode::Numpad9,
        glfw::Key::KpDecimal => remawin::types::KeyCode::Decimal,
        glfw::Key::KpDivide => remawin::types::KeyCode::Divide,
        glfw::Key::KpMultiply => remawin::types::KeyCode::Multiply,
        glfw::Key::KpSubtract => remawin::types::KeyCode::Subtract,
        glfw::Key::KpAdd => remawin::types::KeyCode::Add,
        glfw::Key::KpEnter => remawin::types::KeyCode::NumpadEnter,
        glfw::Key::KpEqual => remawin::types::KeyCode::NumpadEquals,
        glfw::Key::LeftShift => remawin::types::KeyCode::LShift,
        glfw::Key::LeftControl => remawin::types::KeyCode::LControl,
        glfw::Key::LeftAlt => remawin::types::KeyCode::LAlt,
        glfw::Key::LeftSuper => remawin::types::KeyCode::LWin,
        glfw::Key::RightShift => remawin::types::KeyCode::RShift,
        glfw::Key::RightControl => remawin::types::KeyCode::RControl,
        glfw::Key::RightAlt => remawin::types::KeyCode::RAlt,
        glfw::Key::RightSuper => remawin::types::KeyCode::RWin,
        glfw::Key::Menu => remawin::types::KeyCode::LMenu,
        glfw::Key::Unknown => remawin::types::KeyCode::None,
    }
}
