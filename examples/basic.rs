#[macro_use]
extern crate log;
extern crate glutin;
extern crate rebindit;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use rebindit::{Event, WindowEvent, ControllerEvent, InputRebinder};
use rebindit::types::{MappedType, ActionMetadata, ActionArgument};

use glutin::GlContext;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ContextId {
    Default,
    UI,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Action {
    Close,
    Text,
    MoveForward,
    FireAbility1,
    RotateDirection,
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
            _ => Vec::default(),
        }
    }
}

fn poll_events(events_loop: &mut glutin::EventsLoop) -> Vec<glutin::Event> {
    let mut raw = Vec::default();
    events_loop.poll_events(|event| { raw.push(event); });
    raw
}

fn main() {
    debug!("Starting");
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    debug!("Window initialized");

    let mut event_mapper = InputRebinder::<Action, ContextId>::new((1024.0, 768.0));
    event_mapper
        .with_contexts(&mut rebindit::util::contexts_from_file("config/simple.ron")
            .unwrap())
        .activate_context(&ContextId::Default, 1);

    let mut running = true;
    while running {
        for event in event_mapper.process(&poll_events(&mut events_loop)) {
            match event {
                Event::Window(WindowEvent::Close) => {
                    println!("closing!");
                    running = false;
                }
                Event::Controller(ControllerEvent::Action(Action::Close, _)) => {
                    println!("closing!");
                    running = false;
                }
                _ => {
                    println!("{:?}", event);
                }
            }
        }
        gl_window.swap_buffers().unwrap();
    }

    return;
}
