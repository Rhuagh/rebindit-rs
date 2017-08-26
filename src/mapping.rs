use super::types::*;
use super::event::*;

use winit;
use time;

use std::hash::Hash;
use std::cmp::Eq;
use std::clone::Clone;
use std::fmt::Debug;

impl<ACTION, ID> Context<ACTION, ID>
where
    ACTION: Hash + Eq + Debug + Clone,
    ID: Hash + Eq + Debug + Clone,
{
    pub fn process(
        &mut self,
        raw_input: &winit::Event,
        state_storage: &mut StateStorage<ACTION>,
        frame_data: &mut WindowData,
    ) -> Option<ControllerEvent<ACTION, ID>> {
        let event = self.process_internal(raw_input, state_storage, frame_data);
        match &event {
            &Some(ControllerEvent::State(ref c_action, ref state_action, _, _)) => {
                self.update_state_info(c_action, state_action, state_storage);
            }
            _ => (),
        };
        match *raw_input {
            winit::Event::WindowEvent {
                event: winit::WindowEvent::MouseMoved { position, .. }, ..
            } => {
                frame_data.cursor_position = Some((
                    position.0 / frame_data.size.0,
                    position.1 / frame_data.size.1,
                ))
            }
            winit::Event::WindowEvent {
                event: winit::WindowEvent::Resized(width, height), ..
            } => {
                frame_data.size = (width as f64, height as f64);
            }
            _ => (),
        };
        event
    }

    fn process_internal(
        &self,
        raw_input: &winit::Event,
        state_storage: &mut StateStorage<ACTION>,
        frame_data: &mut WindowData,
    ) -> Option<ControllerEvent<ACTION, ID>> {
        for m in &self.mappings {
            if check_mapping(m, raw_input, state_storage) {
                match m.mapped_type {
                    Some(MappedType::Action) => {
                        return Some(as_action(m, raw_input, &self.id, frame_data))
                    }
                    Some(MappedType::State) => {
                        return Some(self.as_state(m, raw_input, state_storage, frame_data));
                    }
                    Some(MappedType::Range) => {
                        return Some(as_range(m, raw_input, &self.id, frame_data))
                    }
                    _ => (),
                }
            }
        }
        None
    }

    fn as_state(
        &self,
        mapping: &Mapping<ACTION>,
        raw_input: &winit::Event,
        state_storage: &mut StateStorage<ACTION>,
        frame_data: &WindowData,
    ) -> ControllerEvent<ACTION, ID> {
        let state = match *raw_input {
            winit::Event::WindowEvent { ref event, .. } => {
                match *event {
                    winit::WindowEvent::KeyboardInput {
                        input: winit::KeyboardInput { state, .. }, ..
                    } |
                    winit::WindowEvent::MouseInput { state, .. } => Some(state.clone()),
                    _ => None,
                }
            }
            _ => None,
        };
        let c_action = mapping.action.clone();
        ControllerEvent::State(
            c_action.clone(),
            self.state_action(&c_action, &state.unwrap(), state_storage),
            self.state_duration(&c_action, state_storage),
            arguments(&mapping.action_args, raw_input, &self.id, frame_data),
        )
    }

    fn state_action(
        &self,
        c_action: &ACTION,
        raw_input: &winit::ElementState,
        state_storage: &mut StateStorage<ACTION>,
    ) -> StateAction {
        match *raw_input {
            winit::ElementState::Pressed => {
                match state_storage.states.get(c_action) {
                    Some(info) => {
                        if info.active {
                            StateAction::Active
                        } else {
                            StateAction::Activated
                        }
                    }
                    None => StateAction::Activated,
                }
            }
            winit::ElementState::Released => StateAction::Deactivated,
        }
    }

    fn state_duration(
        &self,
        c_action: &ACTION,
        state_storage: &mut StateStorage<ACTION>,
    ) -> StateDuration {
        let now = time::precise_time_ns() as f64 / 1000000000.0;
        match state_storage.states.get(c_action) {
            Some(info) => {
                if info.active && info.start_time <= now {
                    now - info.start_time
                } else {
                    0.0
                }
            }
            None => 0.0,
        }
    }

    fn update_state_info(
        &mut self,
        c_action: &ACTION,
        state_action: &StateAction,
        state_storage: &mut StateStorage<ACTION>,
    ) {
        let now = time::precise_time_ns() as f64 / 1000000000.;
        match state_action {
            &StateAction::Active |
            &StateAction::Activated => {
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
                match add {
                    Some(info) => state_storage.states.insert(c_action.clone(), info),
                    None => None,
                };
            }
            &StateAction::Deactivated => {
                match state_storage.states.get_mut(c_action) {
                    Some(ref mut info) => {
                        info.active = false;
                        info.stop_time = now;
                    }
                    None => (),
                };
            }
        };
    }
}

fn range_diff(raw_input: &winit::Event, frame_data: &mut WindowData) -> RangeDiff {
    match *raw_input {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::MouseMoved { position, .. }, ..
        } => {
            if let Some(previous) = frame_data.cursor_position {
                (
                    position.0 as f64 / frame_data.size.0 - previous.0,
                    position.1 as f64 / frame_data.size.1 - previous.1,
                )
            } else {
                (0.0, 0.0)
            }
        }
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
            &ActionArgument::Modifiers => get_modifiers(raw_input),
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
) -> ControllerEvent<ACTION, ID>
where
    ACTION: Debug + Clone,
    ID: Debug + Clone,
{
    ControllerEvent::Action(
        mapping.action.clone(),
        arguments(&mapping.action_args, raw_input, context_id, frame_data),
    )
}

