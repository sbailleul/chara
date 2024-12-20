use std::{collections::HashMap, sync::Arc};

use common::{
    collection::map,
    merge::{Merge, Overwrite},
    thread::Readonly,
};
use serde_json::Map;

use crate::{
    cli::Arguments,
    contexts::{
        ContextDto, DefinitionContextDto, EdgeContext, ProcessorContext, WritePermissionsDto,
    },
    definition::{
        edge::{Edge, EdgeOverride},
        foreign_definition::ForeignDefinition,
        input::DefinitionInput,
        install::Install,
        metadata::Metadata,
        tag::RefTag,
    },
    processor::{Processor, ProcessorOverride},
    reference_value::{LazyRef, LazyRefOrValue, RefOrValue, ReferencedValue},
};
pub type DraftArguments = LazyRefOrValue<Vec<String>>;

pub type DraftEnvironments = LazyRefOrValue<HashMap<String, String>>;
pub type DraftDefinitionInput = DefinitionInput<DraftProcessorOverride>;

impl DraftDefinitionInput {
    pub fn to_defined(&self) -> Option<DefinedDefinitionInput> {
        match self {
            DefinitionInput::File(file) => Some(DefinedDefinitionInput::File(file.clone())),
            DefinitionInput::Text(txt) => Some(DefinedDefinitionInput::Text(txt.clone())),
            DefinitionInput::Value(value) => Some(DefinedDefinitionInput::Value(value.clone())),
            DefinitionInput::Processor(processor) => processor
                .map()
                .map(|processor| DefinedDefinitionInput::Processor(processor)),
        }
    }
}
pub type DefinedDefinitionInput = DefinitionInput<DefinedProcessorOverride>;
pub type DraftForeignDefinition = ForeignDefinition<DraftDefinitionInput>;

pub type DraftProcessor = Processor<DraftArguments, DraftEnvironments>;
pub type DraftProcessorOverride =
    ProcessorOverride<DraftArguments, DraftEnvironments, Option<LazyRef<DraftProcessor>>>;
pub type DefinedProcessorOverride =
    ProcessorOverride<DraftArguments, DraftEnvironments, ReferencedValue<Readonly<DraftProcessor>>>;
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
    pub parent: Option<Readonly<DraftDefinition>>,
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
impl DraftDefinition {
    pub fn processors_contexts(&self) -> Vec<ProcessorContext> {
        let definition_contexts = self.metadata.iter().map(|(metadata_key, metadata_value)| {
            metadata_value.read().ok().map(|metadata_lock| {
                let edge_contexts = metadata_lock
                    .edges
                    .iter()
                    .map(|(edge_key, edge_value)| {
                        edge_value.edge.value().and_then(|edge| {
                            edge.processor
                                .as_ref()
                                .and_then(|processor| processor.map())
                                .map(|processor| EdgeContext {
                                    key: edge_key.clone(),
                                    value: edge.other.clone(),
                                    processor: processor.clone(),
                                })
                        })
                    })
                    .flatten();
                let mut processor_contexts = edge_contexts
                    .map(|edge_context| {
                        let context_without_metadata = ProcessorContext {
                            definition: DefinitionContextDto {
                                location: self.location.clone(),
                                metadata: ContextDto::new(
                                    metadata_key.clone(),
                                    metadata_lock.other.clone(),
                                ),
                                processor_reference: edge_context.processor.processor.r#ref.clone(),
                                write: WritePermissionsDto::edge(),
                                edge: Some(ContextDto::new(
                                    edge_context.key.clone(),
                                    edge_context.value.clone(),
                                )),
                            },
                            processor: edge_context.processor.clone(),
                            metadata: metadata_value.clone(),
                        };
                        if let Some(processor) = metadata_lock
                            .processor
                            .clone()
                            .and_then(|processor| processor.map())
                        {
                            if processor == edge_context.processor {
                                ProcessorContext {
                                    definition: DefinitionContextDto {
                                        processor_reference: edge_context
                                            .processor
                                            .processor
                                            .r#ref
                                            .clone(),
                                        location: self.location.clone(),
                                        edge: Some(ContextDto::new(
                                            edge_context.key,
                                            edge_context.value,
                                        )),
                                        metadata: ContextDto::new(
                                            metadata_key.clone(),
                                            metadata_lock.other.clone(),
                                        ),
                                        write: WritePermissionsDto::both(),
                                    },
                                    processor: processor.clone(),
                                    metadata: metadata_value.clone(),
                                }
                            } else {
                                context_without_metadata
                            }
                        } else {
                            context_without_metadata
                        }
                    })
                    .collect::<Vec<ProcessorContext>>();
                if processor_contexts
                    .iter()
                    .all(|context| context.definition.write.metadata == false)
                {
                    if let Some(processor) = metadata_lock
                        .processor
                        .clone()
                        .and_then(|processor| processor.map())
                    {
                        processor_contexts.push(ProcessorContext {
                            definition: DefinitionContextDto {
                                processor_reference: processor.processor.r#ref.clone(),
                                location: self.location.clone(),
                                edge: None,
                                metadata: ContextDto::new(
                                    metadata_key.clone(),
                                    metadata_lock.other.clone(),
                                ),
                                write: WritePermissionsDto::metadata(),
                            },
                            processor: processor.clone(),
                            metadata: metadata_value.clone(),
                        });
                    }
                }
                processor_contexts
            })
        });
        definition_contexts.flatten().flatten().collect()
    }
}
