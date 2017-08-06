use super::raw::{RawInput, RawInputAction, RawInputEvent, SUPER, ALT, CONTROL, SHIFT, RawInputModifiers};
use super::types::*;
use super::event::*;

use std::hash::Hash;
use std::cmp::Eq;
use std::clone::Clone;
use std::fmt::Debug;

impl <ACTION, ID> Context<ACTION, ID>
    where ACTION: Hash + Eq + Debug + Clone,
          ID: Hash + Eq + Debug + Clone {

    pub fn process(&mut self,
                   raw_input : &RawInput,
                   state_storage: &mut StateStorage<ACTION>) -> Option<ControllerEvent<ACTION, ID>> {
        let event = self.process_internal(raw_input, state_storage);
        match &event {
            &Some(ControllerEvent::State(ref c_action, ref state_action, _, _)) => {
                self.update_state_info(c_action, state_action, raw_input, state_storage);
            },
            _ => ()
        };
        event
    }

    fn process_internal(&self,
                        raw_input : &RawInput,
                        state_storage : &mut StateStorage<ACTION>) -> Option<ControllerEvent<ACTION, ID>> {
        for m in &self.mappings {
            if check_mapping(m, raw_input, state_storage) {
                match m.mapped_type {
                    Some(MappedType::Action) => return Some(as_action(m, raw_input, &self.id)),
                    Some(MappedType::State) => {
                        return Some(self.as_state(m, raw_input, state_storage));
                    },
                    Some(MappedType::Range) => return Some(as_range(m, raw_input, &self.id)),
                    _ => ()
                }
            }
        }
        None
    }

    fn as_state(&self,
                mapping: &Mapping<ACTION>,
                raw_input : &RawInput,
                state_storage: &mut StateStorage<ACTION>) -> ControllerEvent<ACTION, ID> {
        let r_action = match raw_input.event {
            RawInputEvent::Key(_, ref action, _) => Some(action.clone()),
            RawInputEvent::Button(_, _, ref action, _) => Some(action.clone()),
            _ => None
        };
        let c_action = mapping.action.clone();
        ControllerEvent::State(c_action.clone(),
                                      self.state_action(&c_action, &r_action.unwrap(), state_storage),
                                      self.state_duration(&c_action, raw_input, state_storage),
                                      arguments(&mapping.action_args, raw_input, &self.id))
    }

    fn state_action(&self,
                    c_action: &ACTION,
                    raw_input : &RawInputAction,
                    state_storage: &mut StateStorage<ACTION>) -> StateAction {
        match raw_input {
            &RawInputAction::Press | &RawInputAction::Repeat => {
                match state_storage.states.get(c_action) {
                    Some(info) => {
                        if info.active {
                            StateAction::Active
                        } else {
                            StateAction::Activated
                        }
                    },
                    None => StateAction::Activated
                }
            },
            &RawInputAction::Release => StateAction::Deactivated
        }
    }

    fn state_duration(&self,
                      c_action: &ACTION,
                      raw_input : &RawInput,
                      state_storage: &mut StateStorage<ACTION>) -> StateDuration {
        match state_storage.states.get(c_action) {
            Some(info) => {
                if info.active && info.start_time <= raw_input.time {
                    raw_input.time - info.start_time
                } else {
                    0.0
                }
            },
            None => 0.0
        }
    }

    fn update_state_info(&mut self,
                         c_action: &ACTION,
                         state_action: &StateAction,
                         raw_input : &RawInput,
                         state_storage: &mut StateStorage<ACTION>) {
        match state_action {
            &StateAction::Active | &StateAction::Activated => {
                let add = match state_storage.states.get_mut(c_action) {
                    Some(ref mut info) => {
                        if !info.active {
                            info.active = true;
                            info.start_time = raw_input.time;
                            info.stop_time = 0.0;
                        }
                        None
                    },
                    None => Some(StateInfo {
                        active : true,
                        start_time : raw_input.time,
                        stop_time : 0.0
                    })
                };
                match add {
                    Some(info) => state_storage.states.insert(c_action.clone(), info),
                    None => None
                };
            },
            &StateAction::Deactivated => {
                match state_storage.states.get_mut(c_action) {
                    Some(ref mut info) => {
                        info.active = false;
                        info.stop_time = raw_input.time;
                    },
                    None => ()
                };
            }
        };
    }

}

