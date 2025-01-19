use common::thread::{readonly, Readonly};
use engine::{
    cli::{DraftArguments, DraftEnvironments},
    definition::{
        definition::Definition,
        edge::{Edge, EdgeOverride},
    },
    reference_value::LazyRefOrValue,
};
use serde_json::{Map, Value};

use super::edge::EdgeBuilder;

pub struct EdgeOverrideBuilder {
    edge_override: EdgeOverride,
}

impl EdgeOverrideBuilder {
    pub fn new() -> Self {
        Self {
            edge_override: empty_edge_override(),
        }
    }
    pub fn with_arguments(&mut self, arguments: Vec<DraftArguments>) -> &mut Self {
        self.edge_override.arguments = arguments;
        self
    }
    pub fn with_environments(&mut self, environments: Vec<DraftEnvironments>) -> &mut Self {
        self.edge_override.environments = environments;
        self
    }
    pub fn with_edge(&mut self, edge: LazyRefOrValue<Edge>) -> &mut Self {
        self.edge_override.edge = edge;
        self
    }
    pub fn with_referenced_edge(&mut self, r#ref: &str, edge: Readonly<Edge>) -> &mut Self{
        self.edge_override.edge = LazyRefOrValue::to_referenced_value(r#ref.to_string(),edge);
        self
    }
    pub fn with_other(&mut self, other: Map<String, Value>) -> &mut Self {
        self.edge_override.other = other;
        self
    }
    pub fn with_definition(&mut self, definition: Definition) -> &mut Self {
        self.edge_override.definition = Some(definition);
        self
    }
    pub fn build(&mut self) -> EdgeOverride {
        let edge_override = self.edge_override.clone();
        self.edge_override = empty_edge_override();
        edge_override
    }
    
}

fn empty_edge_override() -> EdgeOverride {
    EdgeOverride {
        arguments: vec![],
        definition: None,
        edge: LazyRefOrValue::Ref("".to_string()),
        environments: vec![],
        other: Map::new(),
    }
}
