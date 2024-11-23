use std::collections::HashMap;

use engine::cli::Argument;
use common::thread::Readonly;

use super::REFERENCE_PREFIX;

pub fn to_arguments(
    dto_arguments: &Vec<String>,
    arguments: &HashMap<String, Readonly<Vec<String>>>,
) -> Vec<Argument> {
    dto_arguments
        .iter()
        .map(|argument| {
            if argument.starts_with(REFERENCE_PREFIX) {
                arguments
                    .get(argument.trim_start_matches(REFERENCE_PREFIX))
                    .map(|v| v.clone())
                    .map(|reference| Argument::Reference {
                        name: argument.clone(),
                        arguments: reference,
                    })
            } else {
                Some(Argument::Value(argument.clone()))
            }
        })
        .flatten()
        .collect()
}

pub fn from_arguments(arguments: Vec<Argument>) -> Vec<String> {
    arguments
        .into_iter()
        .map(|arg| match arg {
            Argument::Value(arg) => arg,
            Argument::Reference { name, .. } => name,
        })
        .collect()
}