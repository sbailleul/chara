use std::{collections::HashMap, process::Command};

use crate::types::thread::Readonly;

pub trait Cli {
    fn arguments(&self) -> Vec<Argument>;
    fn environments(&self) -> Vec<Environment>;
    fn program(&self) -> String;
    fn command(&self) -> Command {
        let mut cmd = Command::new(self.program());
        cmd.args(self.flatten_arguments())
            .envs(self.flatten_environments());
        cmd
    }

    fn flatten_arguments(&self) -> Vec<String> {
        self.arguments()
            .iter()
            .map(|arg| arg.unwrap())
            .flatten()
            .collect()
    }
    fn flatten_environments(&self) -> HashMap<String, String> {
        self.environments()
            .iter()
            .map(|environment| environment.unwrap())
            .flatten()
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum Argument {
    Value(String),
    Reference(Readonly<Vec<String>>),
}
impl Argument {
    fn unwrap(&self) -> Vec<String> {
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
    fn unwrap(&self) -> HashMap<String, String> {
        match self {
            Environment::Value(value) => value.clone(),
            Environment::Reference(reference) => reference
                .read()
                .map_or(HashMap::new(), |value| value.clone()),
        }
    }
}
