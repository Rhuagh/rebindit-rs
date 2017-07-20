#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate time;

pub mod config;
pub mod event;
pub mod types;
pub mod raw;
pub mod mapping;

pub struct InputHandler<C> where C : std::hash::Hash + std::cmp::Eq + std::str::FromStr {
    sources : Vec<Box<raw::RawInputSource>>,
    contexts : Vec<types::Context<C>>,
    active_contexts : Vec<types::ActiveContext>
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
        let f = std::fs::File::open(file).expect("Failed opening bindings config file");
        let bindings : config::ConfigBindings = serde_yaml::from_reader(f).expect("Failed parsing Yaml string");
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
            raw::RawInputEvent::Focus(b) => Some(event::WindowEvent::Focus(
                if b { event::FocusAction::Enter } else { event::FocusAction::Exit })),
            raw::RawInputEvent::Close => Some(event::WindowEvent::Close),
            _ => None
        }
    }

    fn process_controller_input(&mut self, raw_input: &raw::RawInput) -> Option<event::ControllerEvent<C>> {
        for ref active_context in &self.active_contexts {
            match self.contexts[active_context.index].process(raw_input) {
                Some(v) => return Some(v),
                None => ()
            }
        }
        None
    }

    pub fn process(&mut self) -> Vec<event::Event<C>> {
        let raw_input : Vec<raw::RawInput> = self.sources.iter_mut().flat_map(|s| s.process()).collect();
        let mut window_input : Vec<event::Event<C>> = raw_input.iter()
            .filter_map(|ri| self.process_window_input(&ri))
            .map(|wi| event::Event::Window(wi))
            .collect();
        let controller_input : Vec<event::Event<C>> = raw_input.iter()
            .filter_map(|ri| { self.process_controller_input(&ri) } )
            .map(|ci| event::Event::Controller(ci))
            .collect();
        if controller_input.len() > 0 {
            println!("{:?}", controller_input);
        }
        window_input.extend(controller_input);
        window_input
    }
}
