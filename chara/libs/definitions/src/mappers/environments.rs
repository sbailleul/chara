use std::collections::HashMap;

use engine::cli::Environment;
use common::thread::Readonly;

use crate::definition::EnvironmentDto;

use super::REFERENCE_PREFIX;

pub fn from_environments(environments: Vec<Environment>) -> Vec<EnvironmentDto> {
    environments
        .into_iter()
        .map(|env| match env {
            Environment::Value(hash_map) => EnvironmentDto::Value(hash_map),
            Environment::Reference { name, .. } => EnvironmentDto::Reference(name),
        })
        .collect()
}

pub fn to_environments(
    dto_environments: &Vec<EnvironmentDto>,
    environments: &HashMap<String, Readonly<HashMap<String, String>>>,
) -> Vec<Environment> {
    dto_environments
        .iter()
        .map(|environment| match environment {
            EnvironmentDto::Reference(reference) => environments
                .get(reference.trim_start_matches(REFERENCE_PREFIX))
                .map(|v| Environment::Reference {
                    name: reference.clone(),
                    environment: v.clone(),
                }),
            EnvironmentDto::Value(hash_map) => Some(Environment::Value(hash_map.clone())),
        })
        .flatten()
        .collect()
}