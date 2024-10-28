use serde_json::{Map, Value};
use std::{collections::HashMap, sync::Arc};
use types::thread::Readonly;

use crate::contexts_dto::WritePermissionsDto;

use super::{
    cli::{Argument, Environment},
    contexts_dto::DefinitionContextDto,
};
#[derive(Debug)]
pub enum DefinitionInput {
    File(String),
    Text(String),
    Value(Value),
    Processor(Readonly<Processor>),
}

#[derive(Debug)]
pub struct Tag {
    pub label: Option<String>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub other: Map<String, Value>,
}
#[derive(Debug)]
pub struct Metadata {
    pub edges: HashMap<String, Readonly<Edge>>,
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
}

#[derive(Debug, Clone)]
pub struct ProcessorOverride {
    pub arguments: Vec<Argument>,
    pub environments: Vec<Environment>,
    pub processor: Readonly<Processor>,
}
impl ProcessorOverride {
    pub fn processor(processor: &Readonly<Processor>) -> Self {
        Self {
            arguments: vec![],
            environments: vec![],
            processor: processor.clone(),
        }
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
    pub metadata: HashMap<String, Readonly<Metadata>>,
    pub edges: HashMap<String, Readonly<Edge>>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub processors: HashMap<String, Readonly<Processor>>,
    pub arguments: HashMap<String, Readonly<Vec<String>>>,
    pub environments: HashMap<String, Readonly<HashMap<String, String>>>,
    pub foreign_definitions: HashMap<String, Readonly<ForeignDefinition>>,
}

pub struct ProcessorContext {
    pub definition: DefinitionContextDto,
    pub processor: ProcessorOverride,
}
struct EdgeContext {
    key: String,
    value: Map<String, Value>,
    definition: Option<Readonly<ForeignDefinition>>,
    processor: ProcessorOverride,
}

impl Definition {
    pub fn processors_contexts(&self) -> Vec<ProcessorContext> {
        let definition_contexts = self.metadata.iter().map(|(metadata_key, metadata_value)| {
            metadata_value.read().ok().map(|metadata_lock| {
                let edge_contexts = metadata_lock
                    .edges
                    .clone()
                    .into_iter()
                    .map(|(edge_key, edge_value)| {
                        edge_value.read().ok().and_then(|edge_lock| {
                            edge_lock.processor.as_ref().map(|processor| EdgeContext {
                                key: edge_key.clone(),
                                value: edge_lock.other.clone(),
                                processor: processor.clone(),
                                definition: edge_lock.definition.clone(),
                            })
                        })
                    })
                    .flatten();
                let mut processor_contexts = edge_contexts
                    .map(|edge_context| {
                        let context_without_metadata = ProcessorContext {
                            definition: DefinitionContextDto {
                                metadata: (metadata_key.clone(), metadata_lock.other.clone()),
                                write: WritePermissionsDto::edge(),
                                edge: Some((edge_context.key.clone(), edge_context.value.clone())),
                            },
                            processor: edge_context.processor.clone(),
                        };
                        if let Some(processor) = metadata_lock.processor.clone() {
                            if processor == edge_context.processor {
                                ProcessorContext {
                                    definition: DefinitionContextDto {
                                        edge: Some((edge_context.key, edge_context.value)),
                                        metadata: (
                                            metadata_key.clone(),
                                            metadata_lock.other.clone(),
                                        ),
                                        write: WritePermissionsDto::both(),
                                    },
                                    processor: processor.clone(),
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
                                edge: None,
                                metadata: (metadata_key.clone(), metadata_lock.other.clone()),
                                write: WritePermissionsDto::metadata(),
                            },
                            processor: processor.clone(),
                        });
                    }
                }
                processor_contexts
            })
        });
        definition_contexts.flatten().flatten().collect()
    }
}
