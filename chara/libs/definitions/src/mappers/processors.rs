use engine::{definition::Definition, processor::ProcessorOverride};

use crate::definition::{ProcessorOverrideDto, ReferenceOrObjectDto};

use super::{arguments::to_arguments, environments::to_environments, REFERENCE_PREFIX};

pub fn to_node_processor(
    node_processor: &ReferenceOrObjectDto<ProcessorOverrideDto>,
    definition: &Definition,
) -> Option<ProcessorOverride> {
    match node_processor {
        ReferenceOrObjectDto::Reference(reference) => definition
            .processors
            .get(reference.trim_start_matches(REFERENCE_PREFIX))
            .map(|processor| ProcessorOverride::processor(processor, reference)),
        ReferenceOrObjectDto::Object(processor_override) => {
            to_processor_override(processor_override, definition)
        }
    }
}

pub fn to_processor_override(
    processor_override: &ProcessorOverrideDto,
    definition: &Definition,
) -> Option<ProcessorOverride> {
    definition
        .processors
        .get(
            processor_override
                .reference
                .trim_start_matches(REFERENCE_PREFIX),
        )
        .map(|processor| ProcessorOverride {
            reference: processor_override.reference.clone(),
            arguments: to_arguments(&processor_override.arguments, &definition.arguments),
            environments: to_environments(
                &processor_override.environments,
                &definition.environments,
            ),
            processor: processor.clone(),
        })
}