use common::thread::Readonly;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{
    draft::draft_definition::{
        DefinedProcessorOverride, DraftMetadata, DraftProcessorOverride
    },
    reference_value::LazyRef,
};

impl DraftProcessorOverride {
    pub fn map(&self) -> Option<DefinedProcessorOverride> {
        if let Some(inner_processor) = self.processor.as_ref() {
            if let LazyRef::ReferencedValue(inner_processor) = inner_processor {
                Some(DefinedProcessorOverride {
                    arguments: self.arguments.clone(),
                    environments: self.environments.clone(),
                    processor: inner_processor.clone(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
#[derive(Debug)]
pub struct ProcessorContext {
    pub definition: DefinitionContextDto,
    pub processor: DefinedProcessorOverride,
    pub metadata: Readonly<DraftMetadata>,
}
#[derive(Debug)]
pub(crate) struct EdgeContext {
    pub key: String,
    pub value: Map<String, Value>,
    pub processor: DefinedProcessorOverride,
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
