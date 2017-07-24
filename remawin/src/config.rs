use std::str::FromStr;
use std;

use super::types::KeyCode;

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

impl<C> Into<super::types::Mapping<C>> for ConfigMapping
    where C : FromStr + super::types::ActionMetadata + std::hash::Hash + std::cmp::Eq + std::clone::Clone {
    fn into(self) -> super::types::Mapping<C> {
        super::types::Mapping::new(self.raw_type.into(),
                                   self.raw_args.into(),
                                   self.action.parse::<C>().ok())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ConfigContext {
    pub id: String,
    pub mappings: Vec<ConfigMapping>,
}

impl<C, I> Into<super::types::Context<C, I>> for ConfigContext
    where C : FromStr + std::cmp::Eq + std::hash::Hash + super::types::ActionMetadata + std::clone::Clone,
          I : FromStr + std::cmp::Eq + std::hash::Hash + std::clone::Clone {
    fn into(self) -> super::types::Context<C, I> {
        super::types::Context::new(self.id.parse::<I>().ok(),
                                   self.mappings.iter().map(|m| m.clone().into()).collect())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct ConfigBindings {
    pub contexts: Vec<ConfigContext>,
}

impl<C, I> Into<Vec<super::types::Context<C, I>>> for ConfigBindings
    where C : FromStr + std::cmp::Eq + std::hash::Hash + super::types::ActionMetadata + std::clone::Clone,
          I : FromStr + std::cmp::Eq + std::hash::Hash + std::clone::Clone {
    fn into(self) -> Vec<super::types::Context<C, I>> {
        self.contexts.iter().map(|c| c.clone().into()).collect()
    }
}
