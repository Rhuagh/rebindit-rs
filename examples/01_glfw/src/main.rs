#[macro_use]
extern crate log;
extern crate glfw;
extern crate remawin;
extern crate remawin_glfw_mapper;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::sync::mpsc::Receiver;
use glfw::Context;

use remawin_glfw_mapper::GlfwEventMapper;
use remawin::{Event, WindowEvent, ControllerEvent};
use remawin::types::{MappedType, ActionMetadata, ActionArgument};

use std::default::Default;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ContextId {
    Default,
    UI
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Action {
    Close,
    Text,
    MoveForward,
    FireAbility1,
    RotateDirection
}

impl ActionMetadata for Action {
    fn mapped_type(&self) -> MappedType {
        match self {
            &Action::Close => MappedType::Action,
            &Action::Text => MappedType::Action,
            &Action::MoveForward => MappedType::State,
            &Action::FireAbility1 => MappedType::Action,
            &Action::RotateDirection => MappedType::Range,
        }
    }

    fn args(&self) -> Vec<ActionArgument> {
        match self {
            &Action::FireAbility1 => vec![ActionArgument::CursorPosition],
            _ => Vec::default()
        }
    }
}

fn window_init(width : u32, height: u32, title : &str) -> (glfw::Glfw, glfw::Window, Receiver<(f64, glfw::WindowEvent)>) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    let (mut window, events) =
        match glfw.create_window(width, height, title, glfw::WindowMode::Windowed) {
            Some(d) => d,
            None => panic!("Window was not created")
        };
    window.set_all_polling(true);
    (glfw, window, events)
}

fn poll_events(glfw: &mut glfw::Glfw, events : &Receiver<(f64, glfw::WindowEvent)>) -> Vec<(f64, glfw::WindowEvent)> {
    glfw.poll_events();
    let mut raw = Vec::default();
    for (time, event) in glfw::flush_messages(&events) {
        raw.push((time, event));
    }
    raw
}

fn main() {
    debug!("Starting");
    let (mut glfw, mut window, events) = window_init(1024, 768, "Test");
    debug!("Window initialized");

    let mut event_mapper = GlfwEventMapper::<Action, ContextId>::new((1024.0, 768.0));
    event_mapper.remapper_mut()
        .with_bindings_file("../../config/bindings.ron")
        .activate_context(&ContextId::Default, 1);

    while !window.should_close() {
        for event in event_mapper.process(&mut poll_events(&mut glfw, &events)) {
            match event {
                Event::Window(WindowEvent::Close) => {
                    println!("closing!");
                    window.set_should_close(true);
                },
                Event::Controller(ControllerEvent::Action(Action::Close, _)) => {
                    println!("closing!");
                    window.set_should_close(true);
                }
                _ => ()
            }
        }
        window.swap_buffers();
    }

    return
}
