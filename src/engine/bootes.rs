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
    pub scrapper: Option<Readonly<Scrapper>>,
    pub other: Map<String, Value>,
}

#[derive(Debug)]
pub struct Scrapper {
    pub use_context: bool,
    pub arguments: Vec<Readonly<Arguments>>,
    pub path: String,
    pub install: Option<String>,
}

pub type Environment = HashMap<String, String>;
pub type Arguments = Vec<String>;
#[derive(Debug)]
pub struct Bootes {
    pub name: String,
    pub metadatas: HashMap<String, Readonly<Metadata>>,
    pub edges: HashMap<String, Readonly<Edge>>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub scrappers: HashMap<String, Readonly<Scrapper>>,
    pub arguments: HashMap<String, Readonly<Arguments>>,
}
