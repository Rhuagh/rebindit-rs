
use std::path::Path;
use std::str::FromStr;

use amethyst_config::Element;

config! {
    enum KeyCode {
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
}

config! {
    enum ConfigRawType {
        Button,
        Key,
        Char,
    }
}

impl Into<super::types::RawType> for ConfigRawType {
    fn into(self) -> super::types::RawType {
        match self {
            ConfigRawType::Button => super::types::RawType::Button,
            ConfigRawType::Key => super::types::RawType::Key,
            ConfigRawType::Char => super::types::RawType::Char,
        }
    }
}

config! {
    enum ConfigArgsType {
        KeyCode,
        Value,
        Modifiers,
        Action,
        CursorPosition,
    }
}

impl Into<super::types::ActionArgument> for ConfigArgsType {
    fn into(self) -> super::types::ActionArgument {
        match self {
            ConfigArgsType::KeyCode => super::types::ActionArgument::KeyCode,
            ConfigArgsType::Value => super::types::ActionArgument::Value,
            ConfigArgsType::Modifiers => super::types::ActionArgument::Modifiers,
            ConfigArgsType::Action => super::types::ActionArgument::Action,
            ConfigArgsType::CursorPosition => super::types::ActionArgument::CursorPosition,
        }
    }
}

config! {
    enum ConfigMappedType {
        Action,
        State,
        Range,
    }
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

config! {
    enum ConfigInputAction {
        Press,
        Release,
        Repeat,
    }
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

config! {
    struct ConfigRawArgs {
        pub action : ConfigInputAction = ConfigInputAction::Release,
        pub keycode : KeyCode = KeyCode::Yen,
    }
}

impl Into<super::types::RawArgs> for ConfigRawArgs {
    fn into(self) -> super::types::RawArgs {
        super::types::RawArgs {
            action : Some(self.action.into()),
            keycode : match self.keycode {
                KeyCode::None => None,
                x => Some(x)
            }
        }
    }
}

config! {
    struct ConfigRaw {
        pub raw_type : ConfigRawType = ConfigRawType::Key,
        pub args : ConfigRawArgs = ConfigRawArgs::default(),
    }
}

impl Into<super::types::Raw> for ConfigRaw {
    fn into(self) -> super::types::Raw {
        super::types::Raw {
            raw_type : self.raw_type.into(),
            raw_args : self.args.into()
        }
    }
}

config! {
    struct ConfigMapped {
        pub constant_id : String = String::default(),
        pub args : Vec<ConfigArgsType> = Vec::default(),
    }
}

impl<C : FromStr> Into<super::types::Mapped<C>> for ConfigMapped {
    fn into(self) -> super::types::Mapped<C> {
        let action = match self.constant_id.parse::<C>() {
            Ok(x) => Some(x),
            Err(_) => None
        };
        super::types::Mapped {
            action : action,
            args : self.args.iter().map(|a| a.clone().into()).collect()
        }
    }
}

config! {
    struct ConfigMapping {
        pub raw : ConfigRaw = ConfigRaw::default(),
        pub mapped : ConfigMapped = ConfigMapped::default(),
    }
}

impl<C : FromStr> Into<super::types::Mapping<C>> for ConfigMapping {
    fn into(self) -> super::types::Mapping<C> {
        super::types::Mapping {
            raw : self.raw.into(),
            mapped : self.mapped.into(),
            mapped_type : None,
        }
    }
}

config! {
    struct ConfigContext {
        pub id : String = "".to_string(),
        pub mappings : Vec<ConfigMapping> = Vec::default(),
    }
}

impl<C : FromStr> Into<super::types::Context<C>> for ConfigContext {
    fn into(self) -> super::types::Context<C> {
        super::types::Context {
            id : self.id.clone(),
            mappings : self.mappings.iter().map(|m| m.clone().into()).collect()
        }
    }
}

config! {
    struct ConfigBindings {
        pub contexts : Vec<ConfigContext> = Vec::default(),
    }
}

impl<C : FromStr> Into<Vec<super::types::Context<C>>> for ConfigBindings {
    fn into(self) -> Vec<super::types::Context<C>> {
        self.contexts.iter().map(|c| c.clone().into()).collect()
    }
}
