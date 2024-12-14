use std::collections::HashMap;

use common::thread::{Read, Readonly};
use engine::{
    cli::Arguments,
    draft::draft_definition::DraftArguments,
    reference_value::{LazyRefValue, ReferencedValue},
};

use super::REFERENCE_PREFIX;

pub fn to_arguments(
    dto_arguments: &Vec<String>,
    arguments: &HashMap<String, Readonly<Vec<String>>>,
) -> Vec<Arguments> {
    to_draft_arguments(dto_arguments, arguments)
        .into_iter()
        .map(|arg| match arg {
            LazyRefValue::Ref(_) => None,
            LazyRefValue::ReferencedValue(ref_value) => {
                Some(Arguments::ReferencedValue(ReferencedValue {
                    r#ref: ref_value.r#ref,
                    value: ref_value.value,
                }))
            }
            LazyRefValue::Value(value) => Some(Arguments::Value(value)),
        })
        .flatten()
        .collect()
}

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
                        LazyRefValue::referenced_value(argument.clone(), arguments.clone())
                    })
                    .or(Some(LazyRefValue::Ref(argument.clone())))
            } else {
                Some(LazyRefValue::Value(vec![argument.clone()]))
            }
        })
        .flatten()
        .collect()
}

pub fn from_arguments(arguments: Vec<Arguments>) -> Vec<String> {
    arguments
        .into_iter()
        .flat_map(|arg| match arg {
            Arguments::Value(arg) => arg,
            Arguments::ReferencedValue(ReferencedValue { r#ref: _, value })  => value
                .read()
                .map(|lock| (*lock).clone())
                .unwrap_or(Vec::<String>::new()),
        })
        .collect()
}

pub fn from_draft_arguments(arguments: Vec<DraftArguments>) -> Vec<String> {
    arguments
        .iter()
        .flat_map(|arguments| match arguments {
            LazyRefValue::Ref(_) => vec![],
            LazyRefValue::ReferencedValue(ReferencedValue { r#ref, value: _ }) => vec![r#ref.clone()],
            LazyRefValue::Value(value) => value.clone(),
        })
        .collect()
}