fn as_range<ACTION, ID>(
    mapping: &Mapping<ACTION>,
    raw_input: &winit::Event,
    context_id: &ID,
    frame_data: &mut WindowData,
) -> ControllerEvent<ACTION, ID>
where
    ACTION: Debug + Clone,
    ID: Debug + Clone,
{
    ControllerEvent::Range(
        mapping.action.clone(),
        range_diff(raw_input, frame_data),
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
            event: winit::WindowEvent::MouseInput {
                ref state,
                ref button,
                ..
            },
            ..
        } => {
            check_button_id(config_button, button)
                && check_state(&mapping.state, state)
                //&& check_modifiers(&raw_args.modifier, modifiers)
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
            event: winit::WindowEvent::KeyboardInput {
                input: winit::KeyboardInput {
                    ref state,
                    ref modifiers,
                    ref virtual_keycode,
                    ..
                },
                ..
            },
            ..
        } => {
            check_keycode(keycode, virtual_keycode.as_ref().unwrap()) &&
                check_state(&mapping.state, state) &&
                check_modifiers(&mapping.modifier, modifiers) &&
                check_state_active(&mapping.state_active, state_storage)
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
        winit::Event::WindowEvent { event: winit::WindowEvent::MouseMoved { .. }, .. } => {
            check_state_active(&mapping.state_active, state_storage)
        }
        _ => false,
    }
}

fn check_char<ACTION: Clone + Hash + Eq>(
    mapping: &Mapping<ACTION>,
    raw_input: &winit::Event,
    state_storage: &StateStorage<ACTION>,
) -> bool {
    match *raw_input {
        winit::Event::WindowEvent { event: winit::WindowEvent::ReceivedCharacter(_), .. } => {
            check_state_active(&mapping.state_active, state_storage)
        }
        _ => false,
    }
}

fn check_state_active<ACTION: Clone + Hash + Eq>(
    state_active: &Option<ACTION>,
    state_storage: &StateStorage<ACTION>,
) -> bool {
    match state_active {
        &Some(ref state) => state_storage.is_active(state),
        &None => true,
    }
}

fn check_button_id(config_button: &MouseButton, raw_button_id: &winit::MouseButton) -> bool {
    *config_button == raw_button_id.into()
}

fn check_keycode(config_keycode: &KeyCode, raw_keycode: &winit::VirtualKeyCode) -> bool {
    *config_keycode == raw_keycode.into()
}

fn check_state(config_action: &Option<RawState>, raw_action: &winit::ElementState) -> bool {
    match *config_action {
        Some(ref c_action) => {
            match *c_action {
                RawState::Press => *raw_action == winit::ElementState::Pressed,
                RawState::Release => *raw_action == winit::ElementState::Released,
            }
        }
        None => true,
    }
}

fn check_modifiers(
    config_modifier: &Option<Modifier>,
    raw_modifiers: &winit::ModifiersState,
) -> bool {
    match *config_modifier {
        Some(Modifier::SHIFT) => raw_modifiers.shift,
        Some(Modifier::ALT) => raw_modifiers.alt,
        Some(Modifier::CONTROL) => raw_modifiers.ctrl,
        Some(Modifier::SUPER) => raw_modifiers.logo,
        None => true,
    }
}

fn get_keycode<ID>(raw_input: &winit::Event) -> Option<Argument<ID>>
where
    ID: Debug + Clone,
{
    match *raw_input {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::KeyboardInput {
                input: winit::KeyboardInput { virtual_keycode, .. }, ..
            },
            ..
        } => Some(Argument::KeyCode(virtual_keycode.as_ref().unwrap().into())),
        _ => None,
    }
}

fn get_modifiers<ID>(raw_input: &winit::Event) -> Option<Argument<ID>>
where
    ID: Debug + Clone,
{
    match *raw_input {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::KeyboardInput {
                input: winit::KeyboardInput { modifiers, .. }, ..
            },
            ..
        } => Some(Argument::Modifiers(modifiers.into())),
        _ => None,
    }
}

fn get_value<ID>(raw_input: &winit::Event) -> Option<Argument<ID>>
where
    ID: Debug + Clone,
{
    match *raw_input {
        winit::Event::WindowEvent { event: winit::WindowEvent::ReceivedCharacter(ch), .. } => {
            Some(Argument::Value(ch))
        }
        _ => None,
    }
}

fn get_action<ID>(raw_input: &winit::Event) -> Option<Argument<ID>>
where
    ID: Debug + Clone,
{
    match *raw_input {
        winit::Event::WindowEvent { ref event, .. } => {
            match *event {
                winit::WindowEvent::KeyboardInput {
                    input: winit::KeyboardInput { state, .. }, ..
                } |
                winit::WindowEvent::MouseInput { state, .. } => Some(Argument::Action(
                    state.clone().into(),
                )),
                _ => None,
            }
        }
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
