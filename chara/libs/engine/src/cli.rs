use std::{collections::HashMap, process::Command};

use types::thread::Readonly;


#[derive(Debug, Clone)]
pub enum Argument {
    Value(String),
    Reference(Readonly<Vec<String>>),
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
