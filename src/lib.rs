extern crate ron;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate time;
extern crate winit;

#[macro_use]
extern crate log;

pub mod event;
pub mod types;
pub mod util;

mod mapping;

pub use event::*;
pub use types::{ActionArgument, ActionMetadata, Context, MappedType, StateInfo};

use types::{ActiveContext, StateStorage, WindowData};

use std::collections::HashMap;

use std::clone::Clone;
use std::cmp::Eq;
use std::fmt::Debug;
use std::hash::Hash;

use serde::de::DeserializeOwned;

pub struct InputRebinder<ACTION, ID>
where
    ACTION: Hash + Eq + Clone,
    ID: Hash + Eq + Clone + Debug,
{
    contexts: HashMap<ID, Context<ACTION, ID>>,
    active_contexts: Vec<ActiveContext<ID>>,
    state_storage: StateStorage<ACTION>,
    frame_data: WindowData,
}

impl<ACTION, ID> InputRebinder<ACTION, ID>
where
    ACTION: Hash + Eq + Clone + ActionMetadata + Debug + DeserializeOwned,
    ID: Hash + Eq + Clone + Debug + DeserializeOwned,
{
    pub fn new(size: (f64, f64)) -> InputRebinder<ACTION, ID> {
        InputRebinder {
            contexts: HashMap::default(),
            active_contexts: Vec::default(),
            state_storage: StateStorage::new(),
            frame_data: WindowData {
                size,
                cursor_position: None,
            },
        }
    }

    pub fn with_context(&mut self, mut context: Context<ACTION, ID>) -> &mut Self {
        context.sanitize();
        self.contexts.insert(context.id.clone(), context);
        self
    }

    pub fn with_contexts(&mut self, contexts: &mut Vec<Context<ACTION, ID>>) -> &mut Self {
        if contexts.len() == 0 {
            return self;
        }
        for c in contexts.drain(..) {
            self.with_context(c);
        }
        debug!("{:?}", self.contexts);
        self
    }

    pub fn activate_context(&mut self, context_id: &ID, priority: u32) {
        if let Some(_) = self.contexts.get(context_id) {
            let pos = self.active_contexts
                .binary_search_by(|p| priority.cmp(&p.priority))
                .unwrap_or_else(|pos| pos);
            self.active_contexts
                .insert(pos, ActiveContext::new(priority, context_id));
        }
        debug!("{:?}", self.active_contexts);
    }

    pub fn deactivate_context(&mut self, context_id: &ID) {
        if let Some(_) = self.contexts.get(context_id) {
            self.active_contexts
                .retain(|ac| ac.context_id != *context_id);
        }
        debug!("{:?}", self.active_contexts);
    }

    pub fn get_state_info(&self, state: &ACTION) -> Option<StateInfo> {
        self.state_storage.get(state)
    }

    pub fn is_state_active(&self, state: &ACTION) -> bool {
        self.state_storage.is_active(state)
    }

    fn process_window_input(&self, raw_input: &winit::Event) -> Option<Event<ACTION, ID>> {
        use winit::{Event as WEvent, WindowEvent};
        match *raw_input {
            WEvent::WindowEvent {
                event: WindowEvent::Resized(x, y),
                ..
            } => Some(Event::Resize(x, y)),

            WEvent::WindowEvent {
                event: WindowEvent::Focused(b),
                ..
            } => Some(Event::Focus(if b {
                FocusAction::Enter
            } else {
                FocusAction::Exit
            })),

            WEvent::WindowEvent {
                event: WindowEvent::Closed,
                ..
            } => Some(Event::Close),

            _ => None,
        }
    }

    fn process_controller_input(
        &mut self,
        raw_input: &winit::Event,
        next: &mut WindowData,
    ) -> Option<Event<ACTION, ID>> {
        let state_storage = &mut self.state_storage;
        let contexts = &self.contexts;
        self.active_contexts
            .iter()
            .filter_map(|ac| {
                contexts
                    .get(&ac.context_id)
                    .and_then(|c| c.process(raw_input, state_storage, next))
            })
            .next()
    }

    pub fn process(&mut self, raw_input: &Vec<winit::Event>) -> Vec<Event<ACTION, ID>> {
        if raw_input.len() <= 0 {
            return Vec::default();
        }
        let mut next = self.frame_data.clone();
        let mut window_input: Vec<Event<ACTION, ID>> = raw_input
            .iter()
            .filter_map(|ri| self.process_window_input(ri))
            .collect();
        let controller_input: Vec<Event<ACTION, ID>> = raw_input
            .iter()
            .filter_map(|ri| self.process_controller_input(ri, &mut next))
            .collect();
        window_input.extend(controller_input);
        self.frame_data = next;
        window_input
    }
}
