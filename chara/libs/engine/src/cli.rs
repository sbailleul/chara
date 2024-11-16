use std::{collections::HashMap, hash::Hash};

use types::thread::Readonly;

#[derive(Debug, Clone)]
pub enum Argument {
    Value(String),
    Reference {
        name: String,
        arguments: Readonly<Vec<String>>,
    },
}
impl PartialEq for Argument {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::Reference { name: name1, .. }, Self::Reference { name: name2, .. }) => {
                name1.eq(name2)
            }
            _ => false,
        }
    }
}
impl Eq for Argument {}
impl Hash for Argument {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Argument::Value(v) => v.hash(state),
            Argument::Reference { arguments, name } => {
                if let Ok(v) = arguments.read() {
                    v.hash(state);
                }
                name.hash(state);
            }
        }
    }
}

impl Argument {
    pub fn unwrap(&self) -> Vec<String> {
        match self {
            Argument::Value(arg) => vec![arg.clone()],
            Argument::Reference { arguments, .. } => {
                arguments.read().map_or(vec![], |args| args.clone())
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum Environment {
    Value(HashMap<String, String>),
    Reference {
        name: String,
        environment: Readonly<HashMap<String, String>>,
    },
}
impl PartialEq for Environment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::Reference{name: name0,..}, Self::Reference{name: name1,..}) => name0.eq(name1),
            _ => false,
        }
    }
}
impl Eq for Environment {}
impl Environment {
    pub fn unwrap(&self) -> HashMap<String, String> {
        match self {
            Environment::Value(value) => value.clone(),
            Environment::Reference{environment,..} => environment
                .read()
                .map_or(HashMap::new(), |value| value.clone()),
        }
    }
}
