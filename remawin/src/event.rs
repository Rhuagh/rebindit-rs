use std;

#[derive(Debug)]
pub enum FocusAction {
    Enter,
    Exit
}

#[derive(Debug)]
pub enum WindowEvent {
    Resize(u32, u32),
    Focus(FocusAction),
    Close
}

pub type StateDuration = f64;
pub type RangeDiff = (f64, f64);

#[derive(Debug)]
pub enum StateAction {
    Active,
    Deactivated
}

#[derive(Debug)]
pub enum Argument {
    KeyCode(super::types::KeyCode),
    Value(char),
    Modifiers(super::types::Modifiers),
    Action(super::types::RawAction),
    CursorPosition(f64, f64),
    ContextId(String),
}

#[derive(Debug)]
pub enum ControllerEvent<C : std::fmt::Debug> {
    Action(C, Vec<Argument>),
    State(C, StateAction, StateDuration, Vec<Argument>),
    Range(C, RangeDiff, Vec<Argument>)
}

#[derive(Debug)]
pub enum Event<C : std::fmt::Debug> {
    Window(WindowEvent),
    Controller(ControllerEvent<C>)
}
