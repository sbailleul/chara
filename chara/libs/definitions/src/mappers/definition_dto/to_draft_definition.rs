use std::{collections::HashMap, path};

use engine::{
    definition::{foreign_definition::ForeignDefinition, input::DefinitionInput},
    draft::draft_definition::{
        DraftDefinition, DraftDefinitionInput, DraftEdge, DraftEdgeOverride, DraftInstall,
        DraftMetadata, DraftProcessor, DraftProcessorOverride,
    },
    reference_value::{LazyRef, LazyRefOrValue, ReferencedValue},
};

use common::thread::readonly;
use uuid::Uuid;

use crate::{
    definition::{DefinitionDto, ForeignDefinitionDto, ReferenceOrObjectDto},
    mappers::{
        arguments::to_draft_arguments,
        environments::to_draft_environments,
        processors::{
            to_draft_processor_override, to_node_draft_processor,
        },
        REFERENCE_PREFIX,
    },
};

impl DefinitionDto {
    pub fn map_draft_overwrite_location(self, location: String) -> DraftDefinition {
        let location = Some(location);
        let mut definition = self.map_draft_with_location(location.clone());
        definition.location = location;
        definition
    }
    pub fn map_draft_with_location(self, location: Option<String>) -> DraftDefinition {
        let mut definition = DraftDefinition {
            id: self.id.clone().unwrap_or(Uuid::new_v4().to_string()),
            location: self.location.clone().or(location),
            name: self.name.clone(),
            arguments: self.arguments(),
            environments: self.environments(),
            edges: HashMap::new(),
            metadata: HashMap::new(),
            processors: HashMap::new(),
            tags: HashMap::new(),
            foreign_definitions: HashMap::new(),
        };
        self.set_draft_processors(&mut definition);
        self.set_draft_edges(&mut definition);
        definition.tags = self.list_tags();
        self.set_draft_metadata(&mut definition);
        definition
    }

    fn set_draft_processors(&self, definition: &mut DraftDefinition) {
        definition.processors = self
            .processors
            .iter()
            .map(|(key, processor)| {
                (
                    key.clone(),
                    readonly(DraftProcessor {
                        arguments: to_draft_arguments(&processor.arguments, &definition.arguments),
                        program: processor.program.clone(),
                        install: processor.install.as_ref().map(|install| DraftInstall {
                            arguments: to_draft_arguments(
                                &install.arguments,
                                &definition.arguments,
                            ),
                            environments: to_draft_environments(
                                &install.environments,
                                &definition.environments,
                            ),
                            program: install.program.clone(),
                            current_directory: install.current_directory.clone(),
                        }),
                        environments: to_draft_environments(
                            &processor.environments,
                            &definition.environments,
                        ),
                        current_directory: processor.current_directory.clone(),
                    }),
                )
            })
            .collect()
    }

