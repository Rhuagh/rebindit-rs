#[macro_use]
extern crate amethyst_config;

#[macro_use]
extern crate bitflags;

pub mod config;
pub mod event;
pub mod types;
pub mod raw;

use amethyst_config::Element;

pub struct InputHandler<C> where C : std::hash::Hash + std::cmp::Eq + std::str::FromStr {
    sources : Vec<Box<raw::RawInputSource>>,
    contexts : Vec<types::Context<C>>,
    active_contexts : Vec<types::ActiveContext>
}

fn get_keycode(raw_input : &raw::RawInput) -> Option<event::Argument> {
    match raw_input.event {
        raw::RawInputEvent::Key(ref keycode, _, _) => Some(event::Argument::KeyCode(keycode.clone())),
        _ => None
    }
}

fn get_modifiers(raw_input: &raw::RawInput) -> Option<event::Argument> {
    match raw_input.event {
        raw::RawInputEvent::Key(_, _, modifiers) => Some(event::Argument::Modifiers(modifiers.into())),
        raw::RawInputEvent::Button(_, _, _, modifiers) => Some(event::Argument::Modifiers(modifiers.into())),
        _ => None
    }
}

fn get_value(raw_input: &raw::RawInput) -> Option<event::Argument> {
    match raw_input.event {
        raw::RawInputEvent::Char(ch) => Some(event::Argument::Value(ch)),
        _ => None
    }
}

fn get_action(raw_input: &raw::RawInput) -> Option<event::Argument> {
    match raw_input.event {
        raw::RawInputEvent::Key(_, ref action, _) => Some(event::Argument::Action(action.clone().into())),
        raw::RawInputEvent::Button(_, _, ref action, _) => Some(event::Argument::Action(action.clone().into())),
        _ => None
    }
}

fn get_cursor_position(raw_input: &raw::RawInput) -> Option<event::Argument> {
    match raw_input.event {
        raw::RawInputEvent::Button(_, (x, y), _, _) => Some(event::Argument::CursorPosition(x, y)),
        _ => None
    }
}

impl<C> InputHandler<C>
    where C : std::hash::Hash + std::cmp::Eq + std::str::FromStr +
              std::fmt::Debug + std::clone::Clone + types::ToMappedType {
    pub fn new() -> InputHandler<C> {
        InputHandler {
            sources : Vec::default(),
            contexts : Vec::default(),
            active_contexts : Vec::default()
        }
    }

    pub fn with_input_source<S>(mut self, source : S) -> Self
        where S : raw::RawInputSource + 'static {
        self.sources.push(Box::new(source));
        self
    }

    pub fn with_context(mut self, context : types::Context<C>) -> Self {
        self.contexts.push(context);
        self
    }

    pub fn with_contexts(mut self, contexts : &mut Vec<types::Context<C>>) -> Self {
        self.contexts.append(contexts);
        println!("{:?}", self.contexts);
        self
    }

    pub fn with_bindings_file(self, file: &str) -> Self {
        let bindings = config::ConfigBindings::from_file(file).expect("Failed reading bindings file");
        let mut contexts : Vec<types::Context<C>> = bindings.into();
        for c in &mut contexts {
            for m in &mut c.mappings {
                match m.mapped.action {
                    Some(ref action) => {
                        m.mapped_type = Some(action.to_mapped_type())
                    },
                    None => ()
                };
            }
            c.mappings.retain(|m| m.mapped.action.is_some() && m.mapped_type.is_some());
        }
        self.with_contexts(&mut contexts)
    }

    pub fn activate_context(&mut self, context_id : &str, priority: u32) {
        match self.contexts.iter().position(|c| c.id == context_id) {
            Some(index) => self.active_contexts.push(types::ActiveContext::new(priority, index)),
            None => ()
        };
        self.active_contexts.sort();
        println!("{:?}", self.active_contexts);
    }

    pub fn deactivate_context(&mut self, context_id : &str) {
        match self.contexts.iter().position(|c| c.id == context_id) {
            Some(index) => {
                match self.active_contexts.iter().position(|ac| ac.index == index) {
                    Some(ac_index) => {
                        self.active_contexts.remove(ac_index);
                        ()
                    },
                    None => ()
                };
            },
            None => ()
        };
    }

    fn process_window_input(&self, raw_input : &raw::RawInput) -> Option<event::WindowEvent> {
        match raw_input.event {
            raw::RawInputEvent::Resize(x, y) => Some(event::WindowEvent::Resize(x, y)),
            raw::RawInputEvent::Focus(b) => Some(event::WindowEvent::Focus(if b { event::FocusAction::Enter } else { event::FocusAction::Exit })),
            raw::RawInputEvent::Close => Some(event::WindowEvent::Close),
            _ => None
        }
    }

    fn check_mapping(&self, mapping : &types::Mapping<C>, raw_input : &raw::RawInput) -> bool {
        false // TODO
    }

    fn as_action(&self, mapped: &types::Mapped<C>, raw_input : &raw::RawInput) -> event::ControllerEvent<C> {
        event::ControllerEvent::Action(mapped.action.clone().unwrap(), self.arguments(&mapped.args, raw_input))
    }

    fn arguments(&self, args : &Vec<types::ActionArgument>, raw_input : &raw::RawInput) -> Vec<event::Argument> {
        args.iter().filter_map(|arg| {
            match arg {
                &types::ActionArgument::KeyCode => get_keycode(raw_input),
                &types::ActionArgument::Value => get_value(raw_input),
                &types::ActionArgument::Modifiers => get_modifiers(raw_input),
                &types::ActionArgument::Action => get_action(raw_input),
                &types::ActionArgument::CursorPosition => get_cursor_position(raw_input),
            }
        }).collect()
    }

    fn process_controller_input_in_context(&self, context : &types::Context<C>, raw_input: &raw::RawInput) -> Option<event::ControllerEvent<C>> {
        for m in &context.mappings {
            if self.check_mapping(m, raw_input) {
                match m.mapped_type {
                    Some(types::MappedType::Action) => return Some(self.as_action(&m.mapped, raw_input)),
                    // TODO
                    _ => ()
                }
            }
        }
        None
    }

    fn process_controller_input(&self, raw_input: &raw::RawInput) -> Option<event::ControllerEvent<C>> {
        for active_context in &self.active_contexts {
            match self.process_controller_input_in_context(&self.contexts[active_context.index], raw_input) {
                Some(v) => return Some(v),
                None => ()
            }
        }
        None
    }

    pub fn process(&mut self) -> Vec<event::Event<C>> {
        let raw_input : Vec<raw::RawInput> = self.sources.iter_mut().flat_map(|s| s.process()).collect();
        if raw_input.len() > 0 {
            println!("{:?}", raw_input);
        }
        let mut window_input : Vec<event::Event<C>> = raw_input.iter()
            .filter_map(|ri| self.process_window_input(&ri))
            .map(|wi| event::Event::Window(wi))
            .collect();
        let controller_input : Vec<event::Event<C>> = raw_input.iter()
            .filter_map(|ri| { self.process_controller_input(&ri) } )
            .map(|ci| event::Event::Controller(ci))
            .collect();
        window_input.extend(controller_input);
        window_input
    }
}
