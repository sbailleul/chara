
use engine::{
    cli::DraftArguments, definition::definition::Definition, reference_value::{LazyRefOrValue, ReferencedValue}
};

use super::REFERENCE_PREFIX;

pub fn to_arguments(
    dto_arguments: &Vec<String>,
    definition: &Definition,
) -> Vec<DraftArguments> {
    dto_arguments
        .iter()
        .map(|argument| {
            if argument.starts_with(REFERENCE_PREFIX) {
                definition.find_argument(argument)
                    .map(|v| v.clone())
                    .map(|arguments| {
                        LazyRefOrValue::to_referenced_value(argument.clone(), arguments.clone())
                    })
                    .or(Some(LazyRefOrValue::Ref(argument.clone())))
            } else {
                Some(LazyRefOrValue::Value(vec![argument.clone()]))
            }
        })
        .flatten()
        .collect()
}


pub fn from_arguments(arguments: Vec<DraftArguments>) -> Vec<String> {
    arguments
        .iter()
        .flat_map(|arguments| match arguments {
            LazyRefOrValue::Ref(_) => vec![],
            LazyRefOrValue::ReferencedValue(ReferencedValue { r#ref, value: _ }) => vec![r#ref.clone()],
            LazyRefOrValue::Value(value) => value.clone(),
        })
        .collect()
}
