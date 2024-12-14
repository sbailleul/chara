use common::{merge::Merge, thread::Readonly};
use serde_json::{Map, Value};

use crate::{
    cli::{Arguments, Environment},
    processor::CleanProcessorOverride,
};

use super::{
    definition::Definition,
    foreign_definition::{CleanForeignDefinition, ForeignDefinition},
    input::CleanDefinitionInput,
};

#[derive(Debug, Clone)]
pub struct Edge<TProcessorOverride, TForeignDefinition> {
    pub definition: Option<Readonly<TForeignDefinition>>,
    pub processor: Option<TProcessorOverride>,
    pub other: Map<String, Value>,
}

pub type CleanEdge = Edge<CleanProcessorOverride, CleanForeignDefinition>;

impl Merge for CleanEdge {
    fn merge(&mut self, other: &Self) {
        self.definition.merge(&other.definition);
        self.processor.merge(&other.processor);
        self.other.merge(&other.other);
    }
}

#[derive(Debug, Clone)]
pub struct EdgeOverride<TArguments, TEnvironment, TEdge> {
    pub arguments: Vec<TArguments>,
    pub environments: Vec<TEnvironment>,
    pub edge: TEdge,
    pub other: Map<String, Value>,
    pub definition: Option<Definition>,
}
pub type CleanEdgeOverride = EdgeOverride<Arguments, Environment, Readonly<CleanEdge>>;

impl Merge for CleanEdgeOverride {
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.environments.merge(&other.environments);
        self.edge.merge(&other.edge);
        self.other.merge(&other.other);
        self.definition.merge(&other.definition);
    }
}
