#[macro_use]
extern crate log;
extern crate glutin;
extern crate remawin;
extern crate remawin_source_glutin;

use remawin::InputHandler;
use remawin_source_glutin::{GlutinInputSource, InputMode};
use remawin::{Event, WindowEvent, ControllerEvent};
use remawin::types::{MappedType, ActionMetadata, ActionArgument};

use std::str::FromStr;
use glutin::GlContext;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Action, ()> {
        match s {
            "Close" => Ok(Action::Close),
            "Text" => Ok(Action::Text),
            "MoveForward" => Ok(Action::MoveForward),
            "FireAbility1" => Ok(Action::FireAbility1),
            "RotateDirection" => Ok(Action::RotateDirection),
            _ => Err(())
        }
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

    let mut input_handler = InputHandler::<Action>::new()
        .with_bindings_file("config/bindings.yml")
        .with_input_source(GlutinInputSource::new(InputMode::PollEventsLoop,
                                                  Some(events_loop),
                                                  (1024.0, 768.0)));

    input_handler.activate_context("default", 1);

    let mut running = true;
    while running {
        for event in input_handler.process() {
            match event {
                Event::Window(WindowEvent::Close) => {
                    println!("closing!");
                    running = false;
                },
                Event::Controller(ControllerEvent::Action(Action::Close, _)) => {
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
