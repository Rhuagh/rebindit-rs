extern crate env_logger;
#[macro_use]
extern crate log;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate glutin;

extern crate rebindit;

use rebindit::*;
use rebindit::types::{Mapping, MouseButton, RawType};
use rebindit::event::ActionType;

use glutin::GlContext;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ContextId {
    Default,
    UI,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum UIAction {
    Close,
    Click,
    Drag,
    Text,
}

impl ActionMetadata for UIAction {
    fn mapped_type(&self) -> MappedType {
        match self {
            &UIAction::Close => MappedType::Action,
            &UIAction::Click => MappedType::State,
            &UIAction::Drag => MappedType::Range,
            &UIAction::Text => MappedType::Action,
        }
    }

    fn args(&self) -> Vec<ActionArgument> {
        match self {
            &UIAction::Click => vec![ActionArgument::CursorPosition],
            &UIAction::Text => vec![ActionArgument::Value],
            _ => Vec::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum GameAction {
    MoveForward,
    FireAbility1,
    RotateDirection,
    InternalButton,
    RotateCamera,
    ToggleUI,
}

impl ActionMetadata for GameAction {
    fn mapped_type(&self) -> MappedType {
        match self {
            &GameAction::MoveForward => MappedType::State,
            &GameAction::FireAbility1 => MappedType::Action,
            &GameAction::RotateDirection => MappedType::Range,
            &GameAction::InternalButton => MappedType::State,
            &GameAction::RotateCamera => MappedType::Range,
            &GameAction::ToggleUI => MappedType::Action,
        }
    }

    fn args(&self) -> Vec<ActionArgument> {
        match self {
            &GameAction::FireAbility1 => vec![ActionArgument::CursorPosition],
            _ => Vec::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum ControllerAction {
    UI(UIAction),
    Game(GameAction),
}

impl ActionMetadata for ControllerAction {
    fn mapped_type(&self) -> rebindit::types::MappedType {
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

fn poll_events(events_loop: &mut glutin::EventsLoop) -> Vec<glutin::Event> {
    let mut raw = Vec::default();
    events_loop.poll_events(|event| {
        raw.push(event);
    });
    raw
}

impl Into<ControllerAction> for UIAction {
    fn into(self) -> ControllerAction {
        ControllerAction::UI(self)
    }
}

impl Into<ControllerAction> for GameAction {
    fn into(self) -> ControllerAction {
        ControllerAction::Game(self)
    }
}

fn main() {
    env_logger::init().unwrap();
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

    let mut event_mapper = InputRebinder::<ControllerAction, ContextId>::new((1024.0, 768.0));
    event_mapper
        .with_context(
            Context::new(ContextId::UI)
                .with_mapping(Mapping::new(RawType::Char, UIAction::Text.into()))
                .with_mapping(Mapping::new(
                    RawType::Button(MouseButton::Left),
                    UIAction::Click.into(),
                ))
                .with_mapping(
                    Mapping::new(RawType::Motion, UIAction::Drag.into())
                        .with_state_active(UIAction::Click.into()),
                ),
        )
        .with_contexts(&mut rebindit::util::contexts_from_file(
            "examples/config/complex.ron",
        ).unwrap())
        .activate_context(&ContextId::Default, 1);

    let mut running = true;
    let mut ui_active = false;
    while running {
        for event in event_mapper.process(&poll_events(&mut events_loop)) {
            match event {
                Event::Close | Event::Controller(ControllerAction::UI(UIAction::Close), ..) => {
                    println!("closing!");
                    running = false;
                }
                Event::Controller(ControllerAction::Game(GameAction::ToggleUI), ..) => {
                    if ui_active {
                        ui_active = false;
                        event_mapper.deactivate_context(&ContextId::UI);
                    } else {
                        ui_active = true;
                        event_mapper.activate_context(&ContextId::UI, 2);
                    }
                }
                Event::Controller(x, ..) => {
                    println!("controller event: {:?}", x);
                }
                _ => (),
            }
        }
        gl_window.swap_buffers().unwrap();
    }

    return;
}
