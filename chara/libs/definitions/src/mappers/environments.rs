use std::collections::HashMap;

use common::thread::Readonly;
use engine::{
    cli::Environment,
    draft::draft_definition::DraftEnvironments,
    reference_value::{LazyRefValue, ReferencedValue},
};

use crate::definition::EnvironmentDto;

use super::REFERENCE_PREFIX;

pub fn from_environments(environments: Vec<Environment>) -> Vec<EnvironmentDto> {
    environments
        .into_iter()
        .map(|env| match env {
            Environment::Value(hash_map) => EnvironmentDto::Value(hash_map),
            Environment::ReferencedValue(ReferencedValue { r#ref, .. }) => {
                EnvironmentDto::Reference(r#ref)
            }
        })
        .collect()
}

pub fn from_draft_environments(environments: Vec<DraftEnvironments>) -> Vec<EnvironmentDto> {
    environments
        .into_iter()
        .map(|env| match env {
            LazyRefValue::Ref(reference) => EnvironmentDto::Reference(reference),
            LazyRefValue::ReferencedValue(ReferencedValue { r#ref, value: _ }) => EnvironmentDto::Reference(r#ref),
            LazyRefValue::Value(value) => EnvironmentDto::Value(value),
        })
        .collect()
}

pub fn to_environments(
    dto_environments: &Vec<EnvironmentDto>,
    environments: &HashMap<String, Readonly<HashMap<String, String>>>,
) -> Vec<Environment> {
    to_draft_environments(dto_environments, environments)
        .into_iter()
        .map(|environment| match environment {
            LazyRefValue::Ref(_) => None,
            LazyRefValue::ReferencedValue(ref_value) => {
                Some(Environment::ReferencedValue(ReferencedValue {
                    r#ref: ref_value.r#ref,
                    value: ref_value.value,
                }))
            }
            LazyRefValue::Value(value) => Some(Environment::Value(value)),
        })
        .flatten()
        .collect()
}
pub fn to_draft_environments(
    dto_environments: &Vec<EnvironmentDto>,
    environments: &HashMap<String, Readonly<HashMap<String, String>>>,
) -> Vec<DraftEnvironments> {
    dto_environments
        .iter()
        .map(|environment| match environment {
            EnvironmentDto::Reference(reference) => environments
                .get(reference.trim_start_matches(REFERENCE_PREFIX))
                .map(|v| {
                    LazyRefValue::ReferencedValue(ReferencedValue {
                        r#ref: reference.clone(),
                        value: v.clone(),
                    })
                })
                .or(Some(LazyRefValue::Ref(reference.clone()))),
            EnvironmentDto::Value(hash_map) => Some(DraftEnvironments::Value(hash_map.clone())),
        })
        .flatten()
        .collect()
}
