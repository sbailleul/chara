use std::collections::HashMap;

use common::{merge::Merge, thread::Readonly};
use serde_json::{Map, Value};

use crate::{
    cli::{Arguments, Environment},
    definition::{
        definition::{Definition, Install, Metadata, RefTag, Tag},
        edge::{Edge, EdgeOverride},
        foreign_definition::ForeignDefinition,
        input::DefinitionInput,
    },
    processor::{Processor, ProcessorOverride},
    reference_value::LazyRefValue,
};
pub type DraftArguments = LazyRefValue<Vec<String>>;
pub type DraftEnvironments = LazyRefValue<HashMap<String, String>>;
pub type DraftDefinitionInput = DefinitionInput<DraftProcessorOverride>;
pub type DraftForeignDefinition = ForeignDefinition<DraftDefinitionInput>;
pub type DraftInstall = Install<DraftArguments, DraftEnvironments>;

pub type DraftProcessor = Processor<DraftArguments, DraftInstall, DraftEnvironments>;
pub type DraftProcessorOverride =
    ProcessorOverride<DraftArguments, DraftEnvironments, LazyRefValue<DraftProcessor>>;

impl DraftProcessorOverride {
    pub fn processor(processor: LazyRefValue<DraftProcessor>) -> Self {
        Self {
            arguments: vec![],
            environments: vec![],
            processor,
        }
    }
}

pub type DraftEdge = Edge<DraftProcessorOverride, DraftForeignDefinition>;

pub type DraftEdgeOverride = EdgeOverride<DraftArguments, DraftEnvironments, LazyRefValue<DraftEdge>>;
impl DraftEdgeOverride {
    pub fn edge(edge: LazyRefValue<DraftEdge>) -> Self {
        Self {
            arguments: vec![],
            definition: None,
            edge,
            environments: vec![],
            other: Map::new(),
        }
    }
}

pub type DraftMetadata = Metadata<DraftEdgeOverride, DraftProcessorOverride, LazyRefValue<RefTag>>;

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
