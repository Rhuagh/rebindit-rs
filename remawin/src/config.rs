use std::str::FromStr;
use std;
use std::collections::HashMap;

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

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum ConfigRawType {
    Button,
    Key,
    Motion,
    Char,
}

impl Into<super::types::RawType> for ConfigRawType {
    fn into(self) -> super::types::RawType {
        match self {
            ConfigRawType::Button => super::types::RawType::Button,
            ConfigRawType::Key => super::types::RawType::Key,
            ConfigRawType::Motion => super::types::RawType::Motion,
            ConfigRawType::Char => super::types::RawType::Char,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum ConfigArgsType {
    KeyCode,
    Value,
    Modifiers,
    Action,
    CursorPosition,
    ContextId,
}

impl Into<super::types::ActionArgument> for ConfigArgsType {
    fn into(self) -> super::types::ActionArgument {
        match self {
            ConfigArgsType::KeyCode => super::types::ActionArgument::KeyCode,
            ConfigArgsType::Value => super::types::ActionArgument::Value,
            ConfigArgsType::Modifiers => super::types::ActionArgument::Modifiers,
            ConfigArgsType::Action => super::types::ActionArgument::Action,
            ConfigArgsType::CursorPosition => super::types::ActionArgument::CursorPosition,
            ConfigArgsType::ContextId => super::types::ActionArgument::ContextId,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum ConfigMappedType {
    Action,
    State,
    Range,
}

impl Into<super::types::MappedType> for ConfigMappedType {
    fn into(self) -> super::types::MappedType {
        match self {
            ConfigMappedType::Action => super::types::MappedType::Action,
            ConfigMappedType::State => super::types::MappedType::State,
            ConfigMappedType::Range => super::types::MappedType::Range,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum ConfigInputAction {
    Press,
    Release,
    Repeat,
}

impl Into<super::types::RawAction> for ConfigInputAction {
    fn into(self) -> super::types::RawAction {
        match self {
            ConfigInputAction::Press => super::types::RawAction::Press,
            ConfigInputAction::Release => super::types::RawAction::Release,
            ConfigInputAction::Repeat => super::types::RawAction::Repeat,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum ConfigModifier {
    ALT,
    CONTROL,
    SHIFT,
    SUPER
}

impl Into<super::types::Modifier> for ConfigModifier {
    fn into(self) -> super::types::Modifier {
        match self {
            ConfigModifier::ALT => super::types::Modifier::ALT,
            ConfigModifier::CONTROL => super::types::Modifier::CONTROL,
            ConfigModifier::SHIFT => super::types::Modifier::SHIFT,
            ConfigModifier::SUPER => super::types::Modifier::SUPER,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ConfigRawArgs {
    pub action: Option<ConfigInputAction>,
    pub keycode: Option<KeyCode>,
    pub button: Option<u32>,
    pub modifier: Option<ConfigModifier>
}

impl Into<super::types::RawArgs> for ConfigRawArgs {
    fn into(self) -> super::types::RawArgs {
        super::types::RawArgs {
            action: match self.action {
                Some(a) => Some(a.into()),
                None => None
            },
            keycode: match self.keycode {
                Some(k) => Some(k.into()),
                None => None
            },
            button: match self.button {
                Some(b) => Some(b),
                None => None
            },
            modifier: match self.modifier {
                Some(m) => Some(m.into()),
                None => None
            }
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ConfigMapping {
    pub raw_type: ConfigRawType,
    pub raw_args : ConfigRawArgs,
    pub action: String,
}

impl<C: FromStr + super::types::ActionMetadata> Into<super::types::Mapping<C>> for ConfigMapping {
    fn into(self) -> super::types::Mapping<C> {
        match self.action.parse::<C>() {
            Ok(action) => {
                let mapped_type = action.mapped_type();
                let args = action.args();
                super::types::Mapping {
                    raw_type : self.raw_type.into(),
                    mapped_type : Some(mapped_type),
                    action : Some(action),
                    action_args : args,
                    raw_args : self.raw_args.into()
                }
            },
            Err(_) => {
                super::types::Mapping {
                    raw_type : self.raw_type.into(),
                    raw_args : self.raw_args.into(),
                    mapped_type : None,
                    action : None,
                    action_args : Vec::default()
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ConfigContext {
    pub id: String,
    pub mappings: Vec<ConfigMapping>,
}

impl<C> Into<super::types::Context<C>> for ConfigContext
    where C : FromStr + std::cmp::Eq + std::hash::Hash + super::types::ActionMetadata {
    fn into(self) -> super::types::Context<C> {
        super::types::Context {
            id: self.id.clone(),
            mappings: self.mappings.iter().map(|m| m.clone().into()).collect(),
            state_storage : HashMap::default()
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ConfigBindings {
    pub contexts: Vec<ConfigContext>,
}

impl<C> Into<Vec<super::types::Context<C>>> for ConfigBindings
    where C : FromStr + std::cmp::Eq + std::hash::Hash + super::types::ActionMetadata {
    fn into(self) -> Vec<super::types::Context<C>> {
        self.contexts.iter().map(|c| c.clone().into()).collect()
    }
}
