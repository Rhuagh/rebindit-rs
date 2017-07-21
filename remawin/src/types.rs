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
pub struct Raw {
    pub raw_type : RawType,
    pub raw_args : RawArgs
}

#[derive(Debug)]
pub enum ActionArgument {
    KeyCode,
    Value,
    Modifiers,
    Action,
    CursorPosition
}

#[derive(Debug, Clone)]
pub enum MappedType {
    Action,
    State,
    Range
}

pub trait ToMappedType {
    fn to_mapped_type(&self) -> MappedType;
}

#[derive(Debug)]
pub struct Mapped<C> {
    pub action : Option<C>,
    pub args : Vec<ActionArgument>
}

#[derive(Debug)]
pub struct Mapping<C> {
    pub mapped_type : Option<MappedType>,
    pub raw : Raw,
    pub mapped : Mapped<C>
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
