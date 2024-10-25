use crate::{engine::contexts_dto::WritePermissionsDto, types::thread::Readonly};
use serde_json::{Map, Value};
use std::{collections::HashMap, sync::Arc};

use super::{
    cli::{Argument, Cli, Environment},
    contexts_dto::BootContextDto,
};

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
    pub definition: Option<String>,
    pub enricher: Option<Readonly<Enricher>>,
    pub other: Map<String, Value>,
}

#[derive(Debug)]
pub struct Install {
    pub arguments: Vec<Argument>,
    pub program: String,
    pub environments: Vec<Environment>,
}

impl Cli for Install {
    fn arguments(&self) -> Vec<Argument> {
        self.arguments.clone()
    }
    fn environments(&self) -> Vec<Environment> {
        self.environments.clone()
    }
    fn program(&self) -> String {
        self.program.clone()
    }
}

#[derive(Debug)]
pub struct Enricher {
    pub arguments: Vec<Argument>,
    pub program: String,
    pub install: Option<Install>,
    pub environments: Vec<Environment>,
}
impl Cli for Enricher {
    fn arguments(&self) -> Vec<Argument> {
        self.arguments.clone()
    }
    fn environments(&self) -> Vec<Environment> {
        self.environments.clone()
    }
    fn program(&self) -> String {
        self.program.clone()
    }
}

#[derive(Debug)]
pub struct Bootes {
    pub name: String,
    pub metadata: HashMap<String, Readonly<Metadata>>,
    pub edges: HashMap<String, Readonly<Edge>>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub enrichers: HashMap<String, Readonly<Enricher>>,
    pub arguments: HashMap<String, Readonly<Vec<String>>>,
    pub environments: HashMap<String, Readonly<HashMap<String, String>>>,
}

pub struct EnricherContext {
    pub bootes: BootContextDto,
    pub enricher: Readonly<Enricher>,
}
struct EdgeContext {
    key: String,
    value: Map<String, Value>,
    enricher: Readonly<Enricher>,
}

impl Bootes {
    pub fn enrichers_contexts(&self) -> Vec<EnricherContext> {
        let bootes_contexts = self.metadata.iter().map(|(metadata_key, metadata_value)| {
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
                            bootes: BootContextDto {
                                metadata: (metadata_key.clone(), metadata_lock.other.clone()),
                                write: WritePermissionsDto::edge(),
                                edge: Some((edge_context.key.clone(), edge_context.value.clone())),
                            },
                            enricher: edge_context.enricher.clone(),
                        };
                        if let Some(enricher) = metadata_lock.enricher.clone() {
                            if Arc::ptr_eq(&enricher, &edge_context.enricher) {
                                EnricherContext {
                                    bootes: BootContextDto {
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
                    .all(|context| context.bootes.write.metadata == false)
                {
                    if let Some(enricher) = metadata_lock.enricher.clone() {
                        enricher_contexts.push(EnricherContext {
                            bootes: BootContextDto {
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
        bootes_contexts.flatten().flatten().collect()
    }
}
