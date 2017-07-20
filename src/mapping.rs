use super::raw;
use super::types;
use super::event;
use std;

impl <C : std::cmp::Eq + std::hash::Hash> types::Context<C> where C : std::fmt::Debug + std::clone::Clone {

    pub fn process(&mut self, raw_input : &raw::RawInput) -> Option<event::ControllerEvent<C>> {
        let event = self.process_internal(raw_input);
        match &event {
            &Some(event::ControllerEvent::State(ref c_action, ref state_action, _, _)) => {
                self.update_state_info(c_action, state_action, raw_input);
            },
            _ => ()
        };
        event
    }

    fn process_internal(&self, raw_input : &raw::RawInput) -> Option<event::ControllerEvent<C>> {
        for m in &self.mappings {
            if check_mapping(m, raw_input) {
                match m.mapped_type {
                    Some(types::MappedType::Action) => return Some(as_action(&m.mapped, raw_input)),
                    Some(types::MappedType::State) => {
                        return Some(self.as_state(&m.mapped, raw_input));
                    },
                    Some(types::MappedType::Range) => return Some(as_range(&m.mapped, raw_input)),
                    _ => ()
                }
            }
        }
        None
    }

    fn as_state(&self, mapped: &types::Mapped<C>, raw_input : &raw::RawInput) -> event::ControllerEvent<C> {
        let r_action = match raw_input.event {
            raw::RawInputEvent::Key(_, ref action, _) => Some(action.clone()),
            raw::RawInputEvent::Button(_, _, ref action, _) => Some(action.clone()),
            _ => None
        };
        let c_action = mapped.action.clone().unwrap();
        event::ControllerEvent::State(c_action.clone(),
                                      state_action(&r_action.unwrap()),
                                      self.state_duration(&c_action, raw_input),
                                      arguments(&mapped.args, raw_input))
    }