    fn set_draft_edges(&self, definition: &mut DraftDefinition) {
        definition.edges = self
            .edges
            .iter()
            .map(|(key, edge)| {
                let foreign_definition = edge
                    .definition
                    .as_ref()
                    .map(|foreign_definition| {
                        let key = foreign_definition.key();

                        if let Some(foreign_definition) = definition.foreign_definitions.get(&key) {
                            Some(foreign_definition.clone())
                        } else {
                            if let ForeignDefinitionDto::Definition(ready_definition) =
                                foreign_definition
                            {
                                let foreign_definition = readonly(ForeignDefinition::output(
                                    ready_definition.clone().map_draft_with_location(None),
                                ));
                                definition
                                    .foreign_definitions
                                    .insert(key, foreign_definition.clone());
                                Some(foreign_definition)
                            } else {
                                let definition_input = match foreign_definition {
                                    ForeignDefinitionDto::String(text_definition) => {
                                        if path::Path::new(text_definition).exists() {
                                            Some(DefinitionInput::File(text_definition.clone()))
                                        } else if let Ok(content) =
                                            serde_json::from_str(text_definition)
                                        {
                                            Some(DefinitionInput::Value(content))
                                        } else if let Some(processor) = definition.processors.get(
                                            text_definition.trim_start_matches(REFERENCE_PREFIX),
                                        ) {
                                            Some(DefinitionInput::Processor(
                                                DraftProcessorOverride::processor(
                                                    LazyRef::referenced_value(
                                                        text_definition.clone(),
                                                        processor.clone(),
                                                    ),
                                                ),
                                            ))
                                        } else {
                                            Some(DefinitionInput::Processor(
                                                DraftProcessorOverride::processor(LazyRef::Ref(
                                                    text_definition.clone(),
                                                )),
                                            ))
                                        }
                                    }
                                    ForeignDefinitionDto::Processor(processor_override) => {
                                        Some(DraftDefinitionInput::Processor(
                                            to_draft_processor_override(
                                                processor_override,
                                                definition,
                                            ),
                                        ))
                                    }
                                    ForeignDefinitionDto::Definition(_) => None,
                                };
                                definition_input.map(|definition_input| {
                                    let foreign_definition =
                                        readonly(ForeignDefinition::input(definition_input));
                                    definition
                                        .foreign_definitions
                                        .insert(key, foreign_definition.clone());
                                    foreign_definition
                                })
                            }
                        }
                    })
                    .flatten();
                (
                    key.clone(),
                    readonly(DraftEdge {
                        definition: foreign_definition,
                        processor: edge
                            .processor
                            .as_ref()
                            .map(|processor| to_node_draft_processor(&processor, &definition)),
                        other: edge.other.clone(),
                    }),
                )
            })
            .collect()
    }
    fn set_draft_metadata(&self, definition: &mut DraftDefinition) {
        definition.metadata = self
            .metadata
            .iter()
            .map(|(key, metadata)| {
                (
                    key.clone(),
                    readonly(DraftMetadata {
                        edges: metadata
                            .edges
                            .iter()
                            .map(|metadata_edge| match metadata_edge {
                                ReferenceOrObjectDto::Reference(reference) => (
                                    reference.clone(),
                                    definition
                                        .edges
                                        .get(reference.trim_start_matches(REFERENCE_PREFIX))
                                        .map(|edge| {
                                            DraftEdgeOverride::edge(LazyRefOrValue::ReferencedValue(
                                                ReferencedValue {
                                                    r#ref: reference.clone(),
                                                    value: edge.clone(),
                                                },
                                            ))
                                        })
                                        .unwrap_or(DraftEdgeOverride::edge(LazyRefOrValue::Ref(
                                            reference.clone(),
                                        ))),
                                ),
                                ReferenceOrObjectDto::Object(metadata_edge) => (
                                    metadata_edge.r#ref.clone(),
                                    definition
                                        .edges
                                        .get(
                                            metadata_edge
                                                .r#ref
                                                .trim_start_matches(REFERENCE_PREFIX),
                                        )
                                        .map(|edge| DraftEdgeOverride {
                                            arguments: to_draft_arguments(
                                                &metadata_edge.arguments,
                                                &definition.arguments,
                                            ),
                                            environments: to_draft_environments(
                                                &metadata_edge.environments,
                                                &definition.environments,
                                            ),
                                            edge: LazyRefOrValue::referenced_value(
                                                metadata_edge.r#ref.clone(),
                                                edge.clone(),
                                            ),
                                            other: metadata_edge.other.clone(),
                                            definition: metadata_edge
                                                .definition
                                                .clone()
                                                .map(Self::map),
                                        })
                                        .unwrap_or(DraftEdgeOverride {
                                            arguments: to_draft_arguments(
                                                &metadata_edge.arguments,
                                                &definition.arguments,
                                            ),
                                            environments: to_draft_environments(
                                                &metadata_edge.environments,
                                                &definition.environments,
                                            ),
                                            edge: LazyRefOrValue::Ref(metadata_edge.r#ref.clone()),
                                            other: metadata_edge.other.clone(),
                                            definition: metadata_edge
                                                .definition
                                                .clone()
                                                .map(Self::map),
                                        }),
                                ),
                            })
                            .collect(),
                        tags: metadata
                            .tags
                            .iter()
                            .map(|tag| {
                                definition
                                    .tags
                                    .get(tag)
                                    .map(|found_tag| {
                                        (
                                            tag.clone(),
                                            LazyRefOrValue::referenced_value(
                                                tag.clone(),
                                                found_tag.clone(),
                                            ),
                                        )
                                    })
                                    .unwrap_or((tag.clone(), LazyRefOrValue::Ref(tag.clone())))
                            })
                            .collect(),
                        other: metadata.other.clone(),
                        processor: metadata
                            .processor
                            .as_ref()
                            .map(|processor| to_node_draft_processor(processor, definition)),
                    }),
                )
            })
            .collect()
    }
}
