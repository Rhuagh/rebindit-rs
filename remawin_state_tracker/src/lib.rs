extern crate remawin;

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::Debug;
use std::clone::Clone;

use remawin::event::{Event, ControllerEvent, StateAction};

pub struct RemawinStateTracker<ACTION>
    where ACTION: Hash + Eq + Clone {
    states : HashMap<ACTION, bool>
}

impl <ACTION> RemawinStateTracker<ACTION>
    where ACTION: Hash + Eq + Clone + Debug {

    pub fn new() -> RemawinStateTracker<ACTION> {
        RemawinStateTracker {
            states : HashMap::default()
        }
    }

    pub fn update<ID>(&mut self, events : &Vec<Event<ACTION, ID>>)
        where ID: Hash + Eq + Clone + Debug {
        for e in events {
            match *e {
                Event::Controller(ControllerEvent::State(ref state, ref action, _, _)) => {
                    self.states.insert(state.clone(), action_as_bool(action));
                },
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

pub fn action_as_bool(action : &StateAction) -> bool {
    match *action {
        StateAction::Activated | StateAction::Active => true,
        StateAction::Deactivated => false
    }
}
