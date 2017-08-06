#[macro_use]
extern crate log;
extern crate glutin;
extern crate remawin;
extern crate remawin_glutin_mapper;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use remawin_glutin_mapper::GlutinEventMapper;
use remawin::{Event, WindowEvent, ControllerEvent};
use remawin::types::{MappedType, ActionMetadata, ActionArgument};

use glutin::GlContext;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ContextId {
    Default,
    UI
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum GameAction {
    MoveForward,
    FireAbility1,
    RotateDirection,
    InternalButton,
    RotateCamera
}

impl ActionMetadata for GameAction {
    fn mapped_type(&self) -> MappedType {
        match self {
            &GameAction::MoveForward => MappedType::State,
            &GameAction::FireAbility1 => MappedType::Action,
            &GameAction::RotateDirection => MappedType::Range,
            &GameAction::InternalButton => MappedType::State,
            &GameAction::RotateCamera => MappedType::Range,
        }
    }

    fn args(&self) -> Vec<ActionArgument> {
        match self {
            &GameAction::FireAbility1 => vec![ActionArgument::CursorPosition],
            _ => Vec::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
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

fn poll_events(events_loop : &mut glutin::EventsLoop) -> Vec<glutin::Event> {
    let mut raw = Vec::default();
    events_loop.poll_events(|event| {
        raw.push(event);
    });
    raw
}

fn main() {
    debug!("Starting");
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("Hello, world!").with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    debug!("Window initialized");

    let mut event_mapper = GlutinEventMapper::<ControllerAction, ContextId>::new((1024.0, 768.0));
    event_mapper.remapper_mut()
        .with_bindings_file("bindings.ron")
        .activate_context(&ContextId::Default, 1);

    let mut running = true;
    while running {
        for event in event_mapper.process(&mut poll_events(&mut events_loop)) {
            match event {
                Event::Window(WindowEvent::Close) => {
                    println!("closing!");
                    running = false;
                },
                Event::Controller(ControllerEvent::Action(ControllerAction::UI(UIAction::Close), _)) => {
                    println!("closing!");
                    running = false;
                },
                Event::Controller(x) => {
                    println!("controller event: {:?}", x);
                },
                _ => ()
            }
        }
        gl_window.swap_buffers().unwrap();
    }

    return
}
