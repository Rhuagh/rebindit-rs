extern crate remawin;
extern crate glutin;
extern crate time;

use remawin::raw::{RawInput, RawInputEvent, RawInputAction, RawInputModifiers};
use remawin::types::{DeviceType, WindowData};
use remawin::InputReMapper;

pub struct GlutinEventMapper<C, I>
    where C: std::hash::Hash + std::cmp::Eq + std::str::FromStr +
             std::fmt::Debug + std::clone::Clone + remawin::types::ActionMetadata,
          I: std::hash::Hash + std::cmp::Eq + std::str::FromStr +
             std::fmt::Debug + std::clone::Clone {
    frame_data: WindowData,
    input_remapper : InputReMapper<C, I>
}

impl <C, I> GlutinEventMapper<C, I>
    where C: std::hash::Hash + std::cmp::Eq + std::str::FromStr +
             std::fmt::Debug + std::clone::Clone + remawin::types::ActionMetadata,
          I: std::hash::Hash + std::cmp::Eq + std::str::FromStr +
             std::fmt::Debug + std::clone::Clone{

    pub fn new(size : (f64, f64)) -> GlutinEventMapper<C, I> {
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

    pub fn process(&mut self, events : &Vec<glutin::Event>) -> Vec<remawin::Event<C, I>> {
        let raw_input = self.process_events(events);
        self.input_remapper.process_raw_input(&raw_input)
    }

    pub fn remapper_mut(&mut self) -> &mut InputReMapper<C, I> {
        &mut self.input_remapper
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
                glutin::VirtualKeyCode::Key1 => remawin::types::KeyCode::Key1,
                glutin::VirtualKeyCode::Key2 => remawin::types::KeyCode::Key2,
                glutin::VirtualKeyCode::Key3 => remawin::types::KeyCode::Key3,
                glutin::VirtualKeyCode::Key4 => remawin::types::KeyCode::Key4,
                glutin::VirtualKeyCode::Key5 => remawin::types::KeyCode::Key5,
                glutin::VirtualKeyCode::Key6 => remawin::types::KeyCode::Key6,
                glutin::VirtualKeyCode::Key7 => remawin::types::KeyCode::Key7,
                glutin::VirtualKeyCode::Key8 => remawin::types::KeyCode::Key8,
                glutin::VirtualKeyCode::Key9 => remawin::types::KeyCode::Key9,
                glutin::VirtualKeyCode::Key0 => remawin::types::KeyCode::Key0,
                glutin::VirtualKeyCode::A => remawin::types::KeyCode::A,
                glutin::VirtualKeyCode::B => remawin::types::KeyCode::B,
                glutin::VirtualKeyCode::C => remawin::types::KeyCode::C,
                glutin::VirtualKeyCode::D => remawin::types::KeyCode::D,
                glutin::VirtualKeyCode::E => remawin::types::KeyCode::E,
                glutin::VirtualKeyCode::F => remawin::types::KeyCode::F,
                glutin::VirtualKeyCode::G => remawin::types::KeyCode::G,
                glutin::VirtualKeyCode::H => remawin::types::KeyCode::H,
                glutin::VirtualKeyCode::I => remawin::types::KeyCode::I,
                glutin::VirtualKeyCode::J => remawin::types::KeyCode::J,
                glutin::VirtualKeyCode::K => remawin::types::KeyCode::K,
                glutin::VirtualKeyCode::L => remawin::types::KeyCode::L,
                glutin::VirtualKeyCode::M => remawin::types::KeyCode::M,
                glutin::VirtualKeyCode::N => remawin::types::KeyCode::N,
                glutin::VirtualKeyCode::O => remawin::types::KeyCode::O,
                glutin::VirtualKeyCode::P => remawin::types::KeyCode::P,
                glutin::VirtualKeyCode::Q => remawin::types::KeyCode::Q,
                glutin::VirtualKeyCode::R => remawin::types::KeyCode::R,
                glutin::VirtualKeyCode::S => remawin::types::KeyCode::S,
                glutin::VirtualKeyCode::T => remawin::types::KeyCode::T,
                glutin::VirtualKeyCode::U => remawin::types::KeyCode::U,
                glutin::VirtualKeyCode::V => remawin::types::KeyCode::V,
                glutin::VirtualKeyCode::W => remawin::types::KeyCode::W,
                glutin::VirtualKeyCode::X => remawin::types::KeyCode::X,
                glutin::VirtualKeyCode::Y => remawin::types::KeyCode::Y,
                glutin::VirtualKeyCode::Z => remawin::types::KeyCode::Z,
                glutin::VirtualKeyCode::Escape => remawin::types::KeyCode::Escape,
                glutin::VirtualKeyCode::F1 => remawin::types::KeyCode::F1,
                glutin::VirtualKeyCode::F2 => remawin::types::KeyCode::F2,
                glutin::VirtualKeyCode::F3 => remawin::types::KeyCode::F3,
                glutin::VirtualKeyCode::F4 => remawin::types::KeyCode::F4,
                glutin::VirtualKeyCode::F5 => remawin::types::KeyCode::F5,
                glutin::VirtualKeyCode::F6 => remawin::types::KeyCode::F6,
                glutin::VirtualKeyCode::F7 => remawin::types::KeyCode::F7,
                glutin::VirtualKeyCode::F8 => remawin::types::KeyCode::F8,
                glutin::VirtualKeyCode::F9 => remawin::types::KeyCode::F9,
                glutin::VirtualKeyCode::F10 => remawin::types::KeyCode::F10,
                glutin::VirtualKeyCode::F11 => remawin::types::KeyCode::F11,
                glutin::VirtualKeyCode::F12 => remawin::types::KeyCode::F12,
                glutin::VirtualKeyCode::F13 => remawin::types::KeyCode::F13,
                glutin::VirtualKeyCode::F14 => remawin::types::KeyCode::F14,
                glutin::VirtualKeyCode::F15 => remawin::types::KeyCode::F15,
                glutin::VirtualKeyCode::Snapshot => remawin::types::KeyCode::Snapshot,
                glutin::VirtualKeyCode::Scroll => remawin::types::KeyCode::Scroll,
                glutin::VirtualKeyCode::Pause => remawin::types::KeyCode::Pause,
                glutin::VirtualKeyCode::Insert => remawin::types::KeyCode::Insert,
                glutin::VirtualKeyCode::Home => remawin::types::KeyCode::Home,
                glutin::VirtualKeyCode::Delete => remawin::types::KeyCode::Delete,
                glutin::VirtualKeyCode::End => remawin::types::KeyCode::End,
                glutin::VirtualKeyCode::PageDown => remawin::types::KeyCode::PageDown,
                glutin::VirtualKeyCode::PageUp => remawin::types::KeyCode::PageUp,
                glutin::VirtualKeyCode::Left => remawin::types::KeyCode::Left,
                glutin::VirtualKeyCode::Up => remawin::types::KeyCode::Up,
                glutin::VirtualKeyCode::Right => remawin::types::KeyCode::Right,
                glutin::VirtualKeyCode::Down => remawin::types::KeyCode::Down,
                glutin::VirtualKeyCode::Back => remawin::types::KeyCode::Back,
                glutin::VirtualKeyCode::Return => remawin::types::KeyCode::Return,
                glutin::VirtualKeyCode::Space => remawin::types::KeyCode::Space,
                glutin::VirtualKeyCode::Compose => remawin::types::KeyCode::Compose,
                glutin::VirtualKeyCode::Numlock => remawin::types::KeyCode::Numlock,
                glutin::VirtualKeyCode::Numpad0 => remawin::types::KeyCode::Numpad0,
                glutin::VirtualKeyCode::Numpad1 => remawin::types::KeyCode::Numpad1,
                glutin::VirtualKeyCode::Numpad2 => remawin::types::KeyCode::Numpad2,
                glutin::VirtualKeyCode::Numpad3 => remawin::types::KeyCode::Numpad3,
                glutin::VirtualKeyCode::Numpad4 => remawin::types::KeyCode::Numpad4,
                glutin::VirtualKeyCode::Numpad5 => remawin::types::KeyCode::Numpad5,
                glutin::VirtualKeyCode::Numpad6 => remawin::types::KeyCode::Numpad6,
                glutin::VirtualKeyCode::Numpad7 => remawin::types::KeyCode::Numpad7,
                glutin::VirtualKeyCode::Numpad8 => remawin::types::KeyCode::Numpad8,
                glutin::VirtualKeyCode::Numpad9 => remawin::types::KeyCode::Numpad9,
                glutin::VirtualKeyCode::AbntC1 => remawin::types::KeyCode::AbntC1,
                glutin::VirtualKeyCode::AbntC2 => remawin::types::KeyCode::AbntC2,
                glutin::VirtualKeyCode::Add => remawin::types::KeyCode::Add,
                glutin::VirtualKeyCode::Apostrophe => remawin::types::KeyCode::Apostrophe,
                glutin::VirtualKeyCode::Apps => remawin::types::KeyCode::Apps,
                glutin::VirtualKeyCode::At => remawin::types::KeyCode::At,
                glutin::VirtualKeyCode::Ax => remawin::types::KeyCode::Ax,
                glutin::VirtualKeyCode::Backslash => remawin::types::KeyCode::Backslash,
                glutin::VirtualKeyCode::Calculator => remawin::types::KeyCode::Calculator,
                glutin::VirtualKeyCode::Capital => remawin::types::KeyCode::Capital,
                glutin::VirtualKeyCode::Colon => remawin::types::KeyCode::Colon,
                glutin::VirtualKeyCode::Comma => remawin::types::KeyCode::Comma,
                glutin::VirtualKeyCode::Convert => remawin::types::KeyCode::Convert,
                glutin::VirtualKeyCode::Decimal => remawin::types::KeyCode::Decimal,
                glutin::VirtualKeyCode::Divide => remawin::types::KeyCode::Divide,
                glutin::VirtualKeyCode::Equals => remawin::types::KeyCode::Equals,
                glutin::VirtualKeyCode::Grave => remawin::types::KeyCode::Grave,
                glutin::VirtualKeyCode::Kana => remawin::types::KeyCode::Kana,
                glutin::VirtualKeyCode::Kanji => remawin::types::KeyCode::Kanji,
                glutin::VirtualKeyCode::LAlt => remawin::types::KeyCode::LAlt,
                glutin::VirtualKeyCode::LBracket => remawin::types::KeyCode::LBracket,
                glutin::VirtualKeyCode::LControl => remawin::types::KeyCode::LControl,
                glutin::VirtualKeyCode::LMenu => remawin::types::KeyCode::LMenu,
                glutin::VirtualKeyCode::LShift => remawin::types::KeyCode::LShift,
                glutin::VirtualKeyCode::LWin => remawin::types::KeyCode::LWin,
                glutin::VirtualKeyCode::Mail => remawin::types::KeyCode::Mail,
                glutin::VirtualKeyCode::MediaSelect => remawin::types::KeyCode::MediaSelect,
                glutin::VirtualKeyCode::MediaStop => remawin::types::KeyCode::MediaStop,
                glutin::VirtualKeyCode::Minus => remawin::types::KeyCode::Minus,
                glutin::VirtualKeyCode::Multiply => remawin::types::KeyCode::Multiply,
                glutin::VirtualKeyCode::Mute => remawin::types::KeyCode::Mute,
                glutin::VirtualKeyCode::MyComputer => remawin::types::KeyCode::MyComputer,
                glutin::VirtualKeyCode::NavigateForward => remawin::types::KeyCode::NavigateForward,
                glutin::VirtualKeyCode::NavigateBackward => remawin::types::KeyCode::NavigateBackward,
                glutin::VirtualKeyCode::NextTrack => remawin::types::KeyCode::NextTrack,
                glutin::VirtualKeyCode::NoConvert => remawin::types::KeyCode::NoConvert,
                glutin::VirtualKeyCode::NumpadComma => remawin::types::KeyCode::NumpadComma,
                glutin::VirtualKeyCode::NumpadEnter => remawin::types::KeyCode::NumpadEnter,
                glutin::VirtualKeyCode::NumpadEquals => remawin::types::KeyCode::NumpadEquals,
                glutin::VirtualKeyCode::OEM102 => remawin::types::KeyCode::OEM102,
                glutin::VirtualKeyCode::Period => remawin::types::KeyCode::Period,
                glutin::VirtualKeyCode::PlayPause => remawin::types::KeyCode::PlayPause,
                glutin::VirtualKeyCode::Power => remawin::types::KeyCode::Power,
                glutin::VirtualKeyCode::PrevTrack => remawin::types::KeyCode::PrevTrack,
                glutin::VirtualKeyCode::RAlt => remawin::types::KeyCode::RAlt,
                glutin::VirtualKeyCode::RBracket => remawin::types::KeyCode::RBracket,
                glutin::VirtualKeyCode::RControl => remawin::types::KeyCode::RControl,
                glutin::VirtualKeyCode::RMenu => remawin::types::KeyCode::RMenu,
                glutin::VirtualKeyCode::RShift => remawin::types::KeyCode::RShift,
                glutin::VirtualKeyCode::RWin => remawin::types::KeyCode::RWin,
                glutin::VirtualKeyCode::Semicolon => remawin::types::KeyCode::Semicolon,
                glutin::VirtualKeyCode::Slash => remawin::types::KeyCode::Slash,
                glutin::VirtualKeyCode::Sleep => remawin::types::KeyCode::Sleep,
                glutin::VirtualKeyCode::Stop => remawin::types::KeyCode::Stop,
                glutin::VirtualKeyCode::Subtract => remawin::types::KeyCode::Subtract,
                glutin::VirtualKeyCode::Sysrq => remawin::types::KeyCode::Sysrq,
                glutin::VirtualKeyCode::Tab => remawin::types::KeyCode::Tab,
                glutin::VirtualKeyCode::Underline => remawin::types::KeyCode::Underline,
                glutin::VirtualKeyCode::Unlabeled => remawin::types::KeyCode::Unlabeled,
                glutin::VirtualKeyCode::VolumeDown => remawin::types::KeyCode::VolumeDown,
                glutin::VirtualKeyCode::VolumeUp => remawin::types::KeyCode::VolumeUp,
                glutin::VirtualKeyCode::Wake => remawin::types::KeyCode::Wake,
                glutin::VirtualKeyCode::WebBack => remawin::types::KeyCode::WebBack,
                glutin::VirtualKeyCode::WebFavorites => remawin::types::KeyCode::WebFavorites,
                glutin::VirtualKeyCode::WebForward => remawin::types::KeyCode::WebForward,
                glutin::VirtualKeyCode::WebHome => remawin::types::KeyCode::WebHome,
                glutin::VirtualKeyCode::WebRefresh => remawin::types::KeyCode::WebRefresh,
                glutin::VirtualKeyCode::WebSearch => remawin::types::KeyCode::WebSearch,
                glutin::VirtualKeyCode::WebStop => remawin::types::KeyCode::WebStop,
                glutin::VirtualKeyCode::Yen => remawin::types::KeyCode::Yen,
            }
        },
        &None => remawin::types::KeyCode::None
    }
}
