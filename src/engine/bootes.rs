use crate::types::thread::Readonly;
use serde_json::{Map, Value};
use std::collections::HashMap;

use super::cli::{Argument, Cli, Environment};

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
