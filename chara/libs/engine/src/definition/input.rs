use std::borrow::BorrowMut;

use common::merge::Merge;
use serde_json::Value;

use crate::processor::{DefinedProcessorOverride, DraftProcessorOverride};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BaseDefinitionInput<TProcessor> {
    File(String),
    Id(String),
    Text(String),
    Value(Value),
    Processor(TProcessor),
}
impl<TProcessor: Merge + Clone> Merge for BaseDefinitionInput<TProcessor> {
    fn merge(&mut self, other: &BaseDefinitionInput<TProcessor>) {
        if let (BaseDefinitionInput::Value(value), BaseDefinitionInput::Value(other)) =
            (self.borrow_mut(), &other)
        {
            value.merge(other);
        } else {
            *self = other.clone()
        }
    }
}

pub type DraftDefinitionInput = BaseDefinitionInput<DraftProcessorOverride>;
impl DraftDefinitionInput {
    pub fn to_defined(&self) -> Option<DefinedDefinitionInput> {
        match self {
            BaseDefinitionInput::File(file) => Some(DefinedDefinitionInput::File(file.clone())),
            BaseDefinitionInput::Id(id) => Some(DefinedDefinitionInput::Id(id.clone())),
            BaseDefinitionInput::Text(txt) => Some(DefinedDefinitionInput::Text(txt.clone())),
                BaseDefinitionInput::Value(value) => Some(DefinedDefinitionInput::Value(value.clone())),
            BaseDefinitionInput::Processor(processor) => processor
                .map()
                .map(|processor| DefinedDefinitionInput::Processor(processor)),
        }
    }
}
pub type DefinedDefinitionInput = BaseDefinitionInput<DefinedProcessorOverride>;
