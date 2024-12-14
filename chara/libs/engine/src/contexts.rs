use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use common::thread::Readonly;

use crate::{definition::definition::{CleanMetadata, Metadata}, processor::{CleanProcessorOverride}};

#[derive(Debug)]
pub struct ProcessorContext {
    pub definition: DefinitionContextDto,
    pub processor: CleanProcessorOverride,
    pub metadata: Readonly<CleanMetadata>,
}
#[derive(Debug)]
pub(crate) struct EdgeContext {
    pub key: String,
    pub value: Map<String, Value>,
    pub processor: CleanProcessorOverride,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct WritePermissionsDto {
    pub metadata: bool,
    pub edge: bool,
}
impl WritePermissionsDto {
    pub fn both() -> Self {
        Self {
            edge: true,
            metadata: true,
        }
    }
    pub fn edge() -> Self {
        Self {
            edge: true,
            metadata: false,
        }
    }

    pub fn metadata() -> Self {
        Self {
            edge: false,
            metadata: true,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct ContextDto {
    pub name: String,
    pub value: Map<String, Value>,
}

impl ContextDto {
    pub fn new(name: String, value: Map<String, Value>) -> Self {
        Self { name, value }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct DefinitionContextDto {
    pub location: Option<String>,
    pub write: WritePermissionsDto,
    pub metadata: ContextDto,
    pub edge: Option<ContextDto>,
    pub processor_reference: String,
}
