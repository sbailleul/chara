use std::borrow::BorrowMut;

use common::merge::Merge;
use serde_json::Value;

use crate::processor::ProcessorOverride;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DefinitionInput {
    File(String),
    Text(String),
    Value(Value),
    Processor(ProcessorOverride),
}

impl Merge for DefinitionInput {
    fn merge(&mut self, other: &DefinitionInput) {
        if let (DefinitionInput::Value(value), DefinitionInput::Value(other)) =
            (self.borrow_mut(), &other)
        {
            value.merge(other);
        } else {
            *self = other.clone()
        }
    }
}
