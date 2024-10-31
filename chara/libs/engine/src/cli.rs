use std::{collections::HashMap, hash::Hash, sync::Arc};

use types::thread::Readonly;

#[derive(Debug, Clone)]
pub enum Argument {
    Value(String),
    Reference(Readonly<Vec<String>>),
}
impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::Reference(l0), Self::Reference(r0)) => Arc::ptr_eq(&l0, &r0),
            _ => false,
        }
    }
}
impl Eq for Argument {}
impl Hash for Argument {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Argument::Value(v) => v.hash(state),
            Argument::Reference(arc) => {
                if let Ok(v) = arc.read(){
                    v.hash(state);
                } 
            }
        }
    }
}

impl Argument {
    pub fn unwrap(&self) -> Vec<String> {
        match self {
            Argument::Value(arg) => vec![arg.clone()],
            Argument::Reference(reference) => reference.read().map_or(vec![], |args| args.clone()),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Environment {
    Value(HashMap<String, String>),
    Reference(Readonly<HashMap<String, String>>),
}
impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::Reference(l0), Self::Reference(r0)) => Arc::ptr_eq(&l0, &r0),
            _ => false,
        }
    }
}
impl Eq for Environment {}
impl Environment {
    pub fn unwrap(&self) -> HashMap<String, String> {
        match self {
            Environment::Value(value) => value.clone(),
            Environment::Reference(reference) => reference
                .read()
                .map_or(HashMap::new(), |value| value.clone()),
        }
    }
}
