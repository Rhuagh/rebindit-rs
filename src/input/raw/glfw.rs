use glfw;
use glfw::Glfw;
use super::Adapter;
use super::{Input, InputAction, Modifiers, ButtonDevice, ButtonId, Scancode, InputEvent};

use std::sync::mpsc::Receiver;

pub struct GlfwAdapter {
    glfw : Glfw,
    events : Receiver<(f64, glfw::WindowEvent)>
}

impl GlfwAdapter {

    pub fn new(glfw: Glfw, events : Receiver<(f64, glfw::WindowEvent)>) -> GlfwAdapter {
        GlfwAdapter {
            glfw : glfw,
            events : events
        }
    }
}

fn map_action(input : glfw::Action) -> InputAction {
    match input {
        glfw::Action::Release => InputAction::Release,
        glfw::Action::Press => InputAction::Press,
        glfw::Action::Repeat => InputAction::Repeat
    }
}

fn map_modifiers(input : glfw::Modifiers) -> Modifiers {
    Modifiers::from_bits_truncate(input.bits() as u32)
}

fn map_mouse_button(button : glfw::MouseButton) -> ButtonId {
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

impl Adapter for GlfwAdapter {

    fn process(&mut self) -> Vec<InputEvent> {
        self.glfw.poll_events();
        let mut raw = vec![];
        for (time, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(_, scancode, action, modifiers) => {
                    raw.push((time, Input::Key(scancode as Scancode, map_action(action), map_modifiers(modifiers))));
                },
                glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                    raw.push((time, Input::Button(ButtonDevice::Mouse, map_mouse_button(button), map_action(action), map_modifiers(modifiers))));
                },
                glfw::WindowEvent::Scroll(x, y) => {
                    raw.push((time, Input::Scroll(x, y)));
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    raw.push((time, Input::CursorPosition(x, y)));
                }
                _ => {
                    debug!("{:?}", event);
                }
            }
        }
        raw
    }
}
