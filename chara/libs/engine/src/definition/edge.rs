use common::{merge::Merge, thread::Readonly};
use serde_json::{Map, Value};

use crate::{
    cli::{DraftArguments, DraftEnvironments}, draft::draft_definition::DraftDefinition, processor::DraftProcessorOverride, reference_value::LazyRefOrValue
};

use super::foreign_definition::ForeignDefinition;

#[derive(Debug, Clone)]
pub struct Edge {
    pub definition: Option<Readonly<ForeignDefinition>>,
    pub processor: Option<DraftProcessorOverride>,
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
    pub arguments: Vec<DraftArguments>,
    pub environments: Vec<DraftEnvironments>,
    pub edge: LazyRefOrValue<Edge>,
    pub other: Map<String, Value>,
    pub definition: Option<DraftDefinition>,
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
impl EdgeOverride {
    pub fn edge(edge: LazyRefOrValue<Edge>) -> Self {
        Self {
            arguments: vec![],
            definition: None,
            edge,
            environments: vec![],
            other: Map::new(),
        }
    }
}
