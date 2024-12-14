use common::{
    merge::{Merge, Overwrite},
    thread::Readonly,
};
use serde_json::{Map, Value};
use std::collections::HashMap;

use crate::{
    cli::{Arguments, Environment},
    contexts::{
        ContextDto, DefinitionContextDto, EdgeContext, ProcessorContext, WritePermissionsDto,
    },
    processor::{CleanProcessor, CleanProcessorOverride},
    reference_value::ReferencedValue,
};

use super::{
    edge::{CleanEdge, CleanEdgeOverride, Edge, EdgeOverride},
    foreign_definition::{CleanForeignDefinition, ForeignDefinition},
};

#[derive(Debug, Clone)]
pub struct Tag {
    pub label: Option<String>,
    pub tags: HashMap<String, Readonly<RefTag>>,
    pub other: Value,
}
impl Merge for Tag{
    fn merge(&mut self, other: &Self) {
        self.label.overwrite(&other.label);
        self.other.merge(&other.other);
        self.tags.merge(&other.tags);
    }
}
pub type RefTag = ReferencedValue<Tag>;



#[derive(Debug, Clone)]
pub struct Metadata<TEdge, TProcessor, TTag> {
    pub edges: HashMap<String, TEdge>,
    pub tags: HashMap<String, TTag>,
    pub other: Map<String, Value>,
    pub processor: Option<TProcessor>,
}
pub type CleanMetadata = Metadata<CleanEdgeOverride, CleanProcessorOverride, Readonly<RefTag>>;
impl <TEdge: Merge + Clone, TProcessor: Merge+Clone, TTag: Merge + Clone> Merge for  Metadata<TEdge, TProcessor, TTag> {
    fn merge(&mut self, other: &Self) {
        self.edges.merge(&other.edges);
        self.tags.merge(&other.tags);
        self.other.merge(&other.other);
        self.processor.merge(&other.processor);
    }
}

#[derive(Debug, Clone)]
pub struct Install<TArguments, TEnvironment> {
    pub arguments: Vec<TArguments>,
    pub program: String,
    pub environments: Vec<TEnvironment>,
    pub current_directory: Option<String>,
}
pub type CleanInstall = Install<Arguments, Environment>;
impl <TArguments: Merge + Clone, TEnvironment: Merge + Clone> Merge for Install<TArguments, TEnvironment> {
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.program = other.program.clone();
        self.environments.merge(&other.environments);
        self.current_directory.overwrite(&other.current_directory);
    }
}

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
                                processor_reference: edge_context.processor.r#ref.clone(),
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
                                        processor_reference: edge_context.processor.r#ref.clone(),
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
                                processor_reference: processor.r#ref.clone(),
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
