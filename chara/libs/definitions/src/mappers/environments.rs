
use engine::{
    cli::DraftEnvironments, definition::definition::Definition, reference_value::{LazyRefOrValue, ReferencedValue}
};

use crate::definition::EnvironmentDto;



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
    definition: &Definition,
) -> Vec<DraftEnvironments> {
    dto_environments
        .iter()
        .map(|environment| match environment {
            EnvironmentDto::Reference(reference) => definition
                .find_environment(reference)
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
