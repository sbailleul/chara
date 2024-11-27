use common::{merge::Merge, thread::Readonly};
use serde_json::{Map, Value};

use crate::{cli::{Argument, Environment}, processor::ProcessorOverride};

use super::{definition::Definition, foreign_definition::ForeignDefinition};

#[derive(Debug, Clone)]
pub struct Edge {
    pub definition: Option<Readonly<ForeignDefinition>>,
    pub processor: Option<ProcessorOverride>,
    pub other: Map<String, Value>,
}

impl Merge for Edge {
    fn merge(&mut self, other: &Self) {
        self.definition.merge(&other.definition);
        self.processor.merge(&other.processor);
        self.other.merge(&other.other);
    }
}


#[derive(Debug, Clone)]
pub struct EdgeOverride {
    pub arguments: Vec<Argument>,
    pub environments: Vec<Environment>,
    pub edge: Readonly<Edge>,
    pub other: Map<String, Value>,
    pub definition: Option<Definition>,
}

impl Merge for EdgeOverride {
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.environments.merge(&other.environments);
        self.edge.merge(&other.edge);
        self.other.merge(&other.other);
        self.definition.merge(&other.definition);
    }
}