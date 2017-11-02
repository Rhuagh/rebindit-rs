use std::clone::Clone;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum FocusAction {
    Enter,
    Exit,
}

pub type StateDuration = f64;
pub type RangeDiff = (f64, f64);

#[derive(Debug, Clone, PartialEq)]
pub enum StateAction {
    Activated,
    Active,
    Deactivated,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Argument<ID>
where
    ID: Debug + Clone,
{
    KeyCode(super::types::KeyCode),
    Value(char),
    Action(super::types::RawState),
    CursorPosition(f64, f64),
    ContextId(ID),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    Action,
    State(StateAction, StateDuration),
    Range(RangeDiff),
}

#[derive(Debug, Clone)]
pub enum Event<ACTION: Debug, ID: Debug + Clone> {
    Controller(ACTION, ActionType, Vec<Argument<ID>>),
    Resize(u32, u32),
    Focus(FocusAction),
    Close,
}
