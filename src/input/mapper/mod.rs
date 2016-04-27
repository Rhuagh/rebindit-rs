use std::collections::HashMap;
use std::string::ToString;

pub mod config;

use super::raw::Input as RawInput;
use super::raw::InputEvent as RawInputEvent;
use super::raw::InputAction as RawInputAction;
use super::raw::Modifiers as RawModifiers;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Action,
    State,
    Range
}

#[derive(Debug)]
pub enum Input {
    Action(u32, HashMap<String, Args>),
    State(u32, HashMap<String, Args>),
    Range(u32, HashMap<String, Args>)
}

pub type InputEvent = (f64, String, Input); // (time, context, mapped input)

pub enum RawType {
    Key
}

#[derive(Clone, Debug)]
pub struct Constant {
    pub mapped_type : Type,
    pub name : String,
    pub id : u32,
    pub description : String
}

pub trait MappingRaw {
    fn matches(&self, raw : &RawInput) -> bool;
    fn get_fields(&self, raw : &RawInput, fields : &Vec<String>) -> HashMap<String, Args>;
}

#[derive(Debug)]
pub struct MappingOut {
    pub constant_id : u32,
    pub args : Vec<String>
}

#[derive(Debug)]
pub struct MappingRawKey {
    scancode : Option<u32>,
    action : Option<RawInputAction>,
    modifiers : Option<u32>
}

#[derive(Debug)]
pub enum Args {
    Scancode(u32),
    Modifiers(RawModifiers),
    Action(RawInputAction)
}

impl MappingRaw for MappingRawKey {

    fn matches(&self, raw : &RawInput) -> bool {
        match raw {
            &RawInput::Key(scancode, action, modifiers) => {
                let s_match = match self.scancode {
                    Some(config_scancode) => config_scancode == scancode,
                    None => true
                };
                let a_match = match self.action {
                    Some(config_action) => config_action == action,
                    None => true
                };
                let m_match = match self.modifiers {
                    Some(config_modifiers) => (config_modifiers & modifiers.bits()) == config_modifiers,
                    None => true
                };
                s_match && a_match && m_match
            },
            _ => false
        }
    }

    fn get_fields(&self, raw : &RawInput, fields : &Vec<String>) -> HashMap<String, Args> {
        let mut m = HashMap::new();
        match raw {
            &RawInput::Key(scancode, action, modifiers) => {
                for field in fields {
                    match field.as_ref() {
                        "scancode" => { m.insert("scancode".to_string(), Args::Scancode(scancode)); },
                        "modifiers" => { m.insert("modifiers".to_string(), Args::Modifiers(modifiers)); },
                        "action" => { m.insert("action".to_string(), Args::Action(action)); },
                        _ => ()
                    }
                }
            },
            _ => ()
        }
        m
    }
}

pub struct Mapping {
    pub raw : Box<MappingRaw>,
    pub mapped : MappingOut
}

pub struct Context {
    pub id : String,
    pub priority : u32,
    pub mappings : Vec<Mapping>
}

pub struct InputMapper {
    contexts : Vec<Context>,
    constants : HashMap<u32, Constant>
}

impl InputMapper {

    pub fn new() -> InputMapper {
        InputMapper {
            contexts : vec![],
            constants : HashMap::new()
        }
    }

    pub fn with_context(&mut self, context : Context) -> &mut Self {
        self.contexts.push(context);
        self
    }

    pub fn with_contexts(&mut self, contexts : Vec<Context>) -> &mut Self {
        for context in contexts {
            self.contexts.push(context);
        }
        self.contexts.sort_by(|a, b| a.priority.cmp(&b.priority));
        self
    }

    pub fn with_contexts_file(&mut self, file : &str) -> &mut Self {
        self.with_contexts(config::parse_contexts_file(file))
    }

    pub fn with_constants(&mut self, constants : Vec<Constant>) -> &mut Self {
        for constant in constants {
            self.constants.insert(constant.id.clone(), constant);
        }
        self
    }

    fn process_in_context(&self, input : &RawInput, context : &Context) -> Option<Input> {
        for mapping in &context.mappings {
            if mapping.raw.matches(input) {
                match self.constants.get(&mapping.mapped.constant_id) {
                    Some(constant) => {
                        let args = mapping.raw.get_fields(input, &mapping.mapped.args);
                        match constant.mapped_type {
                            Type::Action => {
                                return Some(Input::Action(constant.id.clone(), args));
                            },
                            Type::State => {
                                return Some(Input::State(constant.id.clone(), args));
                            },
                            Type::Range => {
                                return Some(Input::Range(constant.id.clone(), args));
                            }
                        }
                    },
                    _ => ()
                }
            }
        }
        None
    }

    pub fn with_constants_file(&mut self, file : &str) -> &mut Self {
        self.with_constants(config::parse_constants_file(file))
    }

    pub fn process(&self, input : &RawInputEvent) -> Vec<InputEvent> {
        let mut v = vec![];
        for context in &self.contexts {
            match self.process_in_context(&input.1, &context) {
                Some(i) => v.push((input.0, context.id.clone(), i)),
                None => ()
            }
        }
        v
    }

}
