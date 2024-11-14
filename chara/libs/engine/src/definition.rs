use serde_json::{Map, Value};
use std::{collections::HashMap, hash::Hasher, sync::Arc};
use types::thread::Readonly;

use super::cli::{Argument, Environment};
use crate::contexts::{
    ContextDto, DefinitionContextDto, EdgeContext, ProcessorContext, WritePermissionsDto,
};
#[derive(Debug, PartialEq, Eq)]
pub enum DefinitionInput {
    File(String),
    Text(String),
    Value(Value),
    Processor(ProcessorOverride),
}

#[derive(Debug)]
pub struct Tag {
    pub label: Option<String>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub other: Value,
}
#[derive(Debug)]
pub struct EdgeOverride {
    pub arguments: Vec<Argument>,
    pub environments: Vec<Environment>,
    pub edge: Readonly<Edge>,
    pub other: Map<String, Value>,
    pub definition: Option<Definition>
}
#[derive(Debug)]
pub struct Metadata {
    pub edges: HashMap<String, EdgeOverride>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub other: Map<String, Value>,
    pub processor: Option<ProcessorOverride>,
}

#[derive(Debug)]
pub struct Edge {
    pub definition: Option<Readonly<ForeignDefinition>>,
    pub processor: Option<ProcessorOverride>,
    pub other: Map<String, Value>,
}

#[derive(Debug)]
pub struct Install {
    pub arguments: Vec<Argument>,
    pub program: String,
    pub environments: Vec<Environment>,
    pub current_directory: Option<String>,
}

#[derive(Debug)]
pub struct ForeignDefinition {
    pub input: DefinitionInput,
    pub output: Option<Definition>,
}
impl ForeignDefinition {
    pub fn input(input: DefinitionInput) -> Self {
        ForeignDefinition {
            input,
            output: None,
        }
    }
}

#[derive(Debug)]
pub struct Processor {
    pub arguments: Vec<Argument>,
    pub program: String,
    pub install: Option<Install>,
    pub environments: Vec<Environment>,
    pub current_directory: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessorOverride {
    pub arguments: Vec<Argument>,
    pub environments: Vec<Environment>,
    pub processor: Readonly<Processor>,
    pub reference: String,
}
impl ProcessorOverride {
    pub fn processor(processor: &Readonly<Processor>, reference: &String) -> Self {
        Self {
            arguments: vec![],
            environments: vec![],
            reference: reference.clone(),
            processor: processor.clone(),
        }
    }

    pub fn with(&self, arguments: Vec<Argument>, environments: Vec<Environment>) -> Self {
        let mut processor = self.clone();
        processor.arguments = [arguments, processor.arguments].concat();
        processor.environments = [environments, processor.environments].concat();
        processor
    }
}
impl PartialEq for ProcessorOverride {
    fn eq(&self, other: &Self) -> bool {
        self.arguments == other.arguments
            && self.environments == other.environments
            && Arc::ptr_eq(&self.processor, &other.processor)
    }
}
impl Eq for ProcessorOverride {}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Enrichment {
    pub edge: Option<Map<String, Value>>,
    pub metadata: Option<Map<String, Value>>,
}
#[derive(Debug)]
pub struct ProcessorResult {
    pub enrichment: Option<Enrichment>,
    pub definition: Option<Definition>,
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
                                processor: processor
                                    .with(edge_value.arguments.clone(), edge_value.environments.clone()),
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
