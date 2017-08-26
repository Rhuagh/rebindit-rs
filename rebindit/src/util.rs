use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::Debug;
use std::clone::Clone;
use std::iter::FromIterator;
use serde::de::DeserializeOwned;
use ron;
use std::io::Read;
use std::str;
use std::fs::File;

use event::{Event, ControllerEvent, StateAction, Argument};
use types::*;

pub struct StateTracker<ACTION>
where
    ACTION: Hash + Eq + Clone,
{
    states: HashMap<ACTION, bool>,
}

impl<ACTION> StateTracker<ACTION>
where
    ACTION: Hash + Eq + Clone + Debug,
{
    pub fn new() -> StateTracker<ACTION> {
        StateTracker { states: HashMap::default() }
    }

    pub fn update<ID>(&mut self, events: &Vec<Event<ACTION, ID>>)
    where
        ID: Hash + Eq + Clone + Debug,
    {
        for e in events {
            match *e {
                Event::Controller(ControllerEvent::State(ref state, ref action, _, _)) => {
                    self.states.insert(state.clone(), action_as_bool(action));
                }
                _ => (),
            };
        }
    }

    pub fn is_active(&self, state: &ACTION) -> bool {
        match self.states.get(state) {
            Some(active) => *active,
            None => false,
        }
    }
}

pub fn action_as_bool(action: &StateAction) -> bool {
    match *action {
        StateAction::Activated | StateAction::Active => true,
        StateAction::Deactivated => false,
    }
}

pub struct TextHandler<ACTION: Hash + Eq + Clone> {
    current: Vec<char>,
    text_action: ACTION,
}

impl<ACTION: Hash + Eq + Clone + Debug> TextHandler<ACTION> {
    pub fn new(text_action: ACTION) -> TextHandler<ACTION> {
        TextHandler {
            current: Vec::default(),
            text_action: text_action,
        }
    }

    pub fn consume(&mut self) -> String {
        String::from_iter(self.current.drain(..))
    }

    pub fn update<ID>(&mut self, events: &Vec<Event<ACTION, ID>>)
    where
        ID: Hash + Eq + Clone + Debug,
    {
        for e in events {
            match *e {
                Event::Controller(ControllerEvent::Action(ref action, ref args)) => {
                    if *action == self.text_action {
                        for arg in args {
                            match *arg {
                                Argument::Value(char) => self.current.push(char),
                                _ => (),
                            }
                        }
                    }
                }
                _ => (),
            };
        }
    }
}

#[derive(Debug)]
pub enum BindingsError {
    FileNotFound,
    ReadFailed,
    Utf8Error,
    ParseError,
}

pub fn contexts_from_file<ACTION, ID>(file: &str) -> Result<Vec<Context<ACTION, ID>>, BindingsError>
where
    ACTION: Hash + Eq + Clone + DeserializeOwned + ActionMetadata,
    ID: Clone + DeserializeOwned,
{
    let f = match File::open(file) {
        Ok(f) => f,
        Err(_) => return Err(BindingsError::FileNotFound),
    };
    contexts_from_reader(f)
}

pub fn contexts_from_reader<R, ACTION, ID>(
    mut rdr: R,
) -> Result<Vec<Context<ACTION, ID>>, BindingsError>
where
    R: Read,
    ACTION: Hash + Eq + Clone + DeserializeOwned + ActionMetadata,
    ID: Clone + DeserializeOwned,
{
    let mut bytes = Vec::new();
    match rdr.read_to_end(&mut bytes) {
        Err(_) => return Err(BindingsError::ReadFailed),
        _ => (),
    };
    let s = match str::from_utf8(&bytes) {
        Ok(s) => s,
        Err(_) => return Err(BindingsError::Utf8Error),
    };
    contexts_from_str(s)
}

pub fn contexts_from_str<ACTION, ID>(data: &str) -> Result<Vec<Context<ACTION, ID>>, BindingsError>
where
    ACTION: Hash + Eq + Clone + DeserializeOwned + ActionMetadata,
    ID: Clone + DeserializeOwned,
{
    let mut contexts: Vec<Context<ACTION, ID>> = match ron::de::from_str(&data) {
        Ok(c) => c,
        Err(_) => return Err(BindingsError::ParseError),
    };
    for c in &mut contexts {
        for m in &mut c.mappings {
            let action = m.action.clone();
            m.mapped_type = Some(action.mapped_type());
            m.action_args = action.args();
        }
    }
    Ok(contexts)
}
