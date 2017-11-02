use super::event::*;
use super::types::*;

use time;
use winit;

use std::clone::Clone;
use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;

impl<ACTION, ID> Context<ACTION, ID>
where
    ACTION: Hash + Eq + Debug + Clone,
    ID: Hash + Eq + Debug + Clone,
{
    pub fn process(
        &self,
        raw_input: &winit::Event,
        state_storage: &mut StateStorage<ACTION>,
        frame_data: &mut WindowData,
    ) -> Option<Event<ACTION, ID>> {
        let event = process_internal(&self, raw_input, state_storage, frame_data);
        if let Some(Event::Controller(ref action, ActionType::State(ref state_action, _), _)) =
            event
        {
            update_state_info(action, state_action, state_storage);
        }
        match *raw_input {
            winit::Event::WindowEvent {
                event: winit::WindowEvent::MouseMoved { position, .. },
                ..
            } => {
                frame_data.cursor_position = Some((
                    position.0 / frame_data.size.0,
                    position.1 / frame_data.size.1,
                ))
            }
            winit::Event::WindowEvent {
                event: winit::WindowEvent::Resized(width, height),
                ..
            } => {
                frame_data.size = (width as f64, height as f64);
            }
            _ => (),
        };
        event
    }
}

fn process_internal<ACTION, ID>(
    context: &Context<ACTION, ID>,
    raw_input: &winit::Event,
    state_storage: &StateStorage<ACTION>,
    frame_data: &mut WindowData,
) -> Option<Event<ACTION, ID>>
where
    ACTION: Hash + Eq + Clone + Debug,
    ID: Clone + Debug,
{
    context
        .mappings
        .iter()
        .filter(|m| check_mapping(m, raw_input, state_storage))
        .filter_map(|m| {
            m.mapped_type.as_ref().map(|t| match *t {
                MappedType::Action => as_action(m, raw_input, &context.id, frame_data),
                MappedType::Range => as_range(m, raw_input, &context.id, frame_data),
                MappedType::State => as_state(m, raw_input, &context.id, frame_data, state_storage),
            })
        })
        .next()
}

fn as_state<ACTION, ID>(
    mapping: &Mapping<ACTION>,
    raw_input: &winit::Event,
    context_id: &ID,
    frame_data: &WindowData,
    state_storage: &StateStorage<ACTION>,
) -> Event<ACTION, ID>
where
    ACTION: Hash + Eq + Clone + Debug,
    ID: Clone + Debug,
{
    Event::Controller(
        mapping.action.clone(),
        ActionType::State(
            state_action(&mapping.action, &get_raw_state(raw_input), state_storage),
            state_duration(&mapping.action, state_storage),
        ),
        arguments(&mapping.action_args, raw_input, context_id, frame_data),
    )
}

fn get_raw_state(raw_input: &winit::Event) -> winit::ElementState {
    use winit::{Event, KeyboardInput, WindowEvent};
    match *raw_input {
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    input: KeyboardInput { state, .. },
                    ..
                },
            ..
        } |
        Event::WindowEvent {
            event: WindowEvent::MouseInput { state, .. },
            ..
        } => state.clone(),
        _ => winit::ElementState::Released,
    }
}

fn state_action<ACTION>(
    c_action: &ACTION,
    raw_input: &winit::ElementState,
    state_storage: &StateStorage<ACTION>,
) -> StateAction
where
    ACTION: Hash + Eq + Clone + Debug,
{
    match *raw_input {
        winit::ElementState::Pressed => if state_storage
            .states
            .get(c_action)
            .map(|i| i.active)
            .unwrap_or(false)
        {
            StateAction::Active
        } else {
            StateAction::Activated
        },
        winit::ElementState::Released => StateAction::Deactivated,
    }
}

fn state_duration<ACTION>(c_action: &ACTION, state_storage: &StateStorage<ACTION>) -> StateDuration
where
    ACTION: Hash + Eq + Clone + Debug,
{
    let now = time::precise_time_ns() as f64 / 1000000000.0;
    match state_storage.states.get(c_action) {
        Some(info) => if info.active && info.start_time <= now {
            now - info.start_time
        } else {
            0.0
        },
        None => 0.0,
    }
}

