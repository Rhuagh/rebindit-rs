#[macro_use]
extern crate log;
extern crate glfw;
extern crate input;
extern crate glfw_input_source;

use std::sync::mpsc::Receiver;
use glfw::Context;

use input::InputHandler;
use glfw_input_source::GlfwInputSource;
use input::event::{Event, WindowEvent, ControllerEvent};
use input::types::{MappedType, ToMappedType};

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIAction {
    Close,
    Text
}

impl ToMappedType for UIAction {
    fn to_mapped_type(&self) -> MappedType {
        match self {
            &UIAction::Close => MappedType::Action,
            &UIAction::Text => MappedType::Action,
        }
    }
}

impl FromStr for UIAction {
    type Err = ();

    fn from_str(s: &str) -> Result<UIAction, ()> {
        match s {
            "Close" => Ok(UIAction::Close),
            "Text" => Ok(UIAction::Text),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CombatAction {
    MoveForward
}

impl ToMappedType for CombatAction {
    fn to_mapped_type(&self) -> MappedType {
        match self {
            &CombatAction::MoveForward => MappedType::State,
        }
    }
}

impl FromStr for CombatAction {
    type Err = ();

    fn from_str(s: &str) -> Result<CombatAction, ()> {
        match s {
            "MoveForward" => Ok(CombatAction::MoveForward),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ControllerAction {
    UI(UIAction),
    Combat(CombatAction)
}

impl ToMappedType for ControllerAction {
    fn to_mapped_type(&self) -> input::types::MappedType {
        match self {
            &ControllerAction::UI(ref action) => action.to_mapped_type(),
            &ControllerAction::Combat(ref action) => action.to_mapped_type(),
        }
    }
}

impl FromStr for ControllerAction {
    type Err = ();

    fn from_str(s: &str) -> Result<ControllerAction, ()> {
        match s.parse::<CombatAction>() {
            Ok(action) => return Ok(ControllerAction::Combat(action)),
            _ => ()
        };
        match s.parse::<UIAction>() {
            Ok(action) => return Ok(ControllerAction::UI(action)),
            _ => ()
        };
        Err(())
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

fn main() {
    debug!("Starting");
    let (glfw, mut window, events) = window_init(1024, 768, "Test");
    debug!("Window initialized");

    let mut input_handler = InputHandler::<ControllerAction>::new()
        .with_bindings_file("config/bindings.yml")
        .with_input_source(GlfwInputSource::new(glfw, events, (1024.0, 768.0)));

    input_handler.activate_context("default", 1);
    while !window.should_close() {
        for event in input_handler.process() {
            match event {
                Event::Window(WindowEvent::Close) => {
                    println!("closing!");
                    window.set_should_close(true);
                },
                Event::Controller(ControllerEvent::Action(ControllerAction::UI(UIAction::Close), _)) => {
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