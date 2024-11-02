use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use types::thread::Readonly;

use crate::definition::{ForeignDefinition, ProcessorOverride};

#[derive(Debug)]
pub struct ProcessorContext {
    pub definition: DefinitionContextDto,
    pub processor: ProcessorOverride,
}
#[derive(Debug)]
pub(crate) struct EdgeContext {
    pub key: String,
    pub value: Value,
    pub definition: Option<Readonly<ForeignDefinition>>,
    pub processor: ProcessorOverride,
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
pub struct DefinitionContextDto {
    pub write: WritePermissionsDto,
    pub metadata: (String, Value),
    pub edge: Option<(String, Value)>,
}