fn update_state_info<ACTION>(
    c_action: &ACTION,
    state_action: &StateAction,
    state_storage: &mut StateStorage<ACTION>,
) where
    ACTION: Hash + Eq + Clone + Debug,
{
    let now = time::precise_time_ns() as f64 / 1000000000.;
    match *state_action {
        StateAction::Active | StateAction::Activated => {
            let add = match state_storage.states.get_mut(c_action) {
                Some(ref mut info) => {
                    if !info.active {
                        info.active = true;
                        info.start_time = now;
                        info.stop_time = 0.0;
                    }
                    None
                }
                None => Some(StateInfo {
                    active: true,
                    start_time: now,
                    stop_time: 0.0,
                }),
            };
            if let Some(info) = add {
                state_storage.states.insert(c_action.clone(), info);
            }
        }
        StateAction::Deactivated => {
            if let Some(ref mut info) = state_storage.states.get_mut(c_action) {
                info.active = false;
                info.stop_time = now;
            }
        }
    };
}

fn range_diff(raw_input: &winit::Event, frame_data: &mut WindowData) -> RangeDiff {
    match *raw_input {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::MouseMoved { position, .. },
            ..
        } => if let Some(previous) = frame_data.cursor_position {
            (
                position.0 as f64 / frame_data.size.0 - previous.0,
                position.1 as f64 / frame_data.size.1 - previous.1,
            )
        } else {
            (0.0, 0.0)
        },
        _ => (0.0, 0.0),
    }
}

fn arguments<ID>(
    args: &Vec<ActionArgument>,
    raw_input: &winit::Event,
    context_id: &ID,
    frame_data: &WindowData,
) -> Vec<Argument<ID>>
where
    ID: Debug + Clone,
{
    args.iter()
        .filter_map(|arg| match arg {
            &ActionArgument::KeyCode => get_keycode(raw_input),
            &ActionArgument::Value => get_value(raw_input),
            &ActionArgument::Action => get_action(raw_input),
            &ActionArgument::CursorPosition => get_cursor_position(frame_data),
            &ActionArgument::ContextId => Some(Argument::ContextId(context_id.clone())),
        })
        .collect()
}

fn as_action<ACTION, ID>(
    mapping: &Mapping<ACTION>,
    raw_input: &winit::Event,
    context_id: &ID,
    frame_data: &WindowData,
) -> Event<ACTION, ID>
where
    ACTION: Debug + Clone,
    ID: Debug + Clone,
{
    Event::Controller(
        mapping.action.clone(),
        ActionType::Action,
        arguments(&mapping.action_args, raw_input, context_id, frame_data),
    )
}

fn as_range<ACTION, ID>(
    mapping: &Mapping<ACTION>,
    raw_input: &winit::Event,
    context_id: &ID,
    frame_data: &mut WindowData,
) -> Event<ACTION, ID>
where
    ACTION: Debug + Clone,
    ID: Debug + Clone,
{
    Event::Controller(
        mapping.action.clone(),
        ActionType::Range(range_diff(raw_input, frame_data)),
        arguments(&mapping.action_args, raw_input, context_id, frame_data),
    )
}

fn check_mapping<ACTION: Clone + Hash + Eq>(
    mapping: &Mapping<ACTION>,
    raw_input: &winit::Event,
    state_storage: &StateStorage<ACTION>,
) -> bool {
    match mapping.raw_type {
        RawType::Button(ref button) => check_button(button, mapping, raw_input, state_storage),
        RawType::Key(ref keycode) => check_key(keycode, mapping, raw_input, state_storage),
        RawType::Motion => check_motion(mapping, raw_input, state_storage),
        RawType::Char => check_char(mapping, raw_input, state_storage),
    }
}

fn check_button<ACTION: Clone + Hash + Eq>(
    config_button: &MouseButton,
    mapping: &Mapping<ACTION>,
    raw_input: &winit::Event,
    state_storage: &StateStorage<ACTION>,
) -> bool {
    match *raw_input {
        winit::Event::WindowEvent {
            event:
                winit::WindowEvent::MouseInput {
                    ref state,
                    ref button,
                    ..
                },
            ..
        } => {
            check_button_id(config_button, button) && check_state(&mapping.state, state)
                && check_state_active(&mapping.state_active, state_storage)
        }
        _ => false,
    }
}

