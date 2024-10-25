use serde::Deserialize;
use serde_json::{Map, Value};

use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct TagDto {
    pub label: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, TagDto>,
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct MetadataDto {
    #[serde(default)]
    pub edges: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub other: Map<String, Value>,
    pub enricher: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EdgeDto {
    pub definition: Option<String>,
    pub enricher: Option<String>,
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct InstallDto {
    pub program: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default)]
    pub environments: Vec<EnvironmentDto>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum EnvironmentDto {
    Reference(String),
    Value(HashMap<String, String>),
}

#[derive(Debug, Deserialize)]
pub struct EnricherDto {
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default)]
    pub environments: Vec<EnvironmentDto>,
    pub program: String,
    pub install: Option<InstallDto>,
}

#[derive(Deserialize, Debug)]
pub struct CharaDto {
    pub name: String,
    #[serde(default)]
    pub metadata: HashMap<String, MetadataDto>,
    #[serde(default)]
    pub edges: HashMap<String, EdgeDto>,
    #[serde(default)]
    pub tags: HashMap<String, TagDto>,
    #[serde(default)]
    pub enrichers: HashMap<String, EnricherDto>,
    #[serde(default)]
    pub arguments: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub environments: HashMap<String, HashMap<String, String>>,
}
