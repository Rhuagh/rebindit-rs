#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate ron;

extern crate time;
extern crate winit;

#[macro_use]
extern crate log;

pub mod event;
pub mod types;
pub mod raw;
pub mod mapping;
pub mod util;
pub mod winit_mapper;

pub use event::*;
pub use types::{ActionMetadata, ActionArgument, MappedType, Context, StateStorage, StateInfo};
pub use raw::RawInput;

use types::{ActiveContext, WindowData};
use raw::RawInputEvent;

use std::collections::HashMap;

use std::hash::Hash;
use std::cmp::Eq;
use std::clone::Clone;
use std::fmt::Debug;

use serde::de::DeserializeOwned;

pub struct InputReMapper<ACTION, ID>
    where ACTION: Hash + Eq + Clone,
          ID: Hash + Eq + Clone + Debug {
    contexts : HashMap<ID, Context<ACTION, ID>>,
    active_contexts : Vec<ActiveContext<ID>>,
    state_storage : StateStorage<ACTION>,
    frame_data: WindowData,
}

impl<ACTION, ID> InputReMapper<ACTION, ID>
    where ACTION: Hash + Eq + Clone + ActionMetadata + Debug + DeserializeOwned,
          ID: Hash + Eq + Clone + Debug + DeserializeOwned {
    pub fn new(size : (f64, f64)) -> InputReMapper<ACTION, ID> {
        InputReMapper {
            contexts : HashMap::default(),
            active_contexts : Vec::default(),
            state_storage : StateStorage::new(),
            frame_data : WindowData {
                size : size,
                cursor_position : None
            },
        }
    }

    pub fn with_context(&mut self, mut context : Context<ACTION, ID>) -> &mut Self {
        context.sanitize();
        self.contexts.insert(context.id.clone(), context);
        self
    }

    pub fn with_contexts(&mut self, contexts : &mut Vec<Context<ACTION, ID>>) -> &mut Self {
        if contexts.len() == 0 {
            return self;
        }
        for c in contexts.drain(..) {
            self.with_context(c);
        }
        debug!("{:?}", self.contexts);
        self
    }

    pub fn activate_context(&mut self, context_id : &ID, priority: u32) {
        match self.contexts.get(context_id) {
            Some(_) => self.active_contexts.push(ActiveContext::new(priority, context_id)),
            None => ()
        };
        self.active_contexts.sort();
        debug!("{:?}", self.active_contexts);
    }

    pub fn toggle_context(&mut self, context_id: &ID, priority: u32) {
        match self.contexts.get(context_id) {
            Some(_) => {
                match self.active_contexts.iter().position(|ac| ac.context_id == *context_id) {
                    Some(ac_index) => {
                        self.active_contexts.remove(ac_index);
                        ()
                    },
                    None => {
                        self.active_contexts.push(ActiveContext::new(priority, context_id));
                        self.active_contexts.sort();
                    }
                };
            },
            None => ()
        };
        debug!("{:?}", self.active_contexts);
    }

    pub fn deactivate_context(&mut self, context_id : &ID) {
        match self.contexts.get(context_id) {
            Some(_) => {
                match self.active_contexts.iter().position(|ac| ac.context_id == *context_id) {
                    Some(ac_index) => {
                        self.active_contexts.remove(ac_index);
                        ()
                    },
                    None => ()
                };
            },
            None => ()
        };
        debug!("{:?}", self.active_contexts);
    }

    pub fn get_state_info(&self, state: &ACTION) -> Option<StateInfo> {
        self.state_storage.get(state)
    }

    pub fn is_state_active(&self, state: &ACTION) -> bool {
        self.state_storage.is_active(state)
    }

    fn process_window_input(&self, raw_input : &RawInput) -> Option<WindowEvent> {
        match raw_input.event {
            RawInputEvent::Resize(x, y) => Some(WindowEvent::Resize(x, y)),
            RawInputEvent::Focus(b) => Some(WindowEvent::Focus(
                if b { FocusAction::Enter } else { FocusAction::Exit })),
            RawInputEvent::Close => Some(WindowEvent::Close),
            _ => None
        }
    }

    fn process_controller_input(&mut self, raw_input: &RawInput) -> Option<ControllerEvent<ACTION, ID>> {
        for ref active_context in &self.active_contexts {
            match self.contexts
                      .get_mut(&active_context.context_id)
                      .unwrap()
                      .process(raw_input,
                               &mut self.state_storage) {
                Some(v) => return Some(v),
                None => ()
            }
        }
        None
    }

    pub fn process_raw_input(&mut self, raw_input: &Vec<RawInput>) -> Vec<Event<ACTION, ID>> {
        let mut window_input : Vec<Event<ACTION, ID>> = raw_input.iter()
            .filter_map(|ri| self.process_window_input(ri))
            .map(|wi| Event::Window(wi))
            .collect();
        let controller_input : Vec<Event<ACTION, ID>> = raw_input.iter()
            .filter_map(|ri| { self.process_controller_input(ri) } )
            .map(|ci| Event::Controller(ci))
            .collect();
        window_input.extend(controller_input);
        window_input
    }

    pub fn process(&mut self, events: &Vec<winit::Event>) -> Vec<Event<ACTION, ID>> {
        let mut next = self.frame_data.clone();
        let raw = events.iter().flat_map(|e| winit_mapper::process_event(e, &mut next)).collect();
        self.frame_data = next;
        self.process_raw_input(&raw)
    }
}
