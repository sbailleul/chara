use std::collections::HashMap;

use common::thread::Readonly;
use engine::cli::Arguments;

use super::REFERENCE_PREFIX;

pub fn to_arguments(
    dto_arguments: &Vec<String>,
    arguments: &HashMap<String, Readonly<Vec<String>>>,
) -> Vec<Arguments> {
    dto_arguments
        .iter()
        .map(|argument| {
            if argument.starts_with(REFERENCE_PREFIX) {
                arguments
                    .get(argument.trim_start_matches(REFERENCE_PREFIX))
                    .map(|v| v.clone())
                    .map(|arguments| Arguments::Reference {
                        name: argument.clone(),
                        arguments,
                    })
            } else {
                Some(Arguments::Value(vec![argument.clone()]))
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
            Arguments::Reference { arguments, .. } => arguments
                .read()
                .map(|lock| (*lock).clone())
                .unwrap_or(Vec::<String>::new()),
        })
        .collect()
}
