use std::borrow::BorrowMut;

use common::merge::Merge;
use serde_json::Value;

use crate::processor::CleanProcessorOverride;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DefinitionInput<TProcessor> {
    File(String),
    Text(String),
    Value(Value),
    Processor(TProcessor),
}
pub type CleanDefinitionInput = DefinitionInput<CleanProcessorOverride>;
impl Merge for CleanDefinitionInput {
    fn merge(&mut self, other: &CleanDefinitionInput) {
        if let (DefinitionInput::Value(value), DefinitionInput::Value(other)) =
            (self.borrow_mut(), &other)
        {
            value.merge(other);
        } else {
            *self = other.clone()
        }
    }
}
