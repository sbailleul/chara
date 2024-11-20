pub use engine::contexts::{DefinitionContextDto, WritePermissionsDto};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnrichmentDto {
    pub edge: Option<Map<String, Value>>,
    pub metadata: Option<Map<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorResultDto {
    pub enrichment: Option<EnrichmentDto>,
    pub definition: Option<DefinitionDto>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TagDto {
    pub label: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, TagDto>,
    #[serde(flatten)]
    pub other: Value,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetadataEdge {
    pub r#ref: String,
    #[serde(flatten)]
    pub other: Map<String, Value>,
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default)]
    pub environments: Vec<EnvironmentDto>,
    pub definition: Option<DefinitionDto>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetadataDto {
    #[serde(default)]
    pub edges: Vec<ReferenceOrObjectDto<MetadataEdge>>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    pub other: Map<String, Value>,
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
    pub fn definition(&self) -> Option<DefinitionDto> {
        match self {
            ReferenceOrObjectDto::Reference(_reference) => None,
            ReferenceOrObjectDto::Object(edge) => edge.definition.clone(),
        }
    }
    pub fn other(&self) -> Map<String, Value> {
        match self {
            ReferenceOrObjectDto::Reference(_reference) => Map::<String, Value>::new(),
            ReferenceOrObjectDto::Object(edge) => edge.other.clone(),
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
#[serde(untagged)]
pub enum ForeignDefinitionDto {
    String(String),
    Definition(DefinitionDto),
    Processor(ProcessorOverrideDto),
}
impl Hash for DefinitionDto {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.location.hash(state);
    }
}
impl ForeignDefinitionDto {
    pub fn key(&self) -> String {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish().to_string()
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EdgeDto {
    pub definition: Option<ForeignDefinitionDto>,
    pub processor: Option<ReferenceOrObjectDto<ProcessorOverrideDto>>,
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstallDto {
    pub program: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default)]
    pub environments: Vec<EnvironmentDto>,
    #[serde(rename(deserialize = "currentDirectory"))]
    pub current_directory: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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
#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
pub struct ProcessorOverrideDto {
    pub reference: String,
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default)]
    pub environments: Vec<EnvironmentDto>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum ReferenceOrObjectDto<Value> {
    Reference(String),
    Object(Value),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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
