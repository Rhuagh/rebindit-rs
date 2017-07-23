pub use super::config::KeyCode;

use std::cmp::Ordering;
use std::collections::HashMap;
use std;

#[derive(Debug)]
pub enum DeviceType {
    Keyboard,
    Mouse,
    Window
}

#[derive(Debug)]
pub enum RawType {
    Button,
    Key,
    Motion,
    Char
}

#[derive(Debug)]
pub enum RawAction {
    Press,
    Release,
    Repeat
}

bitflags! {
    pub struct Modifiers: u32 {
        const SHIFT = 1 << 0;
        const CONTROL = 1 << 1;
        const ALT = 1 << 2;
        const SUPER = 1 << 3;
    }
}

pub type ButtonId = u32;

#[derive(Debug)]
pub enum Modifier {
    ALT,
    CONTROL,
    SHIFT,
    SUPER
}

#[derive(Debug)]
pub struct RawArgs {
    pub action : Option<RawAction>,
    pub keycode : Option<KeyCode>,
    pub button : Option<ButtonId>,
    pub modifier : Option<Modifier>
}

#[derive(Debug)]
pub enum ActionArgument {
    KeyCode,
    Value,
    Modifiers,
    Action,
    CursorPosition,
    ContextId
}

#[derive(Debug, Clone)]
pub enum MappedType {
    Action,
    State,
    Range
}

pub trait ActionMetadata {
    fn mapped_type(&self) -> MappedType;
    fn args(&self) -> Vec<ActionArgument>;
}

#[derive(Debug)]
pub struct Mapping<C> {
    pub mapped_type : Option<MappedType>,
    pub raw_type : RawType,
    pub raw_args : RawArgs,
    pub action : Option<C>,
    pub action_args : Vec<ActionArgument>,
}

impl <C : std::hash::Hash + std::cmp::Eq + ActionMetadata> Mapping<C> {
    pub fn new(raw_type : RawType,
               raw_args : RawArgs,
               action : Option<C>) -> Self {
        match action {
            Some(action) => {
                Mapping {
                    raw_type : raw_type,
                    raw_args : raw_args,
                    mapped_type : action.mapped_type(),
                    action_args : action.args(),
                    action : Some(action),
                }
            },
            None => {
                Mapping {
                    raw_type : raw_type,
                    raw_args : raw_args,
                    action : None,
                    mapped_type : None,
                    action_args : Vec::default()
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct StateInfo {
    pub active : bool,
    pub start_time : f64,
    pub stop_time : f64
}

#[derive(Debug)]
pub struct Context<C : std::hash::Hash + std::cmp::Eq> {
    pub id : String,
    pub mappings : Vec<Mapping<C>>,
    pub state_storage : HashMap<C, StateInfo>
}

impl <C : std::hash::Hash + std::cmp::Eq> Context<C> {
    pub fn new(id: String, mappings : Vec<Mapping<C>>) -> Self {
        Context {
            id : id,
            mappings : mappings,
            state_storage : HashMap::default()
        }
    }

    pub fn with_mapping(self, mapping: Mapping<C>) -> Self {
        self.mappings.push(mapping);
        self
    }

    pub fn with_mappings(self, mappings: Vec<Mapping<C>>) -> Self {
        self.mappings.append(mappings);
        self
    }
}

#[derive(Debug, Eq, Clone)]
pub struct ActiveContext {
    pub priority : u32,
    pub index : usize
}

pub type WindowPosition = (f64, f64);
pub type WindowSize = (u32, u32);

impl ActiveContext {
    pub fn new(priority: u32, index: usize) -> ActiveContext {
        ActiveContext {
            priority : priority,
            index : index
        }
    }
}

impl PartialEq for ActiveContext {
    fn eq(&self, other: &ActiveContext) -> bool {
        self.priority == other.priority
    }
}

impl Ord for ActiveContext {
    fn cmp(&self, other: &ActiveContext) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for ActiveContext {
    fn partial_cmp(&self, other: &ActiveContext) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
