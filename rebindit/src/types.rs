use winit;
use std::cmp::Ordering;
use std::collections::HashMap;

use std::hash::Hash;
use std::cmp::Eq;
use std::clone::Clone;
use std::fmt::Debug;
use std::str;

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

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Debug, Clone)]
pub enum DeviceType {
    Keyboard,
    Mouse,
    Window,
}

#[derive(Debug, Clone, Deserialize)]
pub enum RawType {
    Button(MouseButton),
    Key(KeyCode),
    Motion,
    Char,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum RawState {
    Press,
    Release,
}

bitflags! {
    pub struct Modifiers: u32 {
        const SHIFT = 1 << 0;
        const CONTROL = 1 << 1;
        const ALT = 1 << 2;
        const SUPER = 1 << 3;
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum Modifier {
    ALT,
    CONTROL,
    SHIFT,
    SUPER,
}

#[derive(Debug, Clone, Deserialize)]
pub enum ActionArgument {
    KeyCode,
    Value,
    Modifiers,
    Action,
    CursorPosition,
    ContextId,
}

#[derive(Debug, Clone, Deserialize)]
pub enum MappedType {
    Action,
    State,
    Range,
}

pub trait ActionMetadata {
    fn mapped_type(&self) -> MappedType;
    fn args(&self) -> Vec<ActionArgument>;
}

#[derive(Debug, Clone, Deserialize)]
pub struct Mapping<ACTION: Clone> {
    pub raw_type: RawType,
    pub state: Option<RawState>,
    pub modifier: Option<Modifier>,
    pub state_active: Option<ACTION>,
    pub action: ACTION,

    #[serde(default = "default_mt")]
    pub mapped_type: Option<MappedType>,
    #[serde(default = "default_aa")]
    pub action_args: Vec<ActionArgument>,
}

fn default_mt() -> Option<MappedType> {
    None
}

fn default_aa() -> Vec<ActionArgument> {
    Vec::default()
}

impl<ACTION: ActionMetadata + Clone> Mapping<ACTION> {
    pub fn new(raw_type: RawType, action: ACTION) -> Self {
        Mapping {
            raw_type: raw_type,
            mapped_type: Some(action.mapped_type()),
            action_args: action.args(),
            action: action,
            state: None,
            state_active: None,
            modifier: None,
        }
    }

    pub fn with_action(mut self, action: RawState) -> Self {
        self.state = Some(action);
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

impl<ACTION: Clone> Mapping<ACTION> {
    pub fn sanitize(&mut self) {
        match self.mapped_type {
            Some(MappedType::Action) => {
                if self.state == None {
                    self.state = Some(RawState::Release);
                }
            }
            Some(MappedType::State) => {
                if self.state != None {
                    self.state = None;
                }
            }
            _ => (),
        }
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct Context<ACTION, ID>
where
    ACTION: Hash + Eq + Clone,
    ID: Clone,
{
    pub id: ID,
    pub mappings: Vec<Mapping<ACTION>>,
}

impl<ACTION, ID> Context<ACTION, ID>
where
    ACTION: Hash + Eq + Clone + ActionMetadata,
    ID: Clone,
{
    pub fn new(id: ID) -> Self {
        Self::new_with_mappings(id, Vec::default())
    }

    pub fn new_with_mappings(id: ID, mappings: Vec<Mapping<ACTION>>) -> Self {
        Context {
            id: id,
            mappings: mappings,
        }
    }

    pub fn with_mapping(mut self, mapping: Mapping<ACTION>) -> Self {
        self.mappings.push(mapping);
        self
    }

    pub fn with_mappings(mut self, mut mappings: Vec<Mapping<ACTION>>) -> Self {
        self.mappings.append(&mut mappings);
        self
    }

    pub fn sanitize(&mut self) {
        for mut m in &mut self.mappings {
            m.sanitize();
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Bindings<ACTION, ID>
where
    ACTION: Hash + Eq + Clone,
    ID: Clone,
{
    contexts: Vec<Context<ACTION, ID>>,
}

#[derive(Debug, Eq, Clone)]
pub struct ActiveContext<ID>
where
    ID: Debug + Clone,
{
    pub priority: u32,
    pub context_id: ID,
}

pub type WindowPosition = (f64, f64);
pub type WindowSize = (u32, u32);

impl<ID> ActiveContext<ID>
where
    ID: Debug + Clone,
{
    pub fn new(priority: u32, context_id: &ID) -> ActiveContext<ID> {
        ActiveContext {
            priority: priority,
            context_id: context_id.clone(),
        }
    }
}

impl<ID> PartialEq for ActiveContext<ID>
where
    ID: Debug + Clone,
{
    fn eq(&self, other: &ActiveContext<ID>) -> bool {
        self.priority == other.priority
    }
}

impl<ID> Ord for ActiveContext<ID>
where
    ID: Debug + Clone + Eq,
{
    fn cmp(&self, other: &ActiveContext<ID>) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<ID> PartialOrd for ActiveContext<ID>
where
    ID: Debug + Clone + Eq,
{
    fn partial_cmp(&self, other: &ActiveContext<ID>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct StateInfo {
    pub active: bool,
    pub start_time: f64,
    pub stop_time: f64,
}

#[derive(Debug, Clone)]
pub struct WindowData {
    pub size: (f64, f64),
    pub cursor_position: Option<WindowPosition>,
}

#[derive(Debug)]
pub struct StateStorage<ACTION>
where
    ACTION: Hash + Eq + Clone,
{
    pub states: HashMap<ACTION, StateInfo>,
}

impl<ACTION> StateStorage<ACTION>
where
    ACTION: Hash + Eq + Clone,
{
    pub fn new() -> StateStorage<ACTION> {
        StateStorage { states: HashMap::default() }
    }

    pub fn get(&self, state: &ACTION) -> Option<StateInfo> {
        match self.states.get(state) {
            Some(info) => Some(info.clone()),
            None => None,
        }
    }

    pub fn is_active(&self, state: &ACTION) -> bool {
        match self.states.get(state) {
            Some(info) => info.active,
            None => false,
        }
    }
}


impl From<winit::ElementState> for RawState {
    fn from(state: winit::ElementState) -> Self {
        match state {
            winit::ElementState::Pressed => RawState::Press,
            winit::ElementState::Released => RawState::Release,
        }
    }
}

impl From<winit::ModifiersState> for Modifiers {
    fn from(modifiers: winit::ModifiersState) -> Self {
        let mut m = Modifiers::empty();
        m.set(SHIFT, modifiers.shift);
        m.set(CONTROL, modifiers.ctrl);
        m.set(ALT, modifiers.alt);
        m.set(SUPER, modifiers.logo);
        m
    }
}

impl<'a> From<&'a winit::MouseButton> for MouseButton {
    fn from(button: &'a winit::MouseButton) -> Self {
        match *button {
            winit::MouseButton::Left => MouseButton::Left,
            winit::MouseButton::Right => MouseButton::Right,
            winit::MouseButton::Middle => MouseButton::Middle,
            winit::MouseButton::Other(b) => MouseButton::Other(b),
        }
    }
}

impl<'a> From<&'a winit::VirtualKeyCode> for KeyCode {
    fn from(kc: &'a winit::VirtualKeyCode) -> KeyCode {
        match *kc {
            winit::VirtualKeyCode::Key1 => KeyCode::Key1,
            winit::VirtualKeyCode::Key2 => KeyCode::Key2,
            winit::VirtualKeyCode::Key3 => KeyCode::Key3,
            winit::VirtualKeyCode::Key4 => KeyCode::Key4,
            winit::VirtualKeyCode::Key5 => KeyCode::Key5,
            winit::VirtualKeyCode::Key6 => KeyCode::Key6,
            winit::VirtualKeyCode::Key7 => KeyCode::Key7,
            winit::VirtualKeyCode::Key8 => KeyCode::Key8,
            winit::VirtualKeyCode::Key9 => KeyCode::Key9,
            winit::VirtualKeyCode::Key0 => KeyCode::Key0,
            winit::VirtualKeyCode::A => KeyCode::A,
            winit::VirtualKeyCode::B => KeyCode::B,
            winit::VirtualKeyCode::C => KeyCode::C,
            winit::VirtualKeyCode::D => KeyCode::D,
            winit::VirtualKeyCode::E => KeyCode::E,
            winit::VirtualKeyCode::F => KeyCode::F,
            winit::VirtualKeyCode::G => KeyCode::G,
            winit::VirtualKeyCode::H => KeyCode::H,
            winit::VirtualKeyCode::I => KeyCode::I,
            winit::VirtualKeyCode::J => KeyCode::J,
            winit::VirtualKeyCode::K => KeyCode::K,
            winit::VirtualKeyCode::L => KeyCode::L,
            winit::VirtualKeyCode::M => KeyCode::M,
            winit::VirtualKeyCode::N => KeyCode::N,
            winit::VirtualKeyCode::O => KeyCode::O,
            winit::VirtualKeyCode::P => KeyCode::P,
            winit::VirtualKeyCode::Q => KeyCode::Q,
            winit::VirtualKeyCode::R => KeyCode::R,
            winit::VirtualKeyCode::S => KeyCode::S,
            winit::VirtualKeyCode::T => KeyCode::T,
            winit::VirtualKeyCode::U => KeyCode::U,
            winit::VirtualKeyCode::V => KeyCode::V,
            winit::VirtualKeyCode::W => KeyCode::W,
            winit::VirtualKeyCode::X => KeyCode::X,
            winit::VirtualKeyCode::Y => KeyCode::Y,
            winit::VirtualKeyCode::Z => KeyCode::Z,
            winit::VirtualKeyCode::Escape => KeyCode::Escape,
            winit::VirtualKeyCode::F1 => KeyCode::F1,
            winit::VirtualKeyCode::F2 => KeyCode::F2,
            winit::VirtualKeyCode::F3 => KeyCode::F3,
            winit::VirtualKeyCode::F4 => KeyCode::F4,
            winit::VirtualKeyCode::F5 => KeyCode::F5,
            winit::VirtualKeyCode::F6 => KeyCode::F6,
            winit::VirtualKeyCode::F7 => KeyCode::F7,
            winit::VirtualKeyCode::F8 => KeyCode::F8,
            winit::VirtualKeyCode::F9 => KeyCode::F9,
            winit::VirtualKeyCode::F10 => KeyCode::F10,
            winit::VirtualKeyCode::F11 => KeyCode::F11,
            winit::VirtualKeyCode::F12 => KeyCode::F12,
            winit::VirtualKeyCode::F13 => KeyCode::F13,
            winit::VirtualKeyCode::F14 => KeyCode::F14,
            winit::VirtualKeyCode::F15 => KeyCode::F15,
            winit::VirtualKeyCode::Snapshot => KeyCode::Snapshot,
            winit::VirtualKeyCode::Scroll => KeyCode::Scroll,
            winit::VirtualKeyCode::Pause => KeyCode::Pause,
            winit::VirtualKeyCode::Insert => KeyCode::Insert,
            winit::VirtualKeyCode::Home => KeyCode::Home,
            winit::VirtualKeyCode::Delete => KeyCode::Delete,
            winit::VirtualKeyCode::End => KeyCode::End,
            winit::VirtualKeyCode::PageDown => KeyCode::PageDown,
            winit::VirtualKeyCode::PageUp => KeyCode::PageUp,
            winit::VirtualKeyCode::Left => KeyCode::Left,
            winit::VirtualKeyCode::Up => KeyCode::Up,
            winit::VirtualKeyCode::Right => KeyCode::Right,
            winit::VirtualKeyCode::Down => KeyCode::Down,
            winit::VirtualKeyCode::Back => KeyCode::Back,
            winit::VirtualKeyCode::Return => KeyCode::Return,
            winit::VirtualKeyCode::Space => KeyCode::Space,
            winit::VirtualKeyCode::Compose => KeyCode::Compose,
            winit::VirtualKeyCode::Numlock => KeyCode::Numlock,
            winit::VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
            winit::VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
            winit::VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
            winit::VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
            winit::VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
            winit::VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
            winit::VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
            winit::VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
            winit::VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
            winit::VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
            winit::VirtualKeyCode::AbntC1 => KeyCode::AbntC1,
            winit::VirtualKeyCode::AbntC2 => KeyCode::AbntC2,
            winit::VirtualKeyCode::Add => KeyCode::Add,
            winit::VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
            winit::VirtualKeyCode::Apps => KeyCode::Apps,
            winit::VirtualKeyCode::At => KeyCode::At,
            winit::VirtualKeyCode::Ax => KeyCode::Ax,
            winit::VirtualKeyCode::Backslash => KeyCode::Backslash,
            winit::VirtualKeyCode::Calculator => KeyCode::Calculator,
            winit::VirtualKeyCode::Capital => KeyCode::Capital,
            winit::VirtualKeyCode::Colon => KeyCode::Colon,
            winit::VirtualKeyCode::Comma => KeyCode::Comma,
            winit::VirtualKeyCode::Convert => KeyCode::Convert,
            winit::VirtualKeyCode::Decimal => KeyCode::Decimal,
            winit::VirtualKeyCode::Divide => KeyCode::Divide,
            winit::VirtualKeyCode::Equals => KeyCode::Equals,
            winit::VirtualKeyCode::Grave => KeyCode::Grave,
            winit::VirtualKeyCode::Kana => KeyCode::Kana,
            winit::VirtualKeyCode::Kanji => KeyCode::Kanji,
            winit::VirtualKeyCode::LAlt => KeyCode::LAlt,
            winit::VirtualKeyCode::LBracket => KeyCode::LBracket,
            winit::VirtualKeyCode::LControl => KeyCode::LControl,
            winit::VirtualKeyCode::LMenu => KeyCode::LMenu,
            winit::VirtualKeyCode::LShift => KeyCode::LShift,
            winit::VirtualKeyCode::LWin => KeyCode::LWin,
            winit::VirtualKeyCode::Mail => KeyCode::Mail,
            winit::VirtualKeyCode::MediaSelect => KeyCode::MediaSelect,
            winit::VirtualKeyCode::MediaStop => KeyCode::MediaStop,
            winit::VirtualKeyCode::Minus => KeyCode::Minus,
            winit::VirtualKeyCode::Multiply => KeyCode::Multiply,
            winit::VirtualKeyCode::Mute => KeyCode::Mute,
            winit::VirtualKeyCode::MyComputer => KeyCode::MyComputer,
            winit::VirtualKeyCode::NavigateForward => KeyCode::NavigateForward,
            winit::VirtualKeyCode::NavigateBackward => KeyCode::NavigateBackward,
            winit::VirtualKeyCode::NextTrack => KeyCode::NextTrack,
            winit::VirtualKeyCode::NoConvert => KeyCode::NoConvert,
            winit::VirtualKeyCode::NumpadComma => KeyCode::NumpadComma,
            winit::VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
            winit::VirtualKeyCode::NumpadEquals => KeyCode::NumpadEquals,
            winit::VirtualKeyCode::OEM102 => KeyCode::OEM102,
            winit::VirtualKeyCode::Period => KeyCode::Period,
            winit::VirtualKeyCode::PlayPause => KeyCode::PlayPause,
            winit::VirtualKeyCode::Power => KeyCode::Power,
            winit::VirtualKeyCode::PrevTrack => KeyCode::PrevTrack,
            winit::VirtualKeyCode::RAlt => KeyCode::RAlt,
            winit::VirtualKeyCode::RBracket => KeyCode::RBracket,
            winit::VirtualKeyCode::RControl => KeyCode::RControl,
            winit::VirtualKeyCode::RMenu => KeyCode::RMenu,
            winit::VirtualKeyCode::RShift => KeyCode::RShift,
            winit::VirtualKeyCode::RWin => KeyCode::RWin,
            winit::VirtualKeyCode::Semicolon => KeyCode::Semicolon,
            winit::VirtualKeyCode::Slash => KeyCode::Slash,
            winit::VirtualKeyCode::Sleep => KeyCode::Sleep,
            winit::VirtualKeyCode::Stop => KeyCode::Stop,
            winit::VirtualKeyCode::Subtract => KeyCode::Subtract,
            winit::VirtualKeyCode::Sysrq => KeyCode::Sysrq,
            winit::VirtualKeyCode::Tab => KeyCode::Tab,
            winit::VirtualKeyCode::Underline => KeyCode::Underline,
            winit::VirtualKeyCode::Unlabeled => KeyCode::Unlabeled,
            winit::VirtualKeyCode::VolumeDown => KeyCode::VolumeDown,
            winit::VirtualKeyCode::VolumeUp => KeyCode::VolumeUp,
            winit::VirtualKeyCode::Wake => KeyCode::Wake,
            winit::VirtualKeyCode::WebBack => KeyCode::WebBack,
            winit::VirtualKeyCode::WebFavorites => KeyCode::WebFavorites,
            winit::VirtualKeyCode::WebForward => KeyCode::WebForward,
            winit::VirtualKeyCode::WebHome => KeyCode::WebHome,
            winit::VirtualKeyCode::WebRefresh => KeyCode::WebRefresh,
            winit::VirtualKeyCode::WebSearch => KeyCode::WebSearch,
            winit::VirtualKeyCode::WebStop => KeyCode::WebStop,
            winit::VirtualKeyCode::Yen => KeyCode::Yen,
        }
    }
}
