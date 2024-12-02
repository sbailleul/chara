use std::collections::HashMap;

use common::thread::Readonly;
use serde_json::{Map, Value};

use crate::{
    cli::Environment, definition::{definition::Definition, foreign_definition::ForeignDefinition, input::DefinitionInput}, reference_value::RefValue
};
type Arguments = RefValue<Vec<String>>;
type Environments = RefValue<HashMap<String, String>>;
#[derive(Debug, Clone)]
pub struct DraftForeignDefinition {
    pub input: Option<DefinitionInput>,
    pub output: Option<DraftDefinition>,
}
#[derive(Debug, Clone)]
pub struct DraftInstall {
    pub arguments: Vec<Arguments>,
    pub program: String,
    pub environments: Vec<Environment>,
    pub current_directory: Option<String>,
}
#[derive(Debug, Clone)]
pub struct DraftProcessor {
    pub arguments: Vec<Arguments>,
    pub program: String,
    pub install: Option<DraftInstall>,
    pub environments: Vec<Environments>,
    pub current_directory: Option<String>,
}
#[derive(Debug, Clone)]
pub struct ProcessorOverride {
    pub arguments: Vec<Arguments>,
    pub environments: Vec<Environments>,
    pub processor: RefValue<DraftProcessor>,
    pub reference: String,
}
#[derive(Debug, Clone)]
pub struct Edge {
    pub definition: Option<Readonly<ForeignDefinition>>,
    pub processor: Option<ProcessorOverride>,
    pub other: Map<String, Value>,
}
#[derive(Debug, Clone)]
pub struct EdgeOverride {
    pub arguments: Vec<Arguments>,
    pub environments: Vec<Environments>,
    pub edge: RefValue<Edge>,
    pub other: Map<String, Value>,
    pub definition: Option<Definition>,
}
#[derive(Debug, Clone)]
pub struct Tag{
    pub label: Option<String>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub other: Value,
}
#[derive(Debug, Clone)]
pub struct DraftMetadata {
    pub edges: HashMap<String, EdgeOverride>,
    pub tags: HashMap<String, RefValue<Tag>>,
    pub other: Map<String, Value>,
    pub processor: Option<ProcessorOverride>,
}

#[derive(Debug, Clone)]
pub struct DraftDefinition {
    pub name: String,
    pub id: String,
    pub location: Option<String>,
    pub metadata: HashMap<String, Readonly<DraftMetadata>>,
    pub edges: HashMap<String, Readonly<Edge>>,
    pub tags: HashMap<String, Readonly<Tag>>,
    pub processors: HashMap<String, Readonly<DraftProcessor>>,
    pub arguments: HashMap<String, Readonly<Vec<String>>>,
    pub environments: HashMap<String, Readonly<HashMap<String, String>>>,
    pub foreign_definitions: HashMap<String, Readonly<ForeignDefinition>>,
}
