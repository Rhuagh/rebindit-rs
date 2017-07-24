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
    Activated,
    Active,
    Deactivated
}

#[derive(Debug)]
pub enum Argument<I> where I : std::fmt::Debug + std::clone::Clone + std::hash::Hash + std::cmp::Eq {
    KeyCode(super::types::KeyCode),
    Value(char),
    Modifiers(super::types::Modifiers),
    Action(super::types::RawAction),
    CursorPosition(f64, f64),
    ContextId(I),
}

#[derive(Debug)]
pub enum ControllerEvent<C : std::fmt::Debug,
                         I : std::fmt::Debug + std::clone::Clone + std::hash::Hash + std::cmp::Eq> {
    Action(C, Vec<Argument<I>>),
    State(C, StateAction, StateDuration, Vec<Argument<I>>),
    Range(C, RangeDiff, Vec<Argument<I>>)
}

#[derive(Debug)]
pub enum Event<C : std::fmt::Debug, I : std::fmt::Debug + std::clone::Clone + std::hash::Hash + std::cmp::Eq> {
    Window(WindowEvent),
    Controller(ControllerEvent<C, I>)
}
