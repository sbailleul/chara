use engine::{
    clean::clean_definition::{CleanDefinition, CleanProcessorOverride},
    draft::draft_definition::{DraftDefinition, DraftProcessorOverride},
    processor::ProcessorOverride,
    reference_value::{LazyRef, ReferencedValue},
};

use crate::definition::{ProcessorOverrideDto, ReferenceOrObjectDto};

use super::{
    arguments::{to_arguments, to_draft_arguments},
    environments::{to_draft_environments, to_environments},
    REFERENCE_PREFIX,
};

pub fn to_node_processor(
    node_processor: &ReferenceOrObjectDto<ProcessorOverrideDto>,
    definition: &CleanDefinition,
) -> Option<CleanProcessorOverride> {
    match node_processor {
        ReferenceOrObjectDto::Reference(reference) => definition
            .processors
            .get(reference.trim_start_matches(REFERENCE_PREFIX))
            .map(|processor| {
                CleanProcessorOverride::processor(&ReferencedValue {
                    r#ref: reference.clone(),
                    value: processor.clone(),
                })
            }),
        ReferenceOrObjectDto::Object(processor_override) => {
            to_processor_override(processor_override, definition)
        }
    }
}

pub fn to_processor_override(
    processor_override: &ProcessorOverrideDto,
    definition: &CleanDefinition,
) -> Option<CleanProcessorOverride> {
    if let Some(reference) = processor_override.reference.as_ref() {
        definition
            .processors
            .get(reference.trim_start_matches(REFERENCE_PREFIX))
            .map(|processor| CleanProcessorOverride {
                arguments: to_arguments(&processor_override.arguments, &definition.arguments),
                environments: to_environments(
                    &processor_override.environments,
                    &definition.environments,
                ),
                processor: ReferencedValue {
                    r#ref: reference.clone(),
                    value: processor.clone(),
                },
            })
    } else {
        None
    }
}

pub fn to_node_draft_processor(
    node_processor: &ReferenceOrObjectDto<ProcessorOverrideDto>,
    definition: &DraftDefinition,
) -> DraftProcessorOverride {
    match node_processor {
        ReferenceOrObjectDto::Reference(reference) => definition
            .processors
            .get(reference.trim_start_matches(REFERENCE_PREFIX))
            .map(|processor| {
                DraftProcessorOverride::processor(&Some(LazyRef::referenced_value(
                    reference.clone(),
                    processor.clone(),
                )))
            })
            .unwrap_or(DraftProcessorOverride::processor(&Some(LazyRef::Ref(
                reference.clone(),
            )))),
        ReferenceOrObjectDto::Object(processor_override) => {
            to_draft_processor_override(processor_override, definition)
        }
    }
}

pub fn to_draft_processor_override(
    processor_override: &ProcessorOverrideDto,
    definition: &DraftDefinition,
) -> DraftProcessorOverride {
    if let Some(reference) = processor_override.reference.as_ref() {
        definition
            .processors
            .get(reference.trim_start_matches(REFERENCE_PREFIX))
            .map(|processor| DraftProcessorOverride {
                arguments: to_draft_arguments(&processor_override.arguments, &definition.arguments),
                environments: to_draft_environments(
                    &processor_override.environments,
                    &definition.environments,
                ),
                processor: Some(LazyRef::referenced_value(
                    reference.clone(),
                    processor.clone(),
                )),
            })
            .unwrap_or(DraftProcessorOverride::processor(&Some(LazyRef::Ref(
                reference.clone(),
            ))))
    } else {
        DraftProcessorOverride {
            arguments: to_draft_arguments(&processor_override.arguments, &definition.arguments),
            environments: to_draft_environments(
                &processor_override.environments,
                &definition.environments,
            ),
            processor: None,
        }
    }
}
