use std;
use std::io::Read;
use yaml_rust::{YamlLoader, Yaml};
use super::super::raw::InputAction as RawInputAction;

impl<'a> From<(super::Type, &'a Yaml)> for super::Constant {
    fn from(input : (super::Type, &'a Yaml)) -> super::Constant {
        let y = input.1;
        let id = match &y["id"] {
            &Yaml::Integer(id) => id as u32,
            _ => 0
        };
        let name = match &y["name"] {
            &Yaml::String(ref s) => s.to_string(),
            _ => String::new()
        };
        let description = match &y["description"] {
            &Yaml::String(ref s) => s.to_string(),
            _ => String::new()
        };
        super::Constant {
            mapped_type : input.0,
            name : name,
            id : id,
            description : description
        }
    }
}

fn parse_raw_mapping(y : &Yaml) -> Option<Box<super::MappingRaw>> {
    let t = match &y["type"] {
        &Yaml::String(ref s) => {
            match s.as_ref() {
                "Key" => super::RawType::Key,
                _ => return None
            }
        },
        _ => return None
    };
    match t {
        super::RawType::Key => {
            let mut raw = super::MappingRawKey {
                scancode : None,
                action : None,
                modifiers : None
            };
            match &y["args"] {
                ref args => {
                    match &args["scancode"] {
                        &Yaml::Integer(code) => raw.scancode = Some(code as u32),
                        _ => ()
                    }
                    match &args["action"] {
                        &Yaml::String(ref s) => {
                            match s.as_ref() {
                                "Press" => raw.action = Some(RawInputAction::Press),
                                "Release" => raw.action = Some(RawInputAction::Release),
                                "Repeat" => raw.action = Some(RawInputAction::Repeat),
                                _ => ()
                            }
                        },
                        _ => ()
                    }
                    match &args["modifiers"] {
                        &Yaml::Integer(modifiers) => raw.modifiers = Some(modifiers as u32),
                        _ => ()
                    }
                }
            }
            Some(Box::new(raw))
        }
    }
}

fn parse_out_mapping(y : &Yaml) -> Option<super::MappingOut> {
    let mut args = Vec::new();
    match &y["args"] {
        ref a => {
            for v in a.as_vec().unwrap().iter() {
                args.push(v.as_str().unwrap().to_string());
            }
        }
    }

    Some(super::MappingOut {
        constant_id : y["constant_id"].as_i64().unwrap() as u32,
        args : args
    })
}

fn parse_mapping(y : &Yaml) -> Option<super::Mapping> {
    let raw = match &y["raw"] {
        ref r => {
            match parse_raw_mapping(r) {
                Some(r2) => r2,
                None => return None
            }
        }
    };
    let mapped = match &y["mapped"] {
        ref r => {
            match parse_out_mapping(r) {
                Some(r2) => r2,
                None => return None
            }
        }
    };
    Some(super::Mapping {
        raw : raw,
        mapped : mapped
    })
}

pub fn parse_context(y : &Yaml) -> Option<super::Context> {
    let id = match &y["id"] {
        &Yaml::String(ref s) => {
            s.to_string()
        },
        _ => return None
    };
    let priority = match &y["priority"] {
        ref s => {
            match s.as_i64() {
                Some(u) => u,
                None => return None
            }
        }
    };
    let mappings = match &y["mappings"] {
        &Yaml::Array(ref ms) => {
            ms.iter()
                .map(|ref m| parse_mapping(&m))
                .filter(|m| m.is_some())
                .map(|m| m.unwrap())
                .collect()
        },
        _ => vec![]
    };
    Some(super::Context {
        id : id,
        priority : priority as u32,
        mappings : mappings
    })
}

pub fn parse_contexts(y : &Yaml) -> Vec<super::Context> {
    match &y["contexts"] {
        &Yaml::Array(ref contexts) => {
            contexts.iter()
                .map(|ref c| parse_context(&c))
                .filter(|c| c.is_some())
                .map(|c| c.unwrap())
                .collect()
        },
        _ => panic!()
    }
}

pub fn parse_constants(y : &Yaml) -> Vec<super::Constant> {
    let mut contants = vec![];
    match &y["actions"] {
        &Yaml::Array(ref actions) => {
            for action in actions {
                contants.push(super::Constant::from((super::Type::Action, action)));
            }
        },
        _ => ()
    }
    match &y["states"] {
        &Yaml::Array(ref states) => {
            for state in states {
                contants.push(super::Constant::from((super::Type::State, state)));
            }
        },
        _ => ()
    }
    match &y["ranges"] {
        &Yaml::Array(ref ranges) => {
            for range in ranges {
                contants.push(super::Constant::from((super::Type::Range, range)));
            }
        },
        _ => ()
    }
    contants
}

pub fn parse_contexts_str(contents : &str) -> Vec<super::Context> {
    let doc = &YamlLoader::load_from_str(contents).unwrap()[0];
    parse_contexts(doc)
}

pub fn parse_constants_str(contents : &str) -> Vec<super::Constant> {
    let doc = &YamlLoader::load_from_str(contents).unwrap()[0];
    parse_constants(doc)
}

pub fn parse_contexts_file(file : &str) -> Vec<super::Context> {
    let mut f = std::fs::File::open(file).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    parse_contexts_str(&s)
}

pub fn parse_constants_file(file : &str) -> Vec<super::Constant> {
    let mut f = std::fs::File::open(file).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    parse_constants_str(&s)
}
