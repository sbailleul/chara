use engine::{
    definition::definition::Definition,
    draft::draft_definition::{DraftDefinition, DraftProcessorOverride},
    processor::{CleanProcessorOverride, ProcessorOverride},
    reference_value::{LazyRefValue, ReferencedValue},
};

use crate::definition::{ProcessorOverrideDto, ReferenceOrObjectDto};

use super::{
    arguments::{to_arguments, to_draft_arguments},
    environments::{to_draft_environments, to_environments},
    REFERENCE_PREFIX,
};

pub fn to_node_processor(
    node_processor: &ReferenceOrObjectDto<ProcessorOverrideDto>,
    definition: &Definition,
) -> Option<CleanProcessorOverride> {
    match node_processor {
        ReferenceOrObjectDto::Reference(reference) => definition
            .processors
            .get(reference.trim_start_matches(REFERENCE_PREFIX))
            .map(|processor| CleanProcessorOverride::processor(processor, reference)),
        ReferenceOrObjectDto::Object(processor_override) => {
            to_processor_override(processor_override, definition)
        }
    }
}

pub fn to_processor_override(
    processor_override: &ProcessorOverrideDto,
    definition: &Definition,
) -> Option<CleanProcessorOverride> {
    definition
        .processors
        .get(
            processor_override
                .reference
                .trim_start_matches(REFERENCE_PREFIX),
        )
        .map(|processor| CleanProcessorOverride {
            r#ref: processor_override.reference.clone(),
            value: ProcessorOverride {
                arguments: to_arguments(&processor_override.arguments, &definition.arguments),
                environments: to_environments(
                    &processor_override.environments,
                    &definition.environments,
                ),
                processor: processor.clone(),
            },
        })
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
                DraftProcessorOverride::processor(LazyRefValue::referenced_value(
                    reference.clone(),
                    processor.clone(),
                ))
            })
            .unwrap_or(DraftProcessorOverride::processor(LazyRefValue::Ref(
                reference.clone(),
            ))),
        ReferenceOrObjectDto::Object(processor_override) => {
            to_draft_processor_override(processor_override, definition)
        }
    }
}

pub fn to_draft_processor_override(
    processor_override: &ProcessorOverrideDto,
    definition: &DraftDefinition,
) -> DraftProcessorOverride {
    definition
        .processors
        .get(
            processor_override
                .reference
                .trim_start_matches(REFERENCE_PREFIX),
        )
        .map(|processor| DraftProcessorOverride {
            arguments: to_draft_arguments(&processor_override.arguments, &definition.arguments),
            environments: to_draft_environments(
                &processor_override.environments,
                &definition.environments,
            ),
            processor: LazyRefValue::referenced_value(
                processor_override.reference.clone(),
                processor.clone(),
            ),
        })
        .unwrap_or(DraftProcessorOverride::processor(LazyRefValue::Ref(
            processor_override.reference.clone(),
        )))
}
