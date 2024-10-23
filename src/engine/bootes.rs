use crate::{engine::contexts_dto::WritePermissionsDto, types::thread::Readonly};
use serde_json::{de::Read, Map, Value};
use std::{collections::HashMap, sync::Arc, vec};

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

struct EnricherContext {
    bootes: BootContextDto,
    enricher: Readonly<Enricher>,
}
struct EdgeContext {
    key: String,
    value: Map<String, Value>,
    enricher: Readonly<Enricher>,
}

impl Bootes {
    fn enrichers_contexts(&self) -> Vec<EnricherContext> {
        self.metadata
            .iter()
            .map(|(metadata_key, metadata_value)| {
                metadata_value.read().ok().map(|metadata_lock| {
                    let edge_contexts = metadata_lock.edges.iter().map(|(edge_key, edge_value)| {
                        edge_value.read().ok().and_then(|edge_lock| {
                            if let Some(enricher) = edge_lock.enricher {
                                Some(EdgeContext {
                                    key: edge_key.clone(),
                                    value: edge_lock.other.clone(),
                                    enricher: enricher.clone(),
                                })
                            } else {
                                None
                            }
                        })
                    });
                    edge_contexts.map(|edge_context| {
                        if let Some(enricher) = metadata_lock.enricher {
                            if Arc::ptr_eq(&enricher, &edge_context.enricher) {
                                Some(EnricherContext {
                                    bootes: BootContextDto {
                                        edge: Some((edge_context.key, edge_context.value)),
                                        metadata: (
                                            metadata_key.clone(),
                                            metadata_lock.other.clone(),
                                        ),
                                        write: WritePermissionsDto {
                                            edge: true,
                                            metadata: true,
                                        },
                                    },
                                    enricher: enricher.clone(),
                                })
                            } else {
                                Some(EnricherContext {
                                    bootes: BootContextDto {
                                        metadata: (
                                            metadata_key.clone(),
                                            metadata_lock.other.clone(),
                                        ),
                                        write: WritePermissionsDto {
                                            edge: false,
                                            metadata: true,
                                        },
                                        edge: None,
                                    },
                                    enricher: enricher.clone(),
                                })
                            }
                        } else {
                            None
                        }
                    })
                })
            })
            .flatten()
            .collect()
    }
}
