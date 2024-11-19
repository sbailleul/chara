use serde_json::{Map, Value};
use std::{collections::HashMap, hash::Hasher, sync::Arc};
use types::thread::Readonly;

use super::cli::{Argument, Environment};
use crate::{contexts::{
    ContextDto, DefinitionContextDto, EdgeContext, ProcessorContext, WritePermissionsDto,
}, processor::{Processor, ProcessorOverride}};
#[derive(Debug, PartialEq, Eq)]
pub enum DefinitionInput {
    File(String),
    Text(String),
    Value(Value),
    Processor(ProcessorOverride),
}

#[derive(Debug)]
pub struct Tag {
    pub reference: String,
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
    pub input: Option<DefinitionInput>,
    pub output: Option<Definition>,
}
impl ForeignDefinition {
    pub fn input(input: DefinitionInput) -> Self {
        ForeignDefinition {
            input: Some(input),
            output: None,
        }
    }
    pub fn output(output: Definition) -> Self {
        ForeignDefinition {
            input: None,
            output: Some(output),
        }
    }
}



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
