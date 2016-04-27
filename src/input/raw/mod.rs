use std;

pub mod glfw;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputAction {
    Press,
    Release,
    Repeat
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ButtonDevice {
    Mouse,
    Other(u32)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FocusAction {
    Enter,
    Exit
}

bitflags! {
    pub flags Modifiers: u32 {
        const SHIFT = 1 << 0,
        const CONTROL = 1 << 1,
        const ALT = 1 << 2,
        const SUPER = 1 << 3
    }
}

pub type ButtonId = u32;
pub type Scancode = u32;

#[derive(Copy, Clone, Debug)]
pub enum Input {
    Key(Scancode, InputAction, Modifiers),
    CursorPosition(f64, f64),
    Button(ButtonDevice, ButtonId, InputAction, Modifiers),
    Scroll(f64, f64),
    Focus(FocusAction),
    Close
}

pub trait Adapter {
    fn process(&mut self) -> Vec<InputEvent>;
}

pub struct AdapterManager {
    adapters : Vec<Box<Adapter>>
}

impl std::fmt::Debug for AdapterManager {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "AdapterManager {{}}")
    }
}

pub type InputEvent = (f64, Input);

impl AdapterManager {

    pub fn new(adapters: Vec<Box<Adapter>>) -> AdapterManager {
        AdapterManager {
            adapters : adapters
        }
    }

    pub fn with_adapter<A>(&mut self, adapter : A) -> &mut AdapterManager
        where A : Adapter + 'static {
        self.adapters.push(Box::new(adapter));
        self
    }

    pub fn process(&mut self) -> Vec<InputEvent> {
        self.adapters.iter_mut()
            .flat_map(|adapter| adapter.process())
            .collect()
    }
}
