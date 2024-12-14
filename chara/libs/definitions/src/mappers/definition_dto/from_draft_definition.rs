use std::collections::HashMap;

use common::ThreadError;
use engine::{
    draft::draft_definition::DraftDefinition, errors::CharaError, reference_value::LazyRefOrValue,
};

use crate::{
    definition::{
        DefinitionDto, EdgeDto, ForeignDefinitionDto, InstallDto, MetadataDto, MetadataEdge,
        ProcessorDto, ProcessorOverrideDto, ReferenceOrObjectDto, TagDto,
    },
    mappers::{
        arguments::from_draft_arguments, environments::from_draft_environments, tags::from_tags,
    },
};

impl DefinitionDto {
    pub fn from_draft_definition(definition: &DraftDefinition) -> Self {
        DefinitionDto {
            id: Some(definition.id.clone()),
            name: definition.name.clone(),
            location: definition.location.clone(),
            metadata: Self::read_draft_metadata(definition),
            edges: Self::read_draft_edges(definition),
            tags: Self::read_draft_tags(definition),
            processors: Self::read_draft_processors(definition),
            arguments: Self::read_draft_arguments(definition),
            environments: Self::read_draft_environments(definition),
        }
    }
    fn read_draft_edges(definition: &DraftDefinition) -> HashMap<String, EdgeDto> {
        definition
            .edges
            .iter()
            .map(|(k, edge)| {
                let edge = edge
                    .read()
                    .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
                let foreign_definition = edge
                    .definition
                    .as_ref()
                    .map(|definition| {
                        let definition = definition
                            .read()
                            .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
                        Ok(definition.output.as_ref().map(|definition| {
                            let definition = DefinitionDto::from_draft_definition(&definition);
                            ForeignDefinitionDto::Definition(definition)
                        }))
                    })
                    .transpose()?
                    .flatten();
                let processor = edge.processor.as_ref().map(|processor| {
                    ReferenceOrObjectDto::Object(ProcessorOverrideDto {
                        arguments: from_draft_arguments(processor.arguments.clone()),
                        environments: from_draft_environments(processor.environments.clone()),
                        reference: processor
                            .processor
                            .as_ref()
                            .map(|processor| processor.reference()),
                    })
                });
                Ok::<(String, EdgeDto), CharaError>((
                    k.clone(),
                    EdgeDto {
                        definition: foreign_definition,
                        other: edge.other.clone(),
                        processor,
                    },
                ))
            })
            .flatten()
            .collect()
    }
    fn read_draft_environments(
        definition: &DraftDefinition,
    ) -> HashMap<String, HashMap<String, String>> {
        definition
            .environments
            .iter()
            .map(|(k, environment)| {
                let environment = environment
                    .read()
                    .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
                Ok::<(String, HashMap<String, String>), CharaError>((
                    k.clone(),
                    environment.clone(),
                ))
            })
            .flatten()
            .collect()
    }
    fn read_draft_arguments(definition: &DraftDefinition) -> HashMap<String, Vec<String>> {
        definition
            .arguments
            .iter()
            .map(|(k, arguments)| {
                let arguments = arguments
                    .read()
                    .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
                Ok::<(String, Vec<String>), CharaError>((k.clone(), arguments.clone()))
            })
            .flatten()
            .collect()
    }
    fn read_draft_processors(definition: &DraftDefinition) -> HashMap<String, ProcessorDto> {
        definition
            .processors
            .iter()
            .map(|(k, processor)| {
                let processor = processor
                    .read()
                    .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
                Ok::<(String, ProcessorDto), CharaError>((
                    k.clone(),
                    ProcessorDto {
                        arguments: from_draft_arguments(processor.arguments.clone()),
                        current_directory: processor.current_directory.clone(),
                        environments: from_draft_environments(processor.environments.clone()),
                        program: processor.program.clone(),
                        install: processor.install.as_ref().map(|install| InstallDto {
                            arguments: from_draft_arguments(install.arguments.clone()),
                            environments: from_draft_environments(install.environments.clone()),
                            current_directory: install.current_directory.clone(),
                            program: install.program.clone(),
                        }),
                    },
                ))
            })
            .flatten()
            .collect()
    }
    fn read_draft_tags(definition: &DraftDefinition) -> HashMap<String, TagDto> {
        definition.tags.get("#").map_or(HashMap::new(), |tag| {
            tag.read()
                .map_err(|_| CharaError::Thread(ThreadError::Poison))
                .map_or(HashMap::new(), |tag| from_tags(&tag.value.tags))
        })
    }
    fn read_draft_metadata(definition: &DraftDefinition) -> HashMap<String, MetadataDto> {
        definition
            .metadata
            .iter()
            .map(|(k, metadata)| {
                let metadata = metadata
                    .read()
                    .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
                Ok::<(String, MetadataDto), CharaError>((
                    k.clone(),
                    MetadataDto {
                        other: metadata.other.clone(),
                        processor: metadata.processor.as_ref().map(|processor| {
                            ReferenceOrObjectDto::<ProcessorOverrideDto>::Object(
                                ProcessorOverrideDto {
                                    arguments: from_draft_arguments(processor.arguments.clone()),
                                    environments: from_draft_environments(
                                        processor.environments.clone(),
                                    ),
                                    reference: processor
                                        .processor
                                        .as_ref()
                                        .map(|processor| processor.reference()),
                                },
                            )
                        }),
                        tags: metadata
                            .tags
                            .iter()
                            .map(|(_, tag)| tag.reference())
                            .flatten()
                            .collect(),
                        edges: metadata
                            .edges
                            .iter()
                            .map(|(k, edge)| {
                                ReferenceOrObjectDto::<MetadataEdge>::Object(MetadataEdge {
                                    arguments: from_draft_arguments(edge.arguments.clone()),
                                    definition: edge
                                        .definition
                                        .as_ref()
                                        .map(DefinitionDto::from_definition),

                                    environments: from_draft_environments(
                                        edge.environments.clone(),
                                    ),
                                    other: edge.other.clone(),
                                    r#ref: k.clone(),
                                })
                            })
                            .collect(),
                    },
                ))
            })
            .flatten()
            .collect()
    }
}
