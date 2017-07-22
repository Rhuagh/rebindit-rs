#[macro_use]
extern crate log;
extern crate glutin;
extern crate remawin;
extern crate remawin_source_glutin;

use remawin::InputHandler;
use remawin_source_glutin::GlutinInputSource;
use remawin::{Event, WindowEvent, ControllerEvent};
use remawin::types::{MappedType, ActionMetadata, ActionArgument};

use std::str::FromStr;
use glutin::GlContext;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIAction {
    Close,
    Text
}

impl ActionMetadata for UIAction {
    fn mapped_type(&self) -> MappedType {
        match self {
            &UIAction::Close => MappedType::Action,
            &UIAction::Text => MappedType::Action,
        }
    }

    fn args(&self) -> Vec<ActionArgument> {
        Vec::default()
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

impl ActionMetadata for GameAction {
    fn mapped_type(&self) -> MappedType {
        match self {
            &GameAction::MoveForward => MappedType::State,
            &GameAction::FireAbility1 => MappedType::Action,
            &GameAction::RotateDirection => MappedType::Range,
        }
    }

    fn args(&self) -> Vec<ActionArgument> {
        match self {
            &GameAction::FireAbility1 => vec![ActionArgument::CursorPosition],
            _ => Vec::default()
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
    Game(GameAction)
}

impl ActionMetadata for ControllerAction {
    fn mapped_type(&self) -> remawin::types::MappedType {
        match self {
            &ControllerAction::UI(ref action) => action.mapped_type(),
            &ControllerAction::Game(ref action) => action.mapped_type(),
        }
    }

    fn args(&self) -> Vec<ActionArgument> {
        match self {
            &ControllerAction::UI(ref action) => action.args(),
            &ControllerAction::Game(ref action) => action.args(),
        }
    }
}

impl FromStr for ControllerAction {
    type Err = ();

    fn from_str(s: &str) -> Result<ControllerAction, ()> {
        match s.parse::<GameAction>() {
            Ok(action) => return Ok(ControllerAction::Game(action)),
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
    let window = glutin::WindowBuilder::new().with_title("Hello, world!").with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new().with_vsync(true);
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
