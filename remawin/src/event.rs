use std::clone::Clone;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum FocusAction {
    Enter,
    Exit
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
    Resize(u32, u32),
    Focus(FocusAction),
    Close
}

pub type StateDuration = f64;
pub type RangeDiff = (f64, f64);

#[derive(Debug, Clone, PartialEq)]
pub enum StateAction {
    Activated,
    Active,
    Deactivated
}

#[derive(Debug, Clone, PartialEq)]
pub enum Argument<ID> where ID: Debug + Clone {
    KeyCode(super::types::KeyCode),
    Value(char),
    Modifiers(super::types::Modifiers),
    Action(super::types::RawAction),
    CursorPosition(f64, f64),
    ContextId(ID),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ControllerEvent<ACTION: Debug,
                         ID: Debug + Clone> {
    Action(ACTION, Vec<Argument<ID>>),
    State(ACTION, StateAction, StateDuration, Vec<Argument<ID>>),
    Range(ACTION, RangeDiff, Vec<Argument<ID>>)
}

#[derive(Debug, Clone)]
pub enum Event<ACTION: Debug, ID: Debug + Clone> {
    Window(WindowEvent),
    Controller(ControllerEvent<ACTION, ID>)
}