fn range_diff(raw_input : &RawInput) -> RangeDiff {
    match raw_input.event {
        RawInputEvent::Motion(x, y) => (x, y),
        _ => (0.0, 0.0)
    }
}

fn arguments<ID>(args : &Vec<ActionArgument>,
                 raw_input : &RawInput,
                 context_id : &ID) -> Vec<Argument<ID>>
    where ID: Debug + Clone {
    args.iter().filter_map(|arg| {
        match arg {
            &ActionArgument::KeyCode => get_keycode(raw_input),
            &ActionArgument::Value => get_value(raw_input),
            &ActionArgument::Modifiers => get_modifiers(raw_input),
            &ActionArgument::Action => get_action(raw_input),
            &ActionArgument::CursorPosition => get_cursor_position(raw_input),
            &ActionArgument::ContextId => Some(Argument::ContextId(context_id.clone())),
        }
    }).collect()
}

fn as_action<ACTION, ID>(mapping: &Mapping<ACTION>,
                         raw_input : &RawInput,
                         context_id : &ID) -> ControllerEvent<ACTION, ID>
    where ACTION: Debug + Clone,
          ID: Debug + Clone {
    ControllerEvent::Action(mapping.action.clone(),
                                   arguments(&mapping.action_args, raw_input, context_id))
}

fn as_range<ACTION, ID>(mapping: &Mapping<ACTION>,
                        raw_input: &RawInput,
                        context_id : &ID) -> ControllerEvent<ACTION, ID>
    where ACTION: Debug + Clone,
          ID: Debug + Clone {
    ControllerEvent::Range(mapping.action.clone(),
                                  range_diff(raw_input),
                                  arguments(&mapping.action_args, raw_input, context_id))
}

fn check_mapping<ACTION: Clone + Hash + Eq>(mapping : &Mapping<ACTION>,
                                raw_input : &RawInput,
                                state_storage : &StateStorage<ACTION>) -> bool {
    match mapping.raw_type {
        RawType::Button => check_button(&mapping.raw_args, raw_input, state_storage),
        RawType::Key => check_key(&mapping.raw_args, raw_input, state_storage),
        RawType::Motion => check_motion(&mapping.raw_args, raw_input, state_storage),
        RawType::Char => check_char(&mapping.raw_args, raw_input, state_storage),
    }
}

fn check_button<ACTION: Clone + Hash + Eq>(raw_args : &RawArgs<ACTION>,
                               raw_input: &RawInput,
                               state_storage : &StateStorage<ACTION>) -> bool {
    match raw_input.event {
        RawInputEvent::Button(ref button_id, _, ref action, ref modifiers) => {
            check_button_id(&raw_args.button, button_id)
                && check_action(&raw_args.action, action)
                && check_modifiers(&raw_args.modifier, modifiers)
                && check_state_active(&raw_args.state_active, state_storage)
        },
        _ => false
    }
}

fn check_key<ACTION: Clone + Hash + Eq>(raw_args : &RawArgs<ACTION>,
                            raw_input: &RawInput,
                            state_storage : &StateStorage<ACTION>) -> bool {
    match raw_input.event {
        RawInputEvent::Key(ref keycode, ref action, ref modifiers) => {
            check_keycode(&raw_args.keycode, keycode)
                && check_action(&raw_args.action, action)
                && check_modifiers(&raw_args.modifier, modifiers)
                && check_state_active(&raw_args.state_active, state_storage)
        },
        _ => false
    }
}

