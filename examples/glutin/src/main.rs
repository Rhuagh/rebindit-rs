#[macro_use]
extern crate log;
extern crate glutin;
extern crate input;
extern crate glutin_input_source;

use input::InputHandler;
use glutin_input_source::GlutinInputSource;
use input::event::{Event, WindowEvent, ControllerEvent};
use input::types::{MappedType, ToMappedType};

use std::str::FromStr;
use glutin::GlContext;

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
pub enum GameAction {
    MoveForward,
    FireAbility1,
    RotateDirection
}

impl ToMappedType for GameAction {
    fn to_mapped_type(&self) -> MappedType {
        match self {
            &GameAction::MoveForward => MappedType::State,
            &GameAction::FireAbility1 => MappedType::Action,
            &GameAction::RotateDirection => MappedType::Range,
        }
    }
}

impl FromStr for GameAction {
    type Err = ();

    fn from_str(s: &str) -> Result<GameAction, ()> {
        match s {
            "MoveForward" => Ok(GameAction::MoveForward),
            "FireAbility1" => Ok(GameAction::FireAbility1),
            "RotateDirection" => Ok(GameAction::RotateDirection),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ControllerAction {
    UI(UIAction),
    Combat(GameAction)
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
        match s.parse::<GameAction>() {
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

fn main() {
    debug!("Starting");
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    debug!("Window initialized");

    let mut input_handler = InputHandler::<ControllerAction>::new()
        .with_bindings_file("config/bindings.yml")
        .with_input_source(GlutinInputSource::new(events_loop, (1024.0, 768.0)));

    input_handler.activate_context("default", 1);

    let mut running = true;
    while running {
        for event in input_handler.process() {
            match event {
                Event::Window(WindowEvent::Close) => {
                    println!("closing!");
                    running = false;
                },
                Event::Controller(ControllerEvent::Action(ControllerAction::UI(UIAction::Close), _)) => {
                    println!("closing!");
                    running = false;
                }
                _ => ()
            }
        }
        gl_window.swap_buffers().unwrap();
    }

    return
}
