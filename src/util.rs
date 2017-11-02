use ron;
use serde::de::DeserializeOwned;
use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::iter::FromIterator;
use std::str;

use event::{ActionType, Argument, Event, StateAction};
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
        StateTracker {
            states: HashMap::default(),
        }
    }

    pub fn update<ID>(&mut self, events: &Vec<Event<ACTION, ID>>)
    where
        ID: Hash + Eq + Clone + Debug,
    {
        for e in events {
            if let Event::Controller(ref action, ActionType::State(ref state_action, _), _) = *e {
                self.states
                    .insert(action.clone(), action_as_bool(state_action));
            }
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
            text_action,
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
            if let Event::Controller(ref action, _, ref args) = *e {
                if *action == self.text_action {
                    for arg in args {
                        if let Argument::Value(char) = *arg {
                            self.current.push(char);
                        }
                    }
                }
            }
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
    contexts_from_reader(File::open(file).map_err(|_| BindingsError::FileNotFound)?)
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
    rdr.read_to_end(&mut bytes)
        .map_err(|_| BindingsError::ReadFailed)?;
    contexts_from_str(str::from_utf8(&bytes)
        .map_err(|_| BindingsError::Utf8Error)?)
}

pub fn contexts_from_str<ACTION, ID>(data: &str) -> Result<Vec<Context<ACTION, ID>>, BindingsError>
where
    ACTION: Hash + Eq + Clone + DeserializeOwned + ActionMetadata,
    ID: Clone + DeserializeOwned,
{
    let mut contexts: Vec<Context<ACTION, ID>> =
        ron::de::from_str(&data).map_err(|_| BindingsError::ParseError)?;
    for m in contexts.iter_mut().flat_map(|c| c.mappings.iter_mut()) {
        m.mapped_type = Some(m.action.mapped_type());
        m.action_args = m.action.args();
    }
    Ok(contexts)
}
