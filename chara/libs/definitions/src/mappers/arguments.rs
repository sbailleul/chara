use std::collections::HashMap;

use common::thread::Readonly;
use engine::{
    cli::Arguments,
    draft::draft_definition::DraftArguments,
    reference_value::{LazyRefOrValue, ReferencedValue},
};

use super::REFERENCE_PREFIX;

pub fn to_draft_arguments(
    dto_arguments: &Vec<String>,
    arguments: &HashMap<String, Readonly<Vec<String>>>,
) -> Vec<DraftArguments> {
    dto_arguments
        .iter()
        .map(|argument| {
            if argument.starts_with(REFERENCE_PREFIX) {
                arguments
                    .get(argument.trim_start_matches(REFERENCE_PREFIX))
                    .map(|v| v.clone())
                    .map(|arguments| {
                        LazyRefOrValue::referenced_value(argument.clone(), arguments.clone())
                    })
                    .or(Some(LazyRefOrValue::Ref(argument.clone())))
            } else {
                Some(LazyRefOrValue::Value(vec![argument.clone()]))
            }
        })
        .flatten()
        .collect()
}


pub fn from_draft_arguments(arguments: Vec<DraftArguments>) -> Vec<String> {
    arguments
        .iter()
        .flat_map(|arguments| match arguments {
            LazyRefOrValue::Ref(_) => vec![],
            LazyRefOrValue::ReferencedValue(ReferencedValue { r#ref, value: _ }) => vec![r#ref.clone()],
            LazyRefOrValue::Value(value) => value.clone(),
        })
        .collect()
}
