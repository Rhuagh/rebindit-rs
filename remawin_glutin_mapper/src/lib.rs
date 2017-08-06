extern crate remawin;
extern crate glutin;
extern crate time;

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

pub struct GlutinEventMapper<ACTION, ID>
    where ACTION: Hash + Eq + Clone,
          ID: Hash + Eq + Clone + Debug {
    frame_data: WindowData,
    input_remapper : InputReMapper<ACTION, ID>
}

impl <ACTION, ID> GlutinEventMapper<ACTION, ID>
    where ACTION: Hash + Eq + Clone + ActionMetadata + Debug + DeserializeOwned,
          ID: Hash + Eq + Clone + Debug + DeserializeOwned {

    pub fn new(size : (f64, f64)) -> GlutinEventMapper<ACTION, ID> {
        GlutinEventMapper {
            frame_data : WindowData {
                size : size,
                cursor_position : None
            },
            input_remapper : InputReMapper::new()
        }
    }

    fn process_events(&mut self, events: &Vec<glutin::Event>) -> Vec<RawInput> {
        let mut next = self.frame_data.clone();
        let raw = events.iter().flat_map(|e| process_event(e, &mut next)).collect();
        self.frame_data = next;
        raw
    }

    pub fn process(&mut self, events : &Vec<glutin::Event>) -> Vec<remawin::Event<ACTION, ID>> {
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

fn process_window_event(event: &glutin::WindowEvent, next: &mut WindowData) -> Vec<RawInput> {
    let t = time::precise_time_s();
    match event {
        &glutin::WindowEvent::Closed => {
            vec![RawInput::new(t, DeviceType::Window, 0, RawInputEvent::Close)]
        },
        &glutin::WindowEvent::Resized(x, y) => {
            next.size = (x as f64, y as f64);
            vec![RawInput::new(t, DeviceType::Window, 0,
                               RawInputEvent::Resize(x as u32, y as u32))]
        },
        &glutin::WindowEvent::Focused(b) => {
            vec![RawInput::new(t, DeviceType::Window, 0,
                               RawInputEvent::Focus(b))]
        },
        &glutin::WindowEvent::ReceivedCharacter(ch) => {
            vec![RawInput::new(t, DeviceType::Keyboard, 0,
                               RawInputEvent::Char(ch))]
        },
        &glutin::WindowEvent::KeyboardInput { input, .. } => {
            vec![RawInput::new(t, DeviceType::Keyboard, 0,
                               RawInputEvent::Key(map_keycode(&input.virtual_keycode),
                                                  map_action(&input.state),
                                                  map_modifiers(&input.modifiers)))]
        },
        &glutin::WindowEvent::MouseInput { state, button, .. } => {
            vec![RawInput::new(t, DeviceType::Mouse, 0,
                               RawInputEvent::Button(map_mouse_button(&button),
                                                     match next.cursor_position {
                                                         Some(position) => position,
                                                         None => (0.0, 0.0)
                                                     },
                                                     map_action(&state),
                                                     RawInputModifiers::empty()))]
        },
        &glutin::WindowEvent::MouseMoved { position : (x, y), .. } => {
            let mut raw = Vec::new();
            raw.push(RawInput::new(t, DeviceType::Mouse, 0,
                                   RawInputEvent::CursorPosition(x/next.size.0,
                                                                 y/next.size.1)));
            match next.cursor_position {
                Some((px, py)) => raw.push(RawInput::new(t, DeviceType::Mouse, 0,
                                                         RawInputEvent::Motion((x-px)/next.size.0,
                                                                               (y-py)/next.size.1))),
                None => ()
            };
            next.cursor_position = Some((x, y));
            raw
        },
        _ => Vec::default()
    }
}

fn process_event(event : &glutin::Event,
                 next : &mut WindowData) -> Vec<RawInput> {
    match event {
        &glutin::Event::WindowEvent { ref event, .. } => {
            process_window_event(event, next)
        },
        _ => Vec::default()
    }
}

fn map_action(element_state: &glutin::ElementState) -> RawInputAction {
    match element_state{
        &glutin::ElementState::Pressed => RawInputAction::Press,
        &glutin::ElementState::Released => RawInputAction::Release,
    }
}

fn map_modifiers(modifiers: &glutin::ModifiersState) -> RawInputModifiers {
    let mut m = RawInputModifiers::empty();
    m.set(remawin::raw::SHIFT, modifiers.shift);
    m.set(remawin::raw::CONTROL, modifiers.ctrl);
    m.set(remawin::raw::ALT, modifiers.alt);
    m.set(remawin::raw::SUPER, modifiers.logo);
    m
}

fn map_mouse_button(button: &glutin::MouseButton) -> u32 {
    match button {
        &glutin::MouseButton::Left => 1,
        &glutin::MouseButton::Right => 2,
        &glutin::MouseButton::Middle => 3,
        &glutin::MouseButton::Other(b) => b as u32,
    }
}

fn map_keycode(keycode: &Option<glutin::VirtualKeyCode>) -> remawin::types::KeyCode {
    match keycode {
        &Some(kc) => {
            match kc {
                glutin::VirtualKeyCode::Key1 => KeyCode::Key1,
                glutin::VirtualKeyCode::Key2 => KeyCode::Key2,
                glutin::VirtualKeyCode::Key3 => KeyCode::Key3,
                glutin::VirtualKeyCode::Key4 => KeyCode::Key4,
                glutin::VirtualKeyCode::Key5 => KeyCode::Key5,
                glutin::VirtualKeyCode::Key6 => KeyCode::Key6,
                glutin::VirtualKeyCode::Key7 => KeyCode::Key7,
                glutin::VirtualKeyCode::Key8 => KeyCode::Key8,
                glutin::VirtualKeyCode::Key9 => KeyCode::Key9,
                glutin::VirtualKeyCode::Key0 => KeyCode::Key0,
                glutin::VirtualKeyCode::A => KeyCode::A,
                glutin::VirtualKeyCode::B => KeyCode::B,
                glutin::VirtualKeyCode::C => KeyCode::C,
                glutin::VirtualKeyCode::D => KeyCode::D,
                glutin::VirtualKeyCode::E => KeyCode::E,
                glutin::VirtualKeyCode::F => KeyCode::F,
                glutin::VirtualKeyCode::G => KeyCode::G,
                glutin::VirtualKeyCode::H => KeyCode::H,
                glutin::VirtualKeyCode::I => KeyCode::I,
                glutin::VirtualKeyCode::J => KeyCode::J,
                glutin::VirtualKeyCode::K => KeyCode::K,
                glutin::VirtualKeyCode::L => KeyCode::L,
                glutin::VirtualKeyCode::M => KeyCode::M,
                glutin::VirtualKeyCode::N => KeyCode::N,
                glutin::VirtualKeyCode::O => KeyCode::O,
                glutin::VirtualKeyCode::P => KeyCode::P,
                glutin::VirtualKeyCode::Q => KeyCode::Q,
                glutin::VirtualKeyCode::R => KeyCode::R,
                glutin::VirtualKeyCode::S => KeyCode::S,
                glutin::VirtualKeyCode::T => KeyCode::T,
                glutin::VirtualKeyCode::U => KeyCode::U,
                glutin::VirtualKeyCode::V => KeyCode::V,
                glutin::VirtualKeyCode::W => KeyCode::W,
                glutin::VirtualKeyCode::X => KeyCode::X,
                glutin::VirtualKeyCode::Y => KeyCode::Y,
                glutin::VirtualKeyCode::Z => KeyCode::Z,
                glutin::VirtualKeyCode::Escape => KeyCode::Escape,
                glutin::VirtualKeyCode::F1 => KeyCode::F1,
                glutin::VirtualKeyCode::F2 => KeyCode::F2,
                glutin::VirtualKeyCode::F3 => KeyCode::F3,
                glutin::VirtualKeyCode::F4 => KeyCode::F4,
                glutin::VirtualKeyCode::F5 => KeyCode::F5,
                glutin::VirtualKeyCode::F6 => KeyCode::F6,
                glutin::VirtualKeyCode::F7 => KeyCode::F7,
                glutin::VirtualKeyCode::F8 => KeyCode::F8,
                glutin::VirtualKeyCode::F9 => KeyCode::F9,
                glutin::VirtualKeyCode::F10 => KeyCode::F10,
                glutin::VirtualKeyCode::F11 => KeyCode::F11,
                glutin::VirtualKeyCode::F12 => KeyCode::F12,
                glutin::VirtualKeyCode::F13 => KeyCode::F13,
                glutin::VirtualKeyCode::F14 => KeyCode::F14,
                glutin::VirtualKeyCode::F15 => KeyCode::F15,
                glutin::VirtualKeyCode::Snapshot => KeyCode::Snapshot,
                glutin::VirtualKeyCode::Scroll => KeyCode::Scroll,
                glutin::VirtualKeyCode::Pause => KeyCode::Pause,
                glutin::VirtualKeyCode::Insert => KeyCode::Insert,
                glutin::VirtualKeyCode::Home => KeyCode::Home,
                glutin::VirtualKeyCode::Delete => KeyCode::Delete,
                glutin::VirtualKeyCode::End => KeyCode::End,
                glutin::VirtualKeyCode::PageDown => KeyCode::PageDown,
                glutin::VirtualKeyCode::PageUp => KeyCode::PageUp,
                glutin::VirtualKeyCode::Left => KeyCode::Left,
                glutin::VirtualKeyCode::Up => KeyCode::Up,
                glutin::VirtualKeyCode::Right => KeyCode::Right,
                glutin::VirtualKeyCode::Down => KeyCode::Down,
                glutin::VirtualKeyCode::Back => KeyCode::Back,
                glutin::VirtualKeyCode::Return => KeyCode::Return,
                glutin::VirtualKeyCode::Space => KeyCode::Space,
                glutin::VirtualKeyCode::Compose => KeyCode::Compose,
                glutin::VirtualKeyCode::Numlock => KeyCode::Numlock,
                glutin::VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
                glutin::VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
                glutin::VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
                glutin::VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
                glutin::VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
                glutin::VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
                glutin::VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
                glutin::VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
                glutin::VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
                glutin::VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
                glutin::VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
                glutin::VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
                glutin::VirtualKeyCode::Add => KeyCode::Add,
                glutin::VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
                glutin::VirtualKeyCode::Apps => KeyCode::Apps,
                glutin::VirtualKeyCode::At => KeyCode::At,
                glutin::VirtualKeyCode::Ax => KeyCode::Ax,
                glutin::VirtualKeyCode::Backslash => KeyCode::Backslash,
                glutin::VirtualKeyCode::Calculator => KeyCode::Calculator,
                glutin::VirtualKeyCode::Capital => KeyCode::Capital,
                glutin::VirtualKeyCode::Colon => KeyCode::Colon,
                glutin::VirtualKeyCode::Comma => KeyCode::Comma,
                glutin::VirtualKeyCode::Convert => KeyCode::Convert,
                glutin::VirtualKeyCode::Decimal => KeyCode::Decimal,
                glutin::VirtualKeyCode::Divide => KeyCode::Divide,
                glutin::VirtualKeyCode::Equals => KeyCode::Equals,
                glutin::VirtualKeyCode::Grave => KeyCode::Grave,
                glutin::VirtualKeyCode::Kana => KeyCode::Kana,
                glutin::VirtualKeyCode::Kanji => KeyCode::Kanji,
                glutin::VirtualKeyCode::LAlt => KeyCode::LAlt,
                glutin::VirtualKeyCode::LBracket => KeyCode::LBracket,
                glutin::VirtualKeyCode::LControl => KeyCode::LControl,
                glutin::VirtualKeyCode::LMenu => KeyCode::LMenu,
                glutin::VirtualKeyCode::LShift => KeyCode::LShift,
                glutin::VirtualKeyCode::LWin => KeyCode::LWin,
                glutin::VirtualKeyCode::Mail => KeyCode::Mail,
                glutin::VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
                glutin::VirtualKeyCode::MediaStop => KeyCode::MediaStop,
                glutin::VirtualKeyCode::Minus => KeyCode::Minus,
                glutin::VirtualKeyCode::Multiply => KeyCode::Multiply,
                glutin::VirtualKeyCode::Mute => KeyCode::Mute,
                glutin::VirtualKeyCode::MyComputer => KeyCode::MyComputer,
                glutin::VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
                glutin::VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
                glutin::VirtualKeyCode::NextTrack => KeyCode::NextTrack,
                glutin::VirtualKeyCode::NoConvert => KeyCode::NoConvert,
                glutin::VirtualKeyCode::NumpadComma => KeyCode::NumpadComma,
                glutin::VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
                glutin::VirtualKeyCode::NumpadEquals => KeyCode::NumpadEquals,
                glutin::VirtualKeyCode::OEM102 => KeyCode::OEM102,
                glutin::VirtualKeyCode::Period => KeyCode::Period,
                glutin::VirtualKeyCode::PlayPause => KeyCode::PlayPause,
                glutin::VirtualKeyCode::Power => KeyCode::Power,
                glutin::VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
                glutin::VirtualKeyCode::RAlt => KeyCode::RAlt,
                glutin::VirtualKeyCode::RBracket => KeyCode::RBracket,
                glutin::VirtualKeyCode::RControl => KeyCode::RControl,
                glutin::VirtualKeyCode::RMenu => KeyCode::RMenu,
                glutin::VirtualKeyCode::RShift => KeyCode::RShift,
                glutin::VirtualKeyCode::RWin => KeyCode::RWin,
                glutin::VirtualKeyCode::Semicolon => KeyCode::Semicolon,
                glutin::VirtualKeyCode::Slash => KeyCode::Slash,
                glutin::VirtualKeyCode::Sleep => KeyCode::Sleep,
                glutin::VirtualKeyCode::Stop => KeyCode::Stop,
                glutin::VirtualKeyCode::Subtract => KeyCode::Subtract,
                glutin::VirtualKeyCode::Sysrq => KeyCode::Sysrq,
                glutin::VirtualKeyCode::Tab => KeyCode::Tab,
                glutin::VirtualKeyCode::Underline => KeyCode::Underline,
                glutin::VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
                glutin::VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
                glutin::VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
                glutin::VirtualKeyCode::Wake => KeyCode::Wake,
                glutin::VirtualKeyCode::WebBack => KeyCode::WebBack,
                glutin::VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
                glutin::VirtualKeyCode::WebForward => KeyCode::WebForward,
                glutin::VirtualKeyCode::WebHome => KeyCode::WebHome,
                glutin::VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
                glutin::VirtualKeyCode::WebSearch => KeyCode::WebSearch,
                glutin::VirtualKeyCode::WebStop => KeyCode::WebStop,
                glutin::VirtualKeyCode::Yen => KeyCode::Yen,
            }
        },
        &None => KeyCode::None
    }
}
