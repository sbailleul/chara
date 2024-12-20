use std::collections::HashMap;

use common::thread::Readonly;
use engine::{
    cli::DraftEnvironments, reference_value::{LazyRefOrValue, ReferencedValue}
};

use crate::definition::EnvironmentDto;

use super::REFERENCE_PREFIX;


pub fn from_environments(environments: Vec<DraftEnvironments>) -> Vec<EnvironmentDto> {
    environments
        .into_iter()
        .map(|env| match env {
            LazyRefOrValue::Ref(reference) => EnvironmentDto::Reference(reference),
            LazyRefOrValue::ReferencedValue(ReferencedValue { r#ref, value: _ }) => EnvironmentDto::Reference(r#ref),
            LazyRefOrValue::Value(value) => EnvironmentDto::Value(value),
        })
        .collect()
}

pub fn to_environments(
    dto_environments: &Vec<EnvironmentDto>,
    environments: &HashMap<String, Readonly<HashMap<String, String>>>,
) -> Vec<DraftEnvironments> {
    dto_environments
        .iter()
        .map(|environment| match environment {
            EnvironmentDto::Reference(reference) => environments
                .get(reference.trim_start_matches(REFERENCE_PREFIX))
                .map(|v| {
                    LazyRefOrValue::ReferencedValue(ReferencedValue {
                        r#ref: reference.clone(),
                        value: v.clone(),
                    })
                })
                .or(Some(LazyRefOrValue::Ref(reference.clone()))),
            EnvironmentDto::Value(hash_map) => Some(DraftEnvironments::Value(hash_map.clone())),
        })
        .flatten()
        .collect()
}
