use winit;
use time;

use types::*;
use raw::*;
use raw;

pub fn process_event(event : &winit::Event,
                     next : &mut WindowData) -> Vec<RawInput> {
    match event {
        &winit::Event::WindowEvent { ref event, .. } => {
            process_window_event(event, next)
        },
        e => {
            println!("other event: {:?}", e);
            Vec::default()
        }
    }
}

fn process_window_event(event: &winit::WindowEvent, next: &mut WindowData) -> Vec<RawInput> {
    let t = time::precise_time_s();
    match *event {
        winit::WindowEvent::Closed => {
            vec![RawInput::new(t, DeviceType::Window, 0, RawInputEvent::Close)]
        },
        winit::WindowEvent::Resized(x, y) => {
            next.size = (x as f64, y as f64);
            vec![RawInput::new(t, DeviceType::Window, 0,
                               RawInputEvent::Resize(x as u32, y as u32))]
        },
        winit::WindowEvent::Focused(b) => {
            vec![RawInput::new(t, DeviceType::Window, 0,
                               RawInputEvent::Focus(b))]
        },
        winit::WindowEvent::ReceivedCharacter(ch) => {
            vec![RawInput::new(t, DeviceType::Keyboard, 0,
                               RawInputEvent::Char(ch))]
        },
        winit::WindowEvent::KeyboardInput { input, .. } => {
            vec![RawInput::new(t, DeviceType::Keyboard, 0,
                               RawInputEvent::Key(map_keycode(&input.virtual_keycode),
                                                  map_action(&input.state),
                                                  map_modifiers(&input.modifiers)))]
        },
        winit::WindowEvent::MouseInput { state, button, .. } => {
            vec![RawInput::new(t, DeviceType::Mouse, 0,
                               RawInputEvent::Button(map_mouse_button(&button),
                                                     match next.cursor_position {
                                                         Some(position) => position,
                                                         None => (0.0, 0.0)
                                                     },
                                                     map_action(&state),
                                                     RawInputModifiers::empty()))]
        },
        winit::WindowEvent::MouseMoved { position : (x, y), .. } => {
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

fn map_action(element_state: &winit::ElementState) -> RawInputAction {
    match element_state{
        &winit::ElementState::Pressed => RawInputAction::Press,
        &winit::ElementState::Released => RawInputAction::Release,
    }
}

fn map_modifiers(modifiers: &winit::ModifiersState) -> RawInputModifiers {
    let mut m = RawInputModifiers::empty();
    m.set(raw::SHIFT, modifiers.shift);
    m.set(raw::CONTROL, modifiers.ctrl);
    m.set(raw::ALT, modifiers.alt);
    m.set(raw::SUPER, modifiers.logo);
    m
}

fn map_mouse_button(button: &winit::MouseButton) -> u32 {
    match button {
        &winit::MouseButton::Left => 1,
        &winit::MouseButton::Right => 2,
        &winit::MouseButton::Middle => 3,
        &winit::MouseButton::Other(b) => b as u32,
    }
}

fn map_keycode(keycode: &Option<winit::VirtualKeyCode>) -> KeyCode {
    match keycode {
        &Some(kc) => {
            match kc {
                winit::VirtualKeyCode::Key1 => KeyCode::Key1,
                winit::VirtualKeyCode::Key2 => KeyCode::Key2,
                winit::VirtualKeyCode::Key3 => KeyCode::Key3,
                winit::VirtualKeyCode::Key4 => KeyCode::Key4,
                winit::VirtualKeyCode::Key5 => KeyCode::Key5,
                winit::VirtualKeyCode::Key6 => KeyCode::Key6,
                winit::VirtualKeyCode::Key7 => KeyCode::Key7,
                winit::VirtualKeyCode::Key8 => KeyCode::Key8,
                winit::VirtualKeyCode::Key9 => KeyCode::Key9,
                winit::VirtualKeyCode::Key0 => KeyCode::Key0,
                winit::VirtualKeyCode::A => KeyCode::A,
                winit::VirtualKeyCode::B => KeyCode::B,
                winit::VirtualKeyCode::C => KeyCode::C,
                winit::VirtualKeyCode::D => KeyCode::D,
                winit::VirtualKeyCode::E => KeyCode::E,
                winit::VirtualKeyCode::F => KeyCode::F,
                winit::VirtualKeyCode::G => KeyCode::G,
                winit::VirtualKeyCode::H => KeyCode::H,
                winit::VirtualKeyCode::I => KeyCode::I,
                winit::VirtualKeyCode::J => KeyCode::J,
                winit::VirtualKeyCode::K => KeyCode::K,
                winit::VirtualKeyCode::L => KeyCode::L,
                winit::VirtualKeyCode::M => KeyCode::M,
                winit::VirtualKeyCode::N => KeyCode::N,
                winit::VirtualKeyCode::O => KeyCode::O,
                winit::VirtualKeyCode::P => KeyCode::P,
                winit::VirtualKeyCode::Q => KeyCode::Q,
                winit::VirtualKeyCode::R => KeyCode::R,
                winit::VirtualKeyCode::S => KeyCode::S,
                winit::VirtualKeyCode::T => KeyCode::T,
                winit::VirtualKeyCode::U => KeyCode::U,
                winit::VirtualKeyCode::V => KeyCode::V,
                winit::VirtualKeyCode::W => KeyCode::W,
                winit::VirtualKeyCode::X => KeyCode::X,
                winit::VirtualKeyCode::Y => KeyCode::Y,
                winit::VirtualKeyCode::Z => KeyCode::Z,
                winit::VirtualKeyCode::Escape => KeyCode::Escape,
                winit::VirtualKeyCode::F1 => KeyCode::F1,
                winit::VirtualKeyCode::F2 => KeyCode::F2,
                winit::VirtualKeyCode::F3 => KeyCode::F3,
                winit::VirtualKeyCode::F4 => KeyCode::F4,
                winit::VirtualKeyCode::F5 => KeyCode::F5,
                winit::VirtualKeyCode::F6 => KeyCode::F6,
                winit::VirtualKeyCode::F7 => KeyCode::F7,
                winit::VirtualKeyCode::F8 => KeyCode::F8,
                winit::VirtualKeyCode::F9 => KeyCode::F9,
                winit::VirtualKeyCode::F10 => KeyCode::F10,
                winit::VirtualKeyCode::F11 => KeyCode::F11,
                winit::VirtualKeyCode::F12 => KeyCode::F12,
                winit::VirtualKeyCode::F13 => KeyCode::F13,
                winit::VirtualKeyCode::F14 => KeyCode::F14,
                winit::VirtualKeyCode::F15 => KeyCode::F15,
                winit::VirtualKeyCode::Snapshot => KeyCode::Snapshot,
                winit::VirtualKeyCode::Scroll => KeyCode::Scroll,
                winit::VirtualKeyCode::Pause => KeyCode::Pause,
                winit::VirtualKeyCode::Insert => KeyCode::Insert,
                winit::VirtualKeyCode::Home => KeyCode::Home,
                winit::VirtualKeyCode::Delete => KeyCode::Delete,
                winit::VirtualKeyCode::End => KeyCode::End,
                winit::VirtualKeyCode::PageDown => KeyCode::PageDown,
                winit::VirtualKeyCode::PageUp => KeyCode::PageUp,
                winit::VirtualKeyCode::Left => KeyCode::Left,
                winit::VirtualKeyCode::Up => KeyCode::Up,
                winit::VirtualKeyCode::Right => KeyCode::Right,
                winit::VirtualKeyCode::Down => KeyCode::Down,
                winit::VirtualKeyCode::Back => KeyCode::Back,
                winit::VirtualKeyCode::Return => KeyCode::Return,
                winit::VirtualKeyCode::Space => KeyCode::Space,
                winit::VirtualKeyCode::Compose => KeyCode::Compose,
                winit::VirtualKeyCode::Numlock => KeyCode::Numlock,
                winit::VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
                winit::VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
                winit::VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
                winit::VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
                winit::VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
                winit::VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
                winit::VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
                winit::VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
                winit::VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
                winit::VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
                winit::VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
                winit::VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
                winit::VirtualKeyCode::Add => KeyCode::Add,
                winit::VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
                winit::VirtualKeyCode::Apps => KeyCode::Apps,
                winit::VirtualKeyCode::At => KeyCode::At,
                winit::VirtualKeyCode::Ax => KeyCode::Ax,
                winit::VirtualKeyCode::Backslash => KeyCode::Backslash,
                winit::VirtualKeyCode::Calculator => KeyCode::Calculator,
                winit::VirtualKeyCode::Capital => KeyCode::Capital,
                winit::VirtualKeyCode::Colon => KeyCode::Colon,
                winit::VirtualKeyCode::Comma => KeyCode::Comma,
                winit::VirtualKeyCode::Convert => KeyCode::Convert,
                winit::VirtualKeyCode::Decimal => KeyCode::Decimal,
                winit::VirtualKeyCode::Divide => KeyCode::Divide,
                winit::VirtualKeyCode::Equals => KeyCode::Equals,
                winit::VirtualKeyCode::Grave => KeyCode::Grave,
                winit::VirtualKeyCode::Kana => KeyCode::Kana,
                winit::VirtualKeyCode::Kanji => KeyCode::Kanji,
                winit::VirtualKeyCode::LAlt => KeyCode::LAlt,
                winit::VirtualKeyCode::LBracket => KeyCode::LBracket,
                winit::VirtualKeyCode::LControl => KeyCode::LControl,
                winit::VirtualKeyCode::LMenu => KeyCode::LMenu,
                winit::VirtualKeyCode::LShift => KeyCode::LShift,
                winit::VirtualKeyCode::LWin => KeyCode::LWin,
                winit::VirtualKeyCode::Mail => KeyCode::Mail,
                winit::VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
                winit::VirtualKeyCode::MediaStop => KeyCode::MediaStop,
                winit::VirtualKeyCode::Minus => KeyCode::Minus,
                winit::VirtualKeyCode::Multiply => KeyCode::Multiply,
                winit::VirtualKeyCode::Mute => KeyCode::Mute,
                winit::VirtualKeyCode::MyComputer => KeyCode::MyComputer,
                winit::VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
                winit::VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
                winit::VirtualKeyCode::NextTrack => KeyCode::NextTrack,
                winit::VirtualKeyCode::NoConvert => KeyCode::NoConvert,
                winit::VirtualKeyCode::NumpadComma => KeyCode::NumpadComma,
                winit::VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
                winit::VirtualKeyCode::NumpadEquals => KeyCode::NumpadEquals,
                winit::VirtualKeyCode::OEM102 => KeyCode::OEM102,
                winit::VirtualKeyCode::Period => KeyCode::Period,
                winit::VirtualKeyCode::PlayPause => KeyCode::PlayPause,
                winit::VirtualKeyCode::Power => KeyCode::Power,
                winit::VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
                winit::VirtualKeyCode::RAlt => KeyCode::RAlt,
                winit::VirtualKeyCode::RBracket => KeyCode::RBracket,
                winit::VirtualKeyCode::RControl => KeyCode::RControl,
                winit::VirtualKeyCode::RMenu => KeyCode::RMenu,
                winit::VirtualKeyCode::RShift => KeyCode::RShift,
                winit::VirtualKeyCode::RWin => KeyCode::RWin,
                winit::VirtualKeyCode::Semicolon => KeyCode::Semicolon,
                winit::VirtualKeyCode::Slash => KeyCode::Slash,
                winit::VirtualKeyCode::Sleep => KeyCode::Sleep,
                winit::VirtualKeyCode::Stop => KeyCode::Stop,
                winit::VirtualKeyCode::Subtract => KeyCode::Subtract,
                winit::VirtualKeyCode::Sysrq => KeyCode::Sysrq,
                winit::VirtualKeyCode::Tab => KeyCode::Tab,
                winit::VirtualKeyCode::Underline => KeyCode::Underline,
                winit::VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
                winit::VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
                winit::VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
                winit::VirtualKeyCode::Wake => KeyCode::Wake,
                winit::VirtualKeyCode::WebBack => KeyCode::WebBack,
                winit::VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
                winit::VirtualKeyCode::WebForward => KeyCode::WebForward,
                winit::VirtualKeyCode::WebHome => KeyCode::WebHome,
                winit::VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
                winit::VirtualKeyCode::WebSearch => KeyCode::WebSearch,
                winit::VirtualKeyCode::WebStop => KeyCode::WebStop,
                winit::VirtualKeyCode::Yen => KeyCode::Yen,
            }
        },
        &None => KeyCode::None
    }
}