fn check_key<ACTION: Clone + Hash + Eq>(
    keycode: &KeyCode,
    mapping: &Mapping<ACTION>,
    raw_input: &winit::Event,
    state_storage: &StateStorage<ACTION>,
) -> bool {
    match *raw_input {
        winit::Event::WindowEvent {
            event:
                winit::WindowEvent::KeyboardInput {
                    input:
                        winit::KeyboardInput {
                            ref state,
                            ref virtual_keycode,
                            ..
                        },
                    ..
                },
            ..
        } => {
            check_keycode(keycode, virtual_keycode.as_ref().unwrap())
                && check_state(&mapping.state, state)
                && check_state_active(&mapping.state_active, state_storage)
        }
        _ => false,
    }
}

fn check_motion<ACTION: Clone + Hash + Eq>(
    mapping: &Mapping<ACTION>,
    raw_input: &winit::Event,
    state_storage: &StateStorage<ACTION>,
) -> bool {
    match *raw_input {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::MouseMoved { .. },
            ..
        } => check_state_active(&mapping.state_active, state_storage),
        _ => false,
    }
}

fn check_char<ACTION: Clone + Hash + Eq>(
    mapping: &Mapping<ACTION>,
    raw_input: &winit::Event,
    state_storage: &StateStorage<ACTION>,
) -> bool {
    match *raw_input {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::ReceivedCharacter(_),
            ..
        } => check_state_active(&mapping.state_active, state_storage),
        _ => false,
    }
}

fn check_state_active<ACTION: Clone + Hash + Eq>(
    state_active: &Option<ACTION>,
    state_storage: &StateStorage<ACTION>,
) -> bool {
    state_active
        .as_ref()
        .map(|state| state_storage.is_active(state))
        .unwrap_or(true)
}

fn check_button_id(config_button: &MouseButton, raw_button_id: &winit::MouseButton) -> bool {
    *config_button == raw_button_id.into()
}

fn check_keycode(config_keycode: &KeyCode, raw_keycode: &winit::VirtualKeyCode) -> bool {
    *config_keycode == raw_keycode.into()
}

fn check_state(config_action: &Option<RawState>, raw_action: &winit::ElementState) -> bool {
    match (config_action, *raw_action) {
        (&Some(RawState::Press), winit::ElementState::Pressed) => true,
        (&Some(RawState::Release), winit::ElementState::Released) => true,
        (&None, _) => true,
        _ => false,
    }
}

fn get_keycode<ID>(raw_input: &winit::Event) -> Option<Argument<ID>>
where
    ID: Debug + Clone,
{
    match *raw_input {
        winit::Event::WindowEvent {
            event:
                winit::WindowEvent::KeyboardInput {
                    input:
                        winit::KeyboardInput {
                            virtual_keycode, ..
                        },
                    ..
                },
            ..
        } => Some(Argument::KeyCode(virtual_keycode.as_ref().unwrap().into())),
        _ => None,
    }
}

fn get_value<ID>(raw_input: &winit::Event) -> Option<Argument<ID>>
where
    ID: Debug + Clone,
{
    match *raw_input {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::ReceivedCharacter(ch),
            ..
        } => Some(Argument::Value(ch)),
        _ => None,
    }
}

fn get_action<ID>(raw_input: &winit::Event) -> Option<Argument<ID>>
where
    ID: Debug + Clone,
{
    match *raw_input {
        winit::Event::WindowEvent { ref event, .. } => match *event {
            winit::WindowEvent::KeyboardInput {
                input: winit::KeyboardInput { state, .. },
                ..
            } |
            winit::WindowEvent::MouseInput { state, .. } => {
                Some(Argument::Action(state.clone().into()))
            }
            _ => None,
        },
        _ => None,
    }
}

fn get_cursor_position<ID>(frame_data: &WindowData) -> Option<Argument<ID>>
where
    ID: Debug + Clone,
{
    match frame_data.cursor_position {
        Some((x, y)) => Some(Argument::CursorPosition(x, y)),
        None => None,
    }
}
