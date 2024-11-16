use std::collections::HashMap;

use engine::{
    cli::{Argument, Environment},
    definition::{Definition, Tag},
    errors::DefinitionError,
};
use types::{thread::Readonly, ThreadError};

use crate::definition::{
    DefinitionDto, EdgeDto, EnvironmentDto, ForeignDefinitionDto, InstallDto, MetadataDto, MetadataEdge, ProcessorDto, ProcessorOverrideDto, ReferenceOrObjectDto, TagDto
};
fn map_arguments(arguments: Vec<Argument>) -> Vec<String> {
    arguments
        .into_iter()
        .map(|arg| match arg {
            Argument::Value(arg) => arg,
            Argument::Reference { name, .. } => name,
        })
        .collect()
}
fn map_tags(tags: &HashMap<String, Readonly<Tag>>) -> HashMap<String, TagDto> {
    tags.iter()
        .map(|(_, tag)| {
            let tag = tag
                .read()
                .map_err(|_| DefinitionError::Thread(ThreadError::Poison))?;
            Ok::<(String, TagDto), DefinitionError>((
                tag.reference.clone(),
                TagDto {
                    tags: map_tags(&tag.tags),
                    label: tag.label.clone(),
                    other: tag.other.clone(),
                },
            ))
        })
        .flatten()
        .collect()
}
fn map_environments(environments: Vec<Environment>) -> Vec<EnvironmentDto> {
    environments
        .into_iter()
        .map(|env| match env {
            Environment::Value(hash_map) => EnvironmentDto::Value(hash_map),
            Environment::Reference { name, .. } => EnvironmentDto::Reference(name),
        })
        .collect()
}
impl DefinitionDto {
    pub fn from_definition(definition: &Definition) -> Self {
        DefinitionDto {
            name: definition.name.clone(),
            location: definition.location.clone(),
            metadata: definition
                .metadata
                .iter()
                .map(|(k, metadata)| {
                    let metadata = metadata
                        .read()
                        .map_err(|_| DefinitionError::Thread(ThreadError::Poison))?;
                    Ok::<(String, MetadataDto), DefinitionError>((
                        k.clone(),
                        MetadataDto {
                            other: metadata.other.clone(),
                            processor: metadata.processor.as_ref().map(|processor| {
                                ReferenceOrObjectDto::<ProcessorOverrideDto>::Object(
                                    ProcessorOverrideDto {
                                        arguments: map_arguments(processor.arguments.clone()),
                                        environments: map_environments(
                                            processor.environments.clone(),
                                        ),
                                        reference: processor.reference.clone(),
                                    },
                                )
                            }),
                            tags: metadata
                                .tags
                                .iter()
                                .map(|(k, tag)| {
                                    let tag = tag.read().map_err(|_| {
                                        DefinitionError::Thread(ThreadError::Poison)
                                    })?;
                                    Ok::<String, DefinitionError>(tag.reference.clone())
                                })
                                .flatten()
                                .collect(),
                            edges: metadata
                                .edges
                                .iter()
                                .map(|(k, edge)| {
                                    ReferenceOrObjectDto::<MetadataEdge>::Object(MetadataEdge {
                                        arguments: map_arguments(edge.arguments.clone()),
                                        definition: edge
                                            .definition
                                            .as_ref()
                                            .map(DefinitionDto::from_definition),
                                        environments: map_environments(edge.environments.clone()),
                                        other: edge.other.clone(),
                                        r#ref: k.clone(),
                                    })
                                })
                                .collect(),
                        },
                    ))
                })
                .flatten()
                .collect(),
            edges: definition
                .edges
                .iter()
                .map(|(k, edge)| {
                    let edge = edge
                        .read()
                        .map_err(|_| DefinitionError::Thread(ThreadError::Poison))?;
                    let foreign_definition = edge
                        .definition.as_ref()
                        .map(|definition| {
                            let definition = definition
                                .read()
                                .map_err(|_| DefinitionError::Thread(ThreadError::Poison))?;
                            Ok(definition
                                .output.as_ref()
                                .map(|definition| {
                                    let definition = DefinitionDto::from_definition(&definition);
                                    ForeignDefinitionDto::Definition(definition)
                                }))
                        })
                        .transpose()?
                        .flatten();
                    let processor = edge.processor.as_ref().map(|processor| {
                        ReferenceOrObjectDto::Object(ProcessorOverrideDto {
                            arguments: map_arguments(processor.arguments.clone()),
                            environments: map_environments(processor.environments.clone()),
                            reference: processor.reference.clone(),
                        })
                    });
                    Ok::<(String, EdgeDto), DefinitionError>((
                        k.clone(),
                        EdgeDto {
                            definition: foreign_definition,
                            other: edge.other.clone(),
                            processor,
                        },
                    ))
                })
                .flatten()
                .collect(),
            tags: definition.tags.get("#").map_or(HashMap::new(), |tag| {
                tag.read()
                    .map_err(|_| DefinitionError::Thread(ThreadError::Poison))
                    .map_or(HashMap::new(), |tag| map_tags(&tag.tags))
            }),
            processors: definition
                .processors
                .iter()
                .map(|(k, processor)| {
                    let processor = processor
                        .read()
                        .map_err(|_| DefinitionError::Thread(ThreadError::Poison))?;
                    Ok::<(String, ProcessorDto), DefinitionError>((
                        k.clone(),
                        ProcessorDto {
                            arguments: map_arguments(processor.arguments.clone()),
                            current_directory: processor.current_directory.clone(),
                            environments: map_environments(processor.environments.clone()),
                            program: processor.program.clone(),
                            install: processor.install.as_ref().map(|install| InstallDto {
                                arguments: map_arguments(install.arguments.clone()),
                                environments: map_environments(install.environments.clone()),
                                current_directory: install.current_directory.clone(),
                                program: install.program.clone(),
                            }),
                        },
                    ))
                })
                .flatten()
                .collect(),
            arguments: definition
                .arguments
                .iter()
                .map(|(k, arguments)| {
                    let arguments = arguments
                        .read()
                        .map_err(|_| DefinitionError::Thread(ThreadError::Poison))?;
                    Ok::<(String, Vec<String>), DefinitionError>((k.clone(), arguments.clone()))
                })
                .flatten()
                .collect(),
            environments: definition
                .environments
                .iter()
                .map(|(k, environment)| {
                    let environment = environment
                        .read()
                        .map_err(|_| DefinitionError::Thread(ThreadError::Poison))?;
                    Ok::<(String, HashMap<String, String>), DefinitionError>((
                        k.clone(),
                        environment.clone(),
                    ))
                })
                .flatten()
                .collect(),
        }
    }
}
