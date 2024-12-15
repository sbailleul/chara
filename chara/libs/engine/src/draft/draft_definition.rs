use std::collections::HashMap;

use common::{
    merge::{Merge, Overwrite},
    thread::Readonly,
};
use serde_json::Map;

use crate::{
    clean::clean_definition::RefTag,
    definition::{
        edge::{Edge, EdgeOverride},
        foreign_definition::ForeignDefinition,
        input::DefinitionInput,
        install::Install,
        metadata::Metadata,
    },
    processor::{Processor, ProcessorOverride},
    reference_value::{LazyRef, LazyRefOrValue},
};
pub type DraftArguments = LazyRefOrValue<Vec<String>>;
pub type DraftEnvironments = LazyRefOrValue<HashMap<String, String>>;
pub type DraftDefinitionInput = DefinitionInput<DraftProcessorOverride>;
pub type DraftForeignDefinition = ForeignDefinition<DraftDefinitionInput>;
pub type DraftInstall = Install<DraftArguments, DraftEnvironments>;

pub type DraftProcessor = Processor<DraftArguments, DraftInstall, DraftEnvironments>;
pub type DraftProcessorOverride =
    ProcessorOverride<DraftArguments, DraftEnvironments, Option<LazyRef<DraftProcessor>>>;

impl DraftProcessorOverride {
    pub fn processor(processor: LazyRef<DraftProcessor>) -> Self {
        Self {
            arguments: vec![],
            environments: vec![],
            processor: Some(processor),
        }
    }
}

pub type DraftEdge = Edge<DraftProcessorOverride, DraftForeignDefinition>;

pub type DraftEdgeOverride =
    EdgeOverride<DraftArguments, DraftEnvironments, LazyRefOrValue<DraftEdge>>;
impl DraftEdgeOverride {
    pub fn edge(edge: LazyRefOrValue<DraftEdge>) -> Self {
        Self {
            arguments: vec![],
            definition: None,
            edge,
            environments: vec![],
            other: Map::new(),
        }
    }
}

pub type DraftMetadata =
    Metadata<DraftEdgeOverride, DraftProcessorOverride, LazyRefOrValue<RefTag>>;

#[derive(Debug, Clone)]
pub struct DraftDefinition {
    pub name: String,
    pub id: String,
    pub location: Option<String>,
    pub metadata: HashMap<String, Readonly<DraftMetadata>>,
    pub edges: HashMap<String, Readonly<DraftEdge>>,
    pub tags: HashMap<String, Readonly<RefTag>>,
    pub processors: HashMap<String, Readonly<DraftProcessor>>,
    pub arguments: HashMap<String, Readonly<Vec<String>>>,
    pub environments: HashMap<String, Readonly<HashMap<String, String>>>,
    pub foreign_definitions: HashMap<String, Readonly<DraftForeignDefinition>>,
}

impl Merge for DraftDefinition {
    fn merge(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.id = other.id.clone();
        self.location.overwrite(&other.location);
        self.metadata.merge(&other.metadata);
        self.edges.merge(&other.edges);
        self.tags.merge(&other.tags);
        self.processors.merge(&other.processors);
        self.arguments.merge(&other.arguments);
        self.environments.merge(&other.environments);
        self.foreign_definitions.merge(&other.foreign_definitions);
    }
}
