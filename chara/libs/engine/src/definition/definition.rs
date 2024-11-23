use common::{merge::{Merge, Overwrite}, thread::Readonly};
use serde_json::{Map, Value};
use std::{collections::HashMap, hash::Hasher};

use crate::{
    cli::{Argument, Environment},
    contexts::{
        ContextDto, DefinitionContextDto, EdgeContext, ProcessorContext, WritePermissionsDto,
    },
    processor::{Processor, ProcessorOverride},
};

use super::foreign_definition::ForeignDefinition;

#[derive(Debug, Clone)]
pub struct Tag {
    pub reference: String,
    pub label: Option<String>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub other: Value,
}

impl Merge for Tag{
    fn merge(&mut self, other: &Self) {
        self.label.overwrite(&other.label);
        self.other.merge(&other.other);
        self.reference = other.reference.clone();
        self.tags.merge(&other.tags);
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
impl Merge for EdgeOverride{
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.environments.merge(&other.environments);
        self.edge.merge(&other.edge);
        self.other.merge(&other.other);
        self.definition.merge(&other.definition);
    }
}
#[derive(Debug, Clone)]
pub struct Metadata {
    pub edges: HashMap<String, EdgeOverride>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub other: Map<String, Value>,
    pub processor: Option<ProcessorOverride>,
}
impl Merge for Metadata{
    fn merge(&mut self, other: &Self) {
        self.edges.merge(&other.edges);
        self.tags.merge(&other.tags);
        self.other.merge(&other.other);
        self.processor.merge(&other.processor);
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub definition: Option<Readonly<ForeignDefinition>>,
    pub processor: Option<ProcessorOverride>,
    pub other: Map<String, Value>,
}

impl Merge for Edge{
    fn merge(&mut self, other: &Self) {
        self.definition.merge(&other.definition);
        self.processor.merge(&other.processor);
        self.other.merge(&other.other);
    }
}

#[derive(Debug, Clone)]
pub struct Install {
    pub arguments: Vec<Argument>,
    pub program: String,
    pub environments: Vec<Environment>,
    pub current_directory: Option<String>,
}

impl Merge for Install{
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.program = other.program.clone();
        self.environments.merge(&other.environments);
        self.current_directory.overwrite(&other.current_directory);
    }
}


#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub location: Option<String>,
    pub metadata: HashMap<String, Readonly<Metadata>>,
    pub edges: HashMap<String, Readonly<Edge>>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub processors: HashMap<String, Readonly<Processor>>,
    pub arguments: HashMap<String, Readonly<Vec<String>>>,
    pub environments: HashMap<String, Readonly<HashMap<String, String>>>,
    pub foreign_definitions: HashMap<String, Readonly<ForeignDefinition>>,
}

impl Definition {
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
                                processor_reference: edge_context.processor.reference.clone(),
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
                                        processor_reference: edge_context
                                            .processor
                                            .reference
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
                    if let Some(processor) = metadata_lock.processor.clone() {
                        processor_contexts.push(ProcessorContext {
                            definition: DefinitionContextDto {
                                processor_reference: processor.reference.clone(),
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
