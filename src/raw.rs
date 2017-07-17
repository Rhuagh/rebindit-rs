
bitflags! {
    pub struct RawInputModifiers: u32 {
        const SHIFT = 1 << 0;
        const CONTROL = 1 << 1;
        const ALT = 1 << 2;
        const SUPER = 1 << 3;
    }
}

impl Into<super::types::Modifiers> for RawInputModifiers {
    fn into(self) -> super::types::Modifiers {
        super::types::Modifiers::from_bits(self.bits()).unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum RawInputAction {
    Press,
    Release,
    Repeat
}

impl Into<super::types::RawAction> for RawInputAction {
    fn into(self) -> super::types::RawAction {
        match self {
            RawInputAction::Press => super::types::RawAction::Press,
            RawInputAction::Release => super::types::RawAction::Release,
            RawInputAction::Repeat => super::types::RawAction::Repeat,
        }
    }
}

#[derive(Debug)]
pub enum RawInputEvent {
    Key(super::types::KeyCode, RawInputAction, RawInputModifiers),
    CursorPosition(f64, f64),
    Move(f64, f64),
    Button(u32, super::types::WindowPosition, RawInputAction, RawInputModifiers),
    Scroll(f64, f64),
    Char(char),
    Resize(u32, u32),
    Focus(bool),
    Close
}

#[derive(Debug)]
pub struct RawInput {
    pub time : f64,
    pub device_type : super::types::DeviceType,
    pub device_id : u32,
    pub event : RawInputEvent
}

impl RawInput {
    #[allow(dead_code)]
    pub fn new(time : f64, device_type : super::types::DeviceType, device_id : u32, event : RawInputEvent) -> RawInput {
        RawInput {
            time : time,
            device_type : device_type,
            device_id : device_id,
            event : event
        }
    }
}

pub trait RawInputSource {
    fn process(&mut self) -> Vec<RawInput>;
}
