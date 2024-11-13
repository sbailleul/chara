pub use engine::contexts::{DefinitionContextDto, WritePermissionsDto};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorResultDto {
    pub metadata: Option<Value>,
    pub edge: Option<DefinitionDto>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TagDto {
    pub label: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, TagDto>,
    #[serde(flatten)]
    pub other: Value,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MetadataEdge {
    pub r#ref: String,
    #[serde(flatten)]
    pub other: Value,
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default)]
    pub environments: Vec<EnvironmentDto>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MetadataDto {
    #[serde(default)]
    pub edges: Vec<ReferenceOrObjectDto<MetadataEdge>>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub other: Value,
    pub processor: Option<ReferenceOrObjectDto<ProcessorOverrideDto>>,
}

impl ReferenceOrObjectDto<MetadataEdge> {
    pub fn reference(&self) -> String {
        match self {
            ReferenceOrObjectDto::Reference(reference) => reference.clone(),
            ReferenceOrObjectDto::Object(edge) => edge.r#ref.clone(),
        }
    }
    pub fn arguments(&self) -> Vec<String> {
        match self {
            ReferenceOrObjectDto::Reference(_reference) => vec![],
            ReferenceOrObjectDto::Object(edge) => edge.arguments.clone(),
        }
    }
    pub fn environments(&self) -> Vec<EnvironmentDto> {
        match self {
            ReferenceOrObjectDto::Reference(_reference) => vec![],
            ReferenceOrObjectDto::Object(edge) => edge.environments.clone(),
        }
    }
}
#[derive(Debug, Deserialize,Serialize, Hash)]
#[serde(untagged)]
pub enum ForeignDefinitionDto {
    String(String),
    Processor(ProcessorOverrideDto),
}
impl ForeignDefinitionDto {
    pub fn key(&self) -> String {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish().to_string()
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EdgeDto {
    pub definition: Option<ForeignDefinitionDto>,
    pub processor: Option<ReferenceOrObjectDto<ProcessorOverrideDto>>,
    #[serde(flatten)]
    pub other: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstallDto {
    pub program: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default)]
    pub environments: Vec<EnvironmentDto>,
    #[serde(rename(deserialize = "currentDirectory"))]
    pub current_directory: Option<String>,
}

#[derive(Debug, Deserialize,Serialize, Clone)]
#[serde(untagged)]
pub enum EnvironmentDto {
    Reference(String),
    Value(HashMap<String, String>),
}
impl Hash for EnvironmentDto {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            EnvironmentDto::Reference(_) => todo!(),
            EnvironmentDto::Value(hash_map) => hash_map.iter().for_each(|(k, v)| {
                k.hash(state);
                v.hash(state);
            }),
        };
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessorDto {
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default)]
    pub environments: Vec<EnvironmentDto>,
    pub program: String,
    pub install: Option<InstallDto>,
    #[serde(rename(deserialize = "currentDirectory"))]
    pub current_directory: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, Hash)]
pub struct ProcessorOverrideDto {
    pub reference: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default)]
    pub environments: Vec<EnvironmentDto>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ReferenceOrObjectDto<Value> {
    Reference(String),
    Object(Value),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DefinitionDto {
    pub name: String,
    pub location: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, MetadataDto>,
    #[serde(default)]
    pub edges: HashMap<String, EdgeDto>,
    #[serde(default)]
    pub tags: HashMap<String, TagDto>,
    #[serde(default)]
    pub processors: HashMap<String, ProcessorDto>,
    #[serde(default)]
    pub arguments: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub environments: HashMap<String, HashMap<String, String>>,
}
