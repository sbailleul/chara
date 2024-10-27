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
    Http(HttpDefinition),
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
    pub enricher: Option<Readonly<Enricher>>,
}

#[derive(Debug)]
pub struct Edge {
    pub definition: Option<Readonly<ForeignDefinition>>,
    pub enricher: Option<Readonly<Enricher>>,
    pub other: Map<String, Value>,
}

#[derive(Debug)]
pub struct Install {
    pub arguments: Vec<Argument>,
    pub program: String,
    pub environments: Vec<Environment>,
}

#[derive(Debug)]
pub struct HttpDefinition {
    pub arguments: Vec<Argument>,
    pub environments: Vec<Environment>,
    pub uri: String,
}

#[derive(Debug)]
pub struct ForeignDefinition {
    pub input: DefinitionInput,
    pub output: Option<Definition>,
}

#[derive(Debug)]
pub struct Enricher {
    pub arguments: Vec<Argument>,
    pub program: String,
    pub install: Option<Install>,
    pub environments: Vec<Environment>,
}

#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub metadata: HashMap<String, Readonly<Metadata>>,
    pub edges: HashMap<String, Readonly<Edge>>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub enrichers: HashMap<String, Readonly<Enricher>>,
    pub arguments: HashMap<String, Readonly<Vec<String>>>,
    pub environments: HashMap<String, Readonly<HashMap<String, String>>>,
    pub foreign_definitions: HashMap<String, Readonly<ForeignDefinition>>,
}

pub struct EnricherContext {
    pub definition: DefinitionContextDto,
    pub enricher: Readonly<Enricher>,
}
struct EdgeContext {
    key: String,
    value: Map<String, Value>,
    enricher: Readonly<Enricher>,
}

impl Definition {
    pub fn enrichers_contexts(&self) -> Vec<EnricherContext> {
        let definition_contexts = self.metadata.iter().map(|(metadata_key, metadata_value)| {
            metadata_value.read().ok().map(|metadata_lock| {
                let edge_contexts = metadata_lock
                    .edges
                    .clone()
                    .into_iter()
                    .map(|(edge_key, edge_value)| {
                        edge_value.read().ok().and_then(|edge_lock| {
                            edge_lock.enricher.clone().map(|enricher| EdgeContext {
                                key: edge_key.clone(),
                                value: edge_lock.other.clone(),
                                enricher: enricher.clone(),
                            })
                        })
                    })
                    .flatten();
                let mut enricher_contexts = edge_contexts
                    .map(|edge_context| {
                        let context_without_metadata = EnricherContext {
                            definition: DefinitionContextDto {
                                metadata: (metadata_key.clone(), metadata_lock.other.clone()),
                                write: WritePermissionsDto::edge(),
                                edge: Some((edge_context.key.clone(), edge_context.value.clone())),
                            },
                            enricher: edge_context.enricher.clone(),
                        };
                        if let Some(enricher) = metadata_lock.enricher.clone() {
                            if Arc::ptr_eq(&enricher, &edge_context.enricher) {
                                EnricherContext {
                                    definition: DefinitionContextDto {
                                        edge: Some((edge_context.key, edge_context.value)),
                                        metadata: (
                                            metadata_key.clone(),
                                            metadata_lock.other.clone(),
                                        ),
                                        write: WritePermissionsDto::both(),
                                    },
                                    enricher: enricher.clone(),
                                }
                            } else {
                                context_without_metadata
                            }
                        } else {
                            context_without_metadata
                        }
                    })
                    .collect::<Vec<EnricherContext>>();
                if enricher_contexts
                    .iter()
                    .all(|context| context.definition.write.metadata == false)
                {
                    if let Some(enricher) = metadata_lock.enricher.clone() {
                        enricher_contexts.push(EnricherContext {
                            definition: DefinitionContextDto {
                                edge: None,
                                metadata: (metadata_key.clone(), metadata_lock.other.clone()),
                                write: WritePermissionsDto::metadata(),
                            },
                            enricher: enricher.clone(),
                        });
                    }
                }
                enricher_contexts
            })
        });
        definition_contexts.flatten().flatten().collect()
    }
}