#[allow(unused_variables)]
fn check_motion<ACTION: Clone + Hash + Eq>(raw_args : &RawArgs<ACTION>,
                               raw_input: &RawInput,
                               state_storage : &StateStorage<ACTION>) -> bool {
    match raw_input.event {
        RawInputEvent::Motion(_, _) => check_state_active(&raw_args.state_active, state_storage),
        _ => false
    }
}

#[allow(unused_variables)]
fn check_char<ACTION: Clone + Hash + Eq>(raw_args : &RawArgs<ACTION>,
                             raw_input: &RawInput,
                             state_storage : &StateStorage<ACTION>) -> bool {
    match raw_input.event {
        RawInputEvent::Char(_) => check_state_active(&raw_args.state_active, state_storage),
        _ => false
    }
}

fn check_state_active<ACTION: Clone + Hash + Eq>(state_active : &Option<ACTION>,
                                                 state_storage: &StateStorage<ACTION>) -> bool {
    match state_active {
        &Some(ref state) => state_storage.is_active(state),
        &None => true
    }
}

fn check_button_id(config_button: &Option<ButtonId>, raw_button_id : &u32) -> bool {
    match config_button {
        &Some(button_id) => button_id == *raw_button_id,
        &None => true
    }
}

fn check_keycode(config_keycode: &Option<KeyCode>, raw_keycode: &KeyCode) -> bool {
    match config_keycode {
        &Some(ref c_keycode) => *c_keycode == *raw_keycode,
        &None => true
    }
}

fn check_action(config_action: &Option<RawAction>, raw_action: &RawInputAction) -> bool {
    match config_action {
        &Some(ref c_action) => {
            match c_action {
                &RawAction::Press => *raw_action == RawInputAction::Press,
                &RawAction::Repeat => *raw_action == RawInputAction::Repeat,
                &RawAction::Release => *raw_action == RawInputAction::Release,
            }
        },
        &None => true
    }
}

fn check_modifiers(config_modifier: &Option<Modifier>, raw_modifiers: &RawInputModifiers) -> bool {
    match config_modifier {
        &Some(Modifier::SHIFT) => *raw_modifiers == SHIFT,
        &Some(Modifier::ALT) => *raw_modifiers == ALT,
        &Some(Modifier::CONTROL) => *raw_modifiers == CONTROL,
        &Some(Modifier::SUPER) => *raw_modifiers == SUPER,
        &None => true
    }
}

fn get_keycode<ID>(raw_input : &RawInput) -> Option<Argument<ID>>
    where ID: Debug + Clone {
    match raw_input.event {
        RawInputEvent::Key(ref keycode, _, _) => Some(Argument::KeyCode(keycode.clone())),
        _ => None
    }
}

fn get_modifiers<ID>(raw_input: &RawInput) -> Option<Argument<ID>>
    where ID: Debug + Clone {
    match raw_input.event {
        RawInputEvent::Key(_, _, modifiers) => Some(Argument::Modifiers(modifiers.into())),
        RawInputEvent::Button(_, _, _, modifiers) => Some(Argument::Modifiers(modifiers.into())),
        _ => None
    }
}

fn get_value<ID>(raw_input: &RawInput) -> Option<Argument<ID>>
    where ID: Debug + Clone {
    match raw_input.event {
        RawInputEvent::Char(ch) => Some(Argument::Value(ch)),
        _ => None
    }
}

fn get_action<ID>(raw_input: &RawInput) -> Option<Argument<ID>>
    where ID: Debug + Clone {
    match raw_input.event {
        RawInputEvent::Key(_, ref action, _) => Some(Argument::Action(action.clone().into())),
        RawInputEvent::Button(_, _, ref action, _) => Some(Argument::Action(action.clone().into())),
        _ => None
    }
}

fn get_cursor_position<ID>(raw_input: &RawInput) -> Option<Argument<ID>>
    where ID: Debug + Clone {
    match raw_input.event {
        RawInputEvent::Button(_, (x, y), _, _) => Some(Argument::CursorPosition(x, y)),
        _ => None
    }
}
