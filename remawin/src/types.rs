use std::cmp::Ordering;
use std::collections::HashMap;

use std::hash::Hash;
use std::cmp::Eq;
use std::clone::Clone;
use std::fmt::Debug;
use std::io::Read;
use std::str;
use std::fs::File;

use serde::de::DeserializeOwned;

use ron;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum KeyCode {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    Snapshot,
    Scroll,
    Pause,
    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,
    Left,
    Up,
    Right,
    Down,
    Back,
    Return,
    Space,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Compose,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LMenu,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RMenu,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    None,
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Keyboard,
    Mouse,
    Window
}

#[derive(Debug, Clone, Deserialize)]
pub enum RawType {
    Button,
    Key,
    Motion,
    Char
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub enum Modifier {
    ALT,
    CONTROL,
    SHIFT,
    SUPER
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawArgs<ACTION : Clone> {
    pub action : Option<RawAction>,
    pub keycode : Option<KeyCode>,
    pub button : Option<ButtonId>,
    pub modifier : Option<Modifier>,
    pub state_active : Option<ACTION>
}

impl <ACTION: Clone> RawArgs<ACTION> {

    pub fn new() -> RawArgs<ACTION> {
        RawArgs {
            action : None,
            keycode : None,
            button : None,
            modifier : None,
            state_active : None,
        }
    }

    pub fn with_action(mut self, action: RawAction) -> Self {
        self.action = Some(action);
        self
    }

    pub fn with_keycode(mut self, keycode: KeyCode) -> Self {
        self.keycode = Some(keycode);
        self
    }

    pub fn with_button(mut self, button: ButtonId) -> Self {
        self.button = Some(button);
        self
    }

    pub fn with_modifier(mut self, modifier: Modifier) -> Self {
        self.modifier = Some(modifier);
        self
    }

    pub fn with_state_active(mut self, state: ACTION) -> Self {
        self.state_active = Some(state);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum ActionArgument {
    KeyCode,
    Value,
    Modifiers,
    Action,
    CursorPosition,
    ContextId
}

#[derive(Debug, Clone, Deserialize)]
pub enum MappedType {
    Action,
    State,
    Range
}

pub trait ActionMetadata {
    fn mapped_type(&self) -> MappedType;
    fn args(&self) -> Vec<ActionArgument>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mapping<ACTION: Clone> {
    pub raw_type : RawType,
    pub raw_args : RawArgs<ACTION>,
    pub action : ACTION,

    #[serde(default = "default_mt")]
    pub mapped_type : Option<MappedType>,
    #[serde(default = "default_aa")]
    pub action_args : Vec<ActionArgument>,
}

fn default_mt() -> Option<MappedType> {
    None
}

fn default_aa() -> Vec<ActionArgument> {
    Vec::default()
}

impl <ACTION: ActionMetadata + Clone> Mapping<ACTION> {
    pub fn new(raw_type : RawType,
               raw_args : RawArgs<ACTION>,
               action: ACTION) -> Self {
        Mapping {
            raw_type: raw_type,
            raw_args: raw_args,
            mapped_type: Some(action.mapped_type()),
            action_args: action.args(),
            action: action,
        }
    }
}

impl <ACTION: Clone> Mapping<ACTION> {
    pub fn sanitize(&mut self) {
        match self.mapped_type {
            Some(MappedType::Action) => {
                if self.raw_args.action == None {
                    self.raw_args.action = Some(RawAction::Release);
                }
            },
            Some(MappedType::State) => {
                if self.raw_args.action != None {
                    self.raw_args.action = None;
                }
            },
            _ => ()
        }
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct Context<ACTION, ID>
    where ACTION: Hash + Eq + Clone,
          ID: Clone {
    pub id : ID,
    pub mappings : Vec<Mapping<ACTION>>,
}

impl <ACTION, ID> Context<ACTION, ID>
    where ACTION: Hash + Eq + Clone + ActionMetadata,
          ID: Clone {

    pub fn new(id: ID) -> Self {
        Self::new_with_mappings(id, Vec::default())
    }

    pub fn new_with_mappings(id: ID, mappings : Vec<Mapping<ACTION>>) -> Self {
        Context {
            id : id,
            mappings : mappings
        }
    }

    pub fn with_mapping(mut self,
                        raw_type: RawType,
                        raw_args : RawArgs<ACTION>,
                        action: ACTION) -> Self {
        self.mappings.push(Mapping::new(raw_type, raw_args, action));
        self
    }

    pub fn with_mappings(mut self, mut mappings: Vec<Mapping<ACTION>>) -> Self {
        self.mappings.append(&mut mappings);
        self
    }

    pub fn sanitize(&mut self) {
        for m in &mut self.mappings {
            m.sanitize();
        }
    }
}

#[derive(Debug)]
pub enum BindingsError {
    FileNotFound,
    ReadFailed,
    Utf8Error,
    ParseError
}

pub fn contexts_from_file<ACTION, ID>(file : &str) -> Result<Vec<Context<ACTION, ID>>, BindingsError>
    where ACTION : Hash + Eq + Clone + DeserializeOwned + ActionMetadata,
          ID : Clone + DeserializeOwned {
    let f = match File::open(file) {
        Ok(f) => f,
        Err(_) => return Err(BindingsError::FileNotFound)
    };
    contexts_from_reader(f)
}

pub fn contexts_from_reader<R, ACTION, ID>(mut rdr: R) -> Result<Vec<Context<ACTION, ID>>, BindingsError>
    where R : Read,
          ACTION : Hash + Eq + Clone + DeserializeOwned + ActionMetadata,
          ID : Clone + DeserializeOwned {
    let mut bytes = Vec::new();
    match rdr.read_to_end(&mut bytes) {
        Err(_) => return Err(BindingsError::ReadFailed),
        _ => ()
    };
    let s = match str::from_utf8(&bytes) {
        Ok(s) => s,
        Err(_) => return Err(BindingsError::Utf8Error)
    };
    contexts_from_str(s)
}

pub fn contexts_from_str<ACTION, ID>(data: &str) -> Result<Vec<Context<ACTION, ID>>, BindingsError>
    where ACTION : Hash + Eq + Clone + DeserializeOwned + ActionMetadata,
          ID : Clone + DeserializeOwned {
    let mut contexts : Vec<Context<ACTION, ID>> =
        match ron::de::from_str(&data) {
            Ok(c) => c,
            Err(_) => return Err(BindingsError::ParseError)
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

#[derive(Debug, Eq, Clone)]
pub struct ActiveContext<ID>
    where ID: Debug + Clone {
    pub priority : u32,
    pub context_id : ID
}

pub type WindowPosition = (f64, f64);
pub type WindowSize = (u32, u32);

impl <ID> ActiveContext<ID>
    where ID: Debug + Clone {
    pub fn new(priority: u32, context_id: &ID) -> ActiveContext<ID> {
        ActiveContext {
            priority : priority,
            context_id : context_id.clone()
        }
    }
}

impl <ID> PartialEq for ActiveContext<ID>
    where ID: Debug + Clone {
    fn eq(&self, other: &ActiveContext<ID>) -> bool {
        self.priority == other.priority
    }
}

impl <ID> Ord for ActiveContext<ID>
    where ID: Debug + Clone + Eq {
    fn cmp(&self, other: &ActiveContext<ID>) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl <ID> PartialOrd for ActiveContext<ID>
    where ID: Debug + Clone + Eq {
    fn partial_cmp(&self, other: &ActiveContext<ID>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct StateInfo {
    pub active : bool,
    pub start_time : f64,
    pub stop_time : f64
}

#[derive(Debug, Clone)]
pub struct WindowData {
    pub size : (f64, f64),
    pub cursor_position : Option<WindowPosition>
}

#[derive(Debug)]
pub struct StateStorage<ACTION>
    where ACTION : Hash + Eq + Clone {
    pub states : HashMap<ACTION, StateInfo>,
}

impl <ACTION> StateStorage<ACTION>
    where ACTION : Hash + Eq + Clone {

    pub fn new() -> StateStorage<ACTION> {
        StateStorage {
            states : HashMap::default()
        }
    }

    pub fn get(&self, state: &ACTION) -> Option<StateInfo> {
        match self.states.get(state) {
            Some(info) => Some(info.clone()),
            None => None
        }
    }

    pub fn is_active(&self, state: &ACTION) -> bool {
        match self.states.get(state) {
            Some(info) => info.active,
            None => false
        }
    }
}
