extern crate remawin;

use std::collections::HashMap;

use remawin::event::{Event, ControllerEvent, StateAction};

pub struct RemawinStateTracker<C>
    where C : std::hash::Hash + std::cmp::Eq + std::str::FromStr + std::clone::Clone + std::fmt::Debug {
    states : HashMap<C, bool>
}

impl <C> RemawinStateTracker<C>
    where C : std::hash::Hash + std::cmp::Eq + std::str::FromStr + std::clone::Clone + std::fmt::Debug {

    pub fn new() -> RemawinStateTracker<C> {
        RemawinStateTracker {
            states : HashMap::default()
        }
    }

    pub fn update<I>(&mut self, events : &Vec<Event<C, I>>)
        where I : std::fmt::Debug + std::clone::Clone + std::hash::Hash + std::cmp::Eq {
        for e in events {
            match *e {
                Event::Controller(ControllerEvent::State(ref state, ref action, _, _)) => {
                    self.states.insert(state.clone(), action_as_bool(action));
                },
                _ => ()
            };
        }
    }

    pub fn is_active(&self, state: &C) -> bool {
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
