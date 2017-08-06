use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::Debug;
use std::clone::Clone;
use std::iter::FromIterator;

use event::{Event, ControllerEvent, StateAction, Argument};

pub struct StateTracker<ACTION>
    where ACTION: Hash + Eq + Clone {
    states: HashMap<ACTION, bool>
}

impl<ACTION> StateTracker<ACTION>
    where ACTION: Hash + Eq + Clone + Debug {
    pub fn new() -> StateTracker<ACTION> {
        StateTracker {
            states: HashMap::default()
        }
    }

    pub fn update<ID>(&mut self, events: &Vec<Event<ACTION, ID>>)
        where ID: Hash + Eq + Clone + Debug {
        for e in events {
            match *e {
                Event::Controller(ControllerEvent::State(ref state, ref action, _, _)) => {
                    self.states.insert(state.clone(), action_as_bool(action));
                }
                _ => ()
            };
        }
    }

    pub fn is_active(&self, state: &ACTION) -> bool {
        match self.states.get(state) {
            Some(active) => *active,
            None => false
        }
    }
}

pub fn action_as_bool(action: &StateAction) -> bool {
    match *action {
        StateAction::Activated | StateAction::Active => true,
        StateAction::Deactivated => false
    }
}

pub struct TextHandler<ACTION: Hash + Eq + Clone> {
    current: Vec<char>,
    text_action: ACTION
}

impl<ACTION: Hash + Eq + Clone + Debug> TextHandler<ACTION> {
    pub fn new(text_action: ACTION) -> TextHandler<ACTION> {
        TextHandler {
            current: Vec::default(),
            text_action: text_action
        }
    }

    pub fn consume(&mut self) -> String {
        String::from_iter(self.current.drain(..))
    }

    pub fn update<ID>(&mut self,
                      events: &Vec<Event<ACTION, ID>>)
        where ID: Hash + Eq + Clone + Debug {
        for e in events {
            match *e {
                Event::Controller(ControllerEvent::Action(ref action, ref args)) => {
                    if *action == self.text_action {
                        for arg in args {
                            match *arg {
                                Argument::Value(char) => self.current.push(char),
                                _ => ()
                            }
                        }
                    }
                }
                _ => ()
            };
        }
    }
}