    fn state_duration(&self, c_action: &C, raw_input : &raw::RawInput) -> event::StateDuration {
        match self.state_storage.get(c_action) {
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

    fn update_state_info(&mut self, c_action: &C, state_action: &event::StateAction, raw_input : &raw::RawInput) {
        match state_action {
            &event::StateAction::Active => {
                let add = match self.state_storage.get_mut(c_action) {
                    Some(ref mut info) => {
                        if !info.active {
                            info.active = true;
                            info.start_time = raw_input.time;
                            info.stop_time = 0.0;
                        }
                        None
                    },
                    None => Some(types::StateInfo {
                        active : true,
                        start_time : raw_input.time,
                        stop_time : 0.0
                    })
                };
                match add {
                    Some(info) => self.state_storage.insert(c_action.clone(), info),
                    None => None
                };
            },
            &event::StateAction::Deactivated => {
                match self.state_storage.get_mut(c_action) {
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

fn state_action(raw_input : &raw::RawInputAction) -> event::StateAction {
    match raw_input {
        &raw::RawInputAction::Press => event::StateAction::Active,
        &raw::RawInputAction::Repeat => event::StateAction::Active,
        &raw::RawInputAction::Release => event::StateAction::Deactivated
    }
}

fn range_diff(raw_input : &raw::RawInput) -> event::RangeDiff {
    match raw_input.event {
        raw::RawInputEvent::Motion(x, y) => (x, y),
        _ => (0.0, 0.0)
    }
}

fn arguments(args : &Vec<types::ActionArgument>, raw_input : &raw::RawInput) -> Vec<event::Argument> {
    args.iter().filter_map(|arg| {
        match arg {
            &types::ActionArgument::KeyCode => get_keycode(raw_input),
            &types::ActionArgument::Value => get_value(raw_input),
            &types::ActionArgument::Modifiers => get_modifiers(raw_input),
            &types::ActionArgument::Action => get_action(raw_input),
            &types::ActionArgument::CursorPosition => get_cursor_position(raw_input),
        }
    }).collect()
}

fn as_action<C>(mapped: &types::Mapped<C>, raw_input : &raw::RawInput) -> event::ControllerEvent<C>
    where C : std::fmt::Debug + std::clone::Clone {
    event::ControllerEvent::Action(mapped.action.clone().unwrap(),
                                   arguments(&mapped.args, raw_input))
}

fn as_range<C>(mapped: &types::Mapped<C>, raw_input: &raw::RawInput) -> event::ControllerEvent<C>
    where C : std::fmt::Debug + std::clone::Clone {
    event::ControllerEvent::Range(mapped.action.clone().unwrap(),
                                  range_diff(raw_input),
                                  arguments(&mapped.args, raw_input))
}

fn check_mapping<C>(mapping : &types::Mapping<C>, raw_input : &raw::RawInput) -> bool {
    match mapping.raw.raw_type {
        types::RawType::Button => check_button(&mapping.raw.raw_args, raw_input),
        types::RawType::Key => check_key(&mapping.raw.raw_args, raw_input),
        types::RawType::Motion => check_motion(&mapping.raw.raw_args, raw_input),
        types::RawType::Char => check_char(&mapping.raw.raw_args, raw_input),
    }
}

fn check_button(raw_args : &types::RawArgs, raw_input: &raw::RawInput) -> bool {
    match raw_input.event {
        raw::RawInputEvent::Button(ref button_id, _, ref action, ref modifiers) => {
            check_button_id(&raw_args.button, button_id)
                && check_action(&raw_args.action, action)
                && check_modifiers(&raw_args.modifier, modifiers)
        },
        _ => false
    }
}

fn check_key(raw_args : &types::RawArgs, raw_input: &raw::RawInput) -> bool {
    match raw_input.event {
        raw::RawInputEvent::Key(ref keycode, ref action, ref modifiers) => {
            check_keycode(&raw_args.keycode, keycode)
                && check_action(&raw_args.action, action)
                && check_modifiers(&raw_args.modifier, modifiers)
        },
        _ => false
    }
}

#[allow(unused_variables)]
fn check_motion(raw_args : &types::RawArgs, raw_input: &raw::RawInput) -> bool {
    match raw_input.event {
        raw::RawInputEvent::Motion(_, _) => true,
        _ => false
    }
}

#[allow(unused_variables)]
fn check_char(raw_args : &types::RawArgs, raw_input: &raw::RawInput) -> bool {
    match raw_input.event {
        raw::RawInputEvent::Char(_) => true,
        _ => false
    }
}

fn check_button_id(config_button: &Option<types::ButtonId>, raw_button_id : &u32) -> bool {
    match config_button {
        &Some(button_id) => button_id == *raw_button_id,
        &None => true
    }
}

fn check_keycode(config_keycode: &Option<types::KeyCode>, raw_keycode: &types::KeyCode) -> bool {
    match config_keycode {
        &Some(ref c_keycode) => *c_keycode == *raw_keycode,
        &None => true
    }
}

fn check_action(config_action: &Option<types::RawAction>, raw_action: &raw::RawInputAction) -> bool {
    match config_action {
        &Some(ref c_action) => {
            match c_action {
                &types::RawAction::Press => *raw_action == raw::RawInputAction::Press,
                &types::RawAction::Repeat => *raw_action == raw::RawInputAction::Repeat,
                &types::RawAction::Release => *raw_action == raw::RawInputAction::Release,
            }
        },
        &None => true
    }
}

fn check_modifiers(config_modifier: &Option<types::Modifier>, raw_modifiers: &raw::RawInputModifiers) -> bool {
    match config_modifier {
        &Some(types::Modifier::SHIFT) => *raw_modifiers == raw::SHIFT,
        &Some(types::Modifier::ALT) => *raw_modifiers == raw::ALT,
        &Some(types::Modifier::CONTROL) => *raw_modifiers == raw::CONTROL,
        &Some(types::Modifier::SUPER) => *raw_modifiers == raw::SUPER,
        &None => true
    }
}

fn get_keycode(raw_input : &raw::RawInput) -> Option<event::Argument> {
    match raw_input.event {
        raw::RawInputEvent::Key(ref keycode, _, _) => Some(event::Argument::KeyCode(keycode.clone())),
        _ => None
    }
}

fn get_modifiers(raw_input: &raw::RawInput) -> Option<event::Argument> {
    match raw_input.event {
        raw::RawInputEvent::Key(_, _, modifiers) => Some(event::Argument::Modifiers(modifiers.into())),
        raw::RawInputEvent::Button(_, _, _, modifiers) => Some(event::Argument::Modifiers(modifiers.into())),
        _ => None
    }
}

fn get_value(raw_input: &raw::RawInput) -> Option<event::Argument> {
    match raw_input.event {
        raw::RawInputEvent::Char(ch) => Some(event::Argument::Value(ch)),
        _ => None
    }
}

fn get_action(raw_input: &raw::RawInput) -> Option<event::Argument> {
    match raw_input.event {
        raw::RawInputEvent::Key(_, ref action, _) => Some(event::Argument::Action(action.clone().into())),
        raw::RawInputEvent::Button(_, _, ref action, _) => Some(event::Argument::Action(action.clone().into())),
        _ => None
    }
}

fn get_cursor_position(raw_input: &raw::RawInput) -> Option<event::Argument> {
    match raw_input.event {
        raw::RawInputEvent::Button(_, (x, y), _, _) => Some(event::Argument::CursorPosition(x, y)),
        _ => None
    }
}
