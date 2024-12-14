use std::borrow::BorrowMut;

use common::merge::Merge;
use serde_json::Value;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DefinitionInput<TProcessor> {
    File(String),
    Text(String),
    Value(Value),
    Processor(TProcessor),
}
impl<TProcessor: Merge + Clone> Merge for DefinitionInput<TProcessor> {
    fn merge(&mut self, other: &DefinitionInput<TProcessor>) {
        if let (DefinitionInput::Value(value), DefinitionInput::Value(other)) =
            (self.borrow_mut(), &other)
        {
            value.merge(other);
        } else {
            *self = other.clone()
        }
    }
}
