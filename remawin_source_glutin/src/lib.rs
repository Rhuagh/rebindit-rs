extern crate remawin;
extern crate glutin;
extern crate time;

use remawin::raw::{RawInputSource, RawInput, RawInputEvent, RawInputAction, RawInputModifiers};
use remawin::types::{DeviceType, WindowPosition};

pub struct GlutinInputSource {
    events_loop : glutin::EventsLoop,
    last_cursor_position : Option<WindowPosition>,
    current_size : (f64, f64),
}

impl GlutinInputSource {
    pub fn new(events_loop: glutin::EventsLoop, current_size : (f64, f64)) -> GlutinInputSource {
        GlutinInputSource {
            events_loop : events_loop,
            last_cursor_position : None,
            current_size: current_size,
        }
    }
}

impl RawInputSource for GlutinInputSource {

    fn process(&mut self) -> Vec<RawInput> {
        let mut raw = vec![];
        let mut next_size = self.current_size.clone();
        let mut next_cursor_position = self.last_cursor_position.clone();
        self.events_loop.poll_events(|event| {
            let t = time::precise_time_s();
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glutin::WindowEvent::Closed => {
                            raw.push(RawInput::new(t, DeviceType::Window, 0,
                                                   RawInputEvent::Close));
                        },
                        glutin::WindowEvent::Resized(x, y) => {
                            raw.push(RawInput::new(t, DeviceType::Window, 0,
                                                   RawInputEvent::Resize(x as u32, y as u32)));
                            next_size = (x as f64, y as f64);
                        },
                        glutin::WindowEvent::Focused(b) => {
                            raw.push(RawInput::new(t, DeviceType::Window, 0,
                                                   RawInputEvent::Focus(b)));
                        },
                        glutin::WindowEvent::ReceivedCharacter(ch) => {
                            raw.push(RawInput::new(t, DeviceType::Keyboard, 0,
                                                   RawInputEvent::Char(ch)));
                        },
                        glutin::WindowEvent::KeyboardInput { input, .. } => {
                            raw.push(RawInput::new(t, DeviceType::Keyboard, 0,
                                                   RawInputEvent::Key(map_keycode(&input.virtual_keycode),
                                                                      map_action(&input.state),
                                                                      map_modifiers(&input.modifiers))));
                        },
                        glutin::WindowEvent::MouseInput { state, button, .. } => {
                            raw.push(RawInput::new(t, DeviceType::Mouse, 0,
                            RawInputEvent::Button(map_mouse_button(&button),
                                                  match next_cursor_position {
                                                      Some(position) => position,
                                                      None => (0.0, 0.0)
                                                  },
                                                  map_action(&state),
                                                  RawInputModifiers::empty())));
                        },
                        glutin::WindowEvent::MouseMoved { position : (x, y), .. } => {
                            raw.push(RawInput::new(t, DeviceType::Mouse, 0,
                                                   RawInputEvent::CursorPosition(x/next_size.0,
                                                                                 y/next_size.1)));
                            match next_cursor_position {
                                Some((px, py)) => raw.push(RawInput::new(t, DeviceType::Mouse, 0,
                                                                         RawInputEvent::Motion((x-px)/next_size.0,
                                                                                               (y-py)/next_size.1))),
                                None => ()
                            };
                            next_cursor_position = Some((x, y));
                        },
                        _ => ()
                    };
                },
                _ => ()
            };
        });
        self.last_cursor_position = next_cursor_position;
        self.current_size = next_size;
        raw
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
        _ => remawin::types::KeyCode::None // TODO
    }
}
