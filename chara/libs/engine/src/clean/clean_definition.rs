use common::{
    merge::{Merge, Overwrite},
    thread::Readonly,
};
use std::{collections::HashMap, sync::Arc};

use crate::{
    cli::{Arguments, Environment},
    contexts::{
        ContextDto, DefinitionContextDto, EdgeContext, ProcessorContext, WritePermissionsDto,
    },
    definition::{
        edge::{Edge, EdgeOverride},
        foreign_definition::ForeignDefinition,
        input::DefinitionInput,
        install::Install,
        metadata::Metadata,
        tag::Tag,
    },
    processor::{Processor, ProcessorOverride},
    reference_value::ReferencedValue,
};

pub type CleanEdge = Edge<CleanProcessorOverride, CleanForeignDefinition>;
pub type CleanDefinitionInput = DefinitionInput<CleanProcessorOverride>;

pub type CleanForeignDefinition = ForeignDefinition<CleanDefinitionInput>;
pub type CleanEdgeOverride = EdgeOverride<Arguments, Environment, Readonly<CleanEdge>>;
pub type CleanInstall = Install<Arguments, Environment>;
pub type RefTag = ReferencedValue<Tag>;
pub type CleanMetadata = Metadata<CleanEdgeOverride, CleanProcessorOverride, Readonly<RefTag>>;

pub type CleanProcessor = Processor<Arguments, CleanInstall, Environment>;
pub type CleanProcessorOverride =
    ProcessorOverride<Arguments, Environment, ReferencedValue<Readonly<CleanProcessor>>>;
impl PartialEq for CleanProcessorOverride {
    fn eq(&self, other: &Self) -> bool {
        self.arguments == other.arguments
            && self.environments == other.environments
            && Arc::ptr_eq(&self.processor.value, &other.processor.value)
    }
}
impl Eq for CleanProcessorOverride {}

#[derive(Debug, Clone)]
pub struct CleanDefinition {
    pub parent: Option<Readonly<CleanDefinition>>,
    pub name: String,
    pub id: String,
    pub location: Option<String>,
    pub metadata: HashMap<String, Readonly<CleanMetadata>>,
    pub edges: HashMap<String, Readonly<CleanEdge>>,
    pub tags: HashMap<String, Readonly<RefTag>>,
    pub processors: HashMap<String, Readonly<CleanProcessor>>,
    pub arguments: HashMap<String, Readonly<Vec<String>>>,
    pub environments: HashMap<String, Readonly<HashMap<String, String>>>,
    pub foreign_definitions: HashMap<String, Readonly<CleanForeignDefinition>>,
}
impl Merge for CleanDefinition {
    fn merge(&mut self, other: &CleanDefinition) {
        self.id = other.id.clone();
        self.arguments.merge(&other.arguments);
        self.environments.merge(&other.environments);
        self.edges.merge(&other.edges);
        self.foreign_definitions.merge(&other.foreign_definitions);
        self.location.overwrite(&other.location);
        self.metadata.merge(&other.metadata);
        self.processors.merge(&other.processors);
        self.tags.merge(&other.tags);
    }
}

impl CleanDefinition {
    fn find_edge(&self, reference: &String) -> Option<Readonly<CleanEdge>> {
        let segments = reference.split("/").collect::<Vec<&str>>();
        self.find_edge_by_segment(segments)
    }
    fn find_edge_by_segment(&self, mut segments: Vec<&str>) -> Option<Readonly<CleanEdge>> {
        if let Some(segment) = segments.pop() {
            if let Some(edge) = self.edges.get(segment) {
                return Some(edge.clone());
            }
        }
        if segments.len() > 0 {
            if let Some(Ok(parent)) = self.parent.as_ref().map(|parent| parent.read()) {
                return parent.find_edge_by_segment(segments);
            }
        }
        None
    }

    pub fn processors_contexts(&self) -> Vec<ProcessorContext> {
        let definition_contexts = self.metadata.iter().map(|(metadata_key, metadata_value)| {
            metadata_value.read().ok().map(|metadata_lock| {
                let edge_contexts = metadata_lock
                    .edges
                    .iter()
                    .map(|(edge_key, edge_value)| {
                        edge_value.edge.read().ok().and_then(|edge_lock| {
                            edge_lock.processor.as_ref().map(|processor| EdgeContext {
                                key: edge_key.clone(),
                                value: edge_lock.other.clone(),
                                processor: processor.from_with(
                                    edge_value.arguments.clone(),
                                    edge_value.environments.clone(),
                                ),
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
                        if let Some(processor) = metadata_lock.processor.clone() {
                            if processor == edge_context.processor {
                                ProcessorContext {
                                    definition: DefinitionContextDto {
                                        processor_reference: edge_context.processor.processor.r#ref.clone(),
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
                    if let Some(processor) = metadata_lock.processor.clone() {
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
