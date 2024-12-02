use std::{collections::HashMap, hash::Hash};

use common::thread::Readonly;


#[derive(Debug, Clone)]
pub enum Arguments {
    Value(Vec<String>),
    Reference {
        name: String,
        arguments: Readonly<Vec<String>>,
    },
}
impl PartialEq for Arguments {
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
impl Eq for Arguments {}
impl Hash for Arguments {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Arguments::Value(v) => v.hash(state),
            Arguments::Reference { arguments, name } => {
                if let Ok(v) = arguments.read() {
                    v.hash(state);
                }
                name.hash(state);
            }
        }
    }
}

impl Arguments {
    pub fn unwrap(&self) -> Vec<String> {
        match self {
            Arguments::Value(arg) => arg.clone(),
            Arguments::Reference { arguments, .. } => {
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
