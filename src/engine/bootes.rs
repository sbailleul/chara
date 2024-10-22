use crate::types::thread::Readonly;
use serde_json::{Map, Value};
use std::collections::HashMap;

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
}

#[derive(Debug)]
pub struct Edge {
    pub definition: Option<String>,
    pub enricher: Option<Readonly<Enricher>>,
    pub other: Map<String, Value>,
}

#[derive(Debug)]
pub struct Install {
    pub path: String,
    pub arguments: Vec<String>,
}
#[derive(Debug)]
pub enum Argument {
    Value(String),
    Reference(Readonly<Vec<String>>),
}

#[derive(Debug)]
pub struct Enricher {
    pub use_context: bool,
    pub arguments: Vec<Argument>,
    pub path: String,
    pub install: Option<Install>,
    pub environments: Vec<Readonly<HashMap<String, String>>>,
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
