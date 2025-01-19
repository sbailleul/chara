use engine::{
    definition::definition::Definition, processor::DraftProcessorOverride, reference_value::LazyRef,
};

use crate::definition::{ProcessorOverrideDto, ReferenceOrObjectDto};

use super::{arguments::to_arguments, environments::to_environments, REFERENCE_PREFIX};

pub fn to_node_processor(
    node_processor: &ReferenceOrObjectDto<ProcessorOverrideDto>,
    definition: &Definition,
) -> DraftProcessorOverride {
    match node_processor {
        ReferenceOrObjectDto::Reference(reference) => definition
            .processors
            .get(reference.trim_start_matches(REFERENCE_PREFIX))
            .map(|processor| {
                DraftProcessorOverride::processor(&Some(LazyRef::new_referenced_value(
                    reference.clone(),
                    processor.clone(),
                )))
            })
            .unwrap_or(DraftProcessorOverride::processor(&Some(LazyRef::Ref(
                reference.clone(),
            )))),
        ReferenceOrObjectDto::Object(processor_override) => {
            to_processor_override(processor_override, definition)
        }
    }
}

pub fn to_processor_override(
    processor_override: &ProcessorOverrideDto,
    definition: &Definition,
) -> DraftProcessorOverride {
    if let Some(reference) = processor_override.reference.as_ref() {
        definition
            .processors
            .get(reference.trim_start_matches(REFERENCE_PREFIX))
            .map(|processor| DraftProcessorOverride {
                arguments: to_arguments(&processor_override.arguments, definition),
                environments: to_environments(&processor_override.environments, definition),
                processor: Some(LazyRef::new_referenced_value(
                    reference.clone(),
                    processor.clone(),
                )),
            })
            .unwrap_or(DraftProcessorOverride::processor(&Some(LazyRef::Ref(
                reference.clone(),
            ))))
    } else {
        DraftProcessorOverride {
            arguments: to_arguments(&processor_override.arguments, definition),
            environments: to_environments(&processor_override.environments, definition),
            processor: None,
        }
    }
}
