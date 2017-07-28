#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate time;

pub mod config;
pub mod event;
pub mod types;
pub mod raw;
pub mod mapping;

pub use event::*;
pub use types::{ActionMetadata, ActionArgument, MappedType};

use std::collections::HashMap;

pub struct InputReMapper<C, I>
    where C : std::hash::Hash + std::cmp::Eq + std::str::FromStr + std::clone::Clone,
          I : std::hash::Hash + std::cmp::Eq + std::str::FromStr +
              std::clone::Clone + std::fmt::Debug {
    contexts : HashMap<I, types::Context<C, I>>,
    active_contexts : Vec<types::ActiveContext<I>>
}

impl<C, I> InputReMapper<C, I>
    where C : std::hash::Hash + std::cmp::Eq + std::str::FromStr +
              std::fmt::Debug + std::clone::Clone + types::ActionMetadata,
          I : std::hash::Hash + std::cmp::Eq + std::str::FromStr +
              std::fmt::Debug + std::clone::Clone {
    pub fn new() -> InputReMapper<C, I> {
        InputReMapper {
            contexts : HashMap::default(),
            active_contexts : Vec::default()
        }
    }

    pub fn with_context(&mut self, context : types::Context<C, I>) -> &mut Self {
        self.contexts.insert(context.id.clone().unwrap(), context);
        self
    }

    pub fn with_contexts(&mut self, contexts : &mut Vec<types::Context<C, I>>) -> &mut Self {
        for c in contexts {
            self.contexts.insert(c.id.clone().unwrap(), c.clone());
        }
        println!("{:?}", self.contexts);
        self
    }

    pub fn with_bindings_file(&mut self, file: &str) -> &mut Self {
        let f = std::fs::File::open(file).expect("Failed opening bindings config file");
        let bindings : config::ConfigBindings = serde_yaml::from_reader(f).expect("Failed parsing Yaml string");
        let mut contexts : Vec<types::Context<C, I>> = bindings.into();
        for c in &mut contexts {
            c.mappings.retain(|m| m.action.is_some() && m.mapped_type.is_some());
        }
        contexts.retain(|c| c.id.is_some());
        self.with_contexts(&mut contexts)
    }

    pub fn activate_context(&mut self, context_id : &I, priority: u32) {
        match self.contexts.get(context_id) {
            Some(_) => self.active_contexts.push(types::ActiveContext::new(priority, context_id)),
            None => ()
        };
        self.active_contexts.sort();
        println!("{:?}", self.active_contexts);
    }

    pub fn deactivate_context(&mut self, context_id : &I) {
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
    }

    fn process_window_input(&self, raw_input : &raw::RawInput) -> Option<WindowEvent> {
        match raw_input.event {
            raw::RawInputEvent::Resize(x, y) => Some(WindowEvent::Resize(x, y)),
            raw::RawInputEvent::Focus(b) => Some(WindowEvent::Focus(
                if b { FocusAction::Enter } else { FocusAction::Exit })),
            raw::RawInputEvent::Close => Some(WindowEvent::Close),
            _ => None
        }
    }

    fn process_controller_input(&mut self, raw_input: &raw::RawInput) -> Option<ControllerEvent<C, I>> {
        for ref active_context in &self.active_contexts {
            match self.contexts.get_mut(&active_context.context_id).unwrap().process(raw_input) {
                Some(v) => return Some(v),
                None => ()
            }
        }
        None
    }

    pub fn process_raw_input(&mut self, raw_input: &Vec<raw::RawInput>) -> Vec<Event<C, I>> {
        let mut window_input : Vec<Event<C, I>> = raw_input.iter()
            .filter_map(|ri| self.process_window_input(ri))
            .map(|wi| Event::Window(wi))
            .collect();
        let controller_input : Vec<Event<C, I>> = raw_input.iter()
            .filter_map(|ri| { self.process_controller_input(ri) } )
            .map(|ci| Event::Controller(ci))
            .collect();
        window_input.extend(controller_input);
        window_input
    }
}
