pub mod raw;
pub mod mapper;

pub use input::mapper::Input;

pub struct InputHandler {
    raw_adapter_manager : raw::AdapterManager,
    mapper : mapper::InputMapper
}

impl InputHandler {

    pub fn new() -> InputHandler {
        InputHandler {
            raw_adapter_manager : raw::AdapterManager::new(vec![]),
            mapper : mapper::InputMapper::new()
        }
    }

    pub fn process(&mut self) -> Vec<mapper::InputEvent> {
        self.raw_adapter_manager
            .process()
            .iter()
            .flat_map(|raw| self.mapper.process(raw))
            .collect()
    }

    pub fn with_raw_adapter<A>(mut self, adapter : A) -> Self
        where A : raw::Adapter + 'static {
        self.raw_adapter_manager.with_adapter(adapter);
        self
    }

    pub fn with_context(mut self, context : mapper::Context) -> Self {
        self.mapper.with_context(context);
        self
    }

    pub fn with_contexts(mut self, contexts : Vec<mapper::Context>) -> Self {
        self.mapper.with_contexts(contexts);
        self
    }

    pub fn with_contexts_file(mut self, file : &str) -> Self {
        self.mapper.with_contexts_file(file);
        self
    }

    pub fn with_constants(mut self, constants : Vec<mapper::Constant>) -> Self {
        self.mapper.with_constants(constants);
        self
    }

    pub fn with_constants_file(mut self, file : &str) -> Self {
        self.mapper.with_constants_file(file);
        self
    }
}
