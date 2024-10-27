use core::hash;
use std::{collections::HashMap, hash::Hash, path, sync::Arc};

use engine::{
    cli::{Argument, Environment},
    definition::{
        Definition, DefinitionInput, Edge, ForeignDefinition, Install, Metadata, Processor, Tag,
    },
};
use serde_json::Map;

use types::thread::{readonly, Readonly};

use crate::definition::{DefinitionDto, EnvironmentDto, ProcessorDefinitionDto, TagDto};

impl DefinitionDto {
    fn arguments(&self) -> HashMap<String, Readonly<Vec<String>>> {
        self.arguments
            .iter()
            .map(|(key, value)| (key.clone(), readonly(value.clone())))
            .collect()
    }
    fn environments(&self) -> HashMap<String, Readonly<HashMap<String, String>>> {
        self.environments
            .iter()
            .map(|(key, value)| (key.clone(), readonly(value.clone())))
            .collect()
    }
    pub fn map(self) -> Definition {
        let mut chara = Definition {
            name: self.name.clone(),
            arguments: self.arguments(),
            environments: self.environments(),
            edges: HashMap::new(),
            metadata: HashMap::new(),
            processors: HashMap::new(),
            tags: HashMap::new(),
            foreign_definitions: HashMap::new(),
        };
        self.set_processors(&mut chara);
        self.set_edges(&mut chara);
        self.set_tags(&mut chara);
        self.set_metadata(&mut chara);
        chara
    }

    fn set_tags(&self, chara: &mut Definition) {
        let tags = extract_tags(
            &readonly(Tag {
                label: None,
                tags: HashMap::new(),
                other: Map::new(),
            }),
            &"#".to_string(),
            &self.tags,
        );
        chara.tags = tags
            .into_iter()
            .map(|(_key, program, _parent_tag, tag)| (program, tag))
            .collect();
    }

    fn set_processors(&self, chara: &mut Definition) {
        chara.processors = self
            .processors
            .iter()
            .map(|(key, processor)| {
                (
                    key.clone(),
                    readonly(Processor {
                        arguments: map_arguments(&processor.arguments, &chara.arguments),
                        program: processor.program.clone(),
                        install: processor.install.as_ref().map(|install| Install {
                            arguments: map_arguments(&install.arguments, &chara.arguments),
                            environments: map_environments(
                                &install.environments,
                                &chara.environments,
                            ),
                            program: install.program.clone(),
                        }),
                        environments: map_environments(
                            &processor.environments,
                            &chara.environments,
                        ),
                    }),
                )
            })
            .collect()
    }

    fn set_edges(&self, chara: &mut Definition) {
        chara.edges = self
            .edges
            .iter()
            .map(|(key, edge)| {
                let definition = edge
                    .definition
                    .as_ref()
                    .map(|definition| {
                        if let Some(definition) = chara.foreign_definitions.get(definition) {
                            Some(definition.clone())
                        } else {
                            let definition_input = if path::Path::new(definition).exists() {
                                Some(DefinitionInput::File(definition.clone()))
                            } else if let Ok(content) = serde_json::from_str(definition) {
                                Some(DefinitionInput::Value(content))
                            } else if let Some(processor) = chara.processors.get(definition) {
                                Some(DefinitionInput::Processor(processor.clone()))
                            } else {
                                None
                            };
                            definition_input.map(|definition_input| {
                                let foreign_definition =
                                    readonly(ForeignDefinition::input(definition_input));
                                chara
                                    .foreign_definitions
                                    .insert(definition.clone(), foreign_definition.clone());
                                foreign_definition
                            })
                        }
                    })
                    .flatten();
                (
                    key.clone(),
                    readonly(Edge {
                        definition,
                        processor: edge.processor.as_ref().and_then(|program| {
                            chara
                                .processors
                                .get(program.trim_start_matches("#/"))
                                .cloned()
                        }),
                        other: edge.other.clone(),
                    }),
                )
            })
            .collect()
    }
    fn set_metadata(&self, chara: &mut Definition) {
        chara.metadata = self
            .metadata
            .iter()
            .map(|(key, metadata)| {
                (
                    key.clone(),
                    readonly(Metadata {
                        edges: metadata
                            .edges
                            .iter()
                            .map(|program| {
                                chara
                                    .edges
                                    .get(program.trim_start_matches("#/"))
                                    .cloned()
                                    .map(|edge| (program.clone(), edge))
                            })
                            .flatten()
                            .collect(),
                        tags: metadata
                            .tags
                            .iter()
                            .map(|tag| {
                                chara
                                    .tags
                                    .get(tag)
                                    .map(|found_tag| (tag.clone(), found_tag.clone()))
                            })
                            .flatten()
                            .collect(),
                        other: metadata.other.clone(),
                        processor: metadata.processor.as_ref().and_then(|program| {
                            chara
                                .processors
                                .get(program.trim_start_matches("#/"))
                                .cloned()
                        }),
                    }),
                )
            })
            .collect()
    }
}

fn map_arguments(
    dto_arguments: &Vec<String>,
    arguments: &HashMap<String, Readonly<Vec<String>>>,
) -> Vec<Argument> {
    dto_arguments
        .iter()
        .map(|program| {
            if program.starts_with("#/") {
                arguments
                    .get(program.trim_start_matches("#/"))
                    .map(|v| v.clone())
                    .map(|reference| Argument::Reference(reference))
            } else {
                Some(Argument::Value(program.clone()))
            }
        })
        .flatten()
        .collect()
}
fn map_environments(
    dto_environments: &Vec<EnvironmentDto>,
    environments: &HashMap<String, Readonly<HashMap<String, String>>>,
) -> Vec<Environment> {
    dto_environments
        .iter()
        .map(|environment| match environment {
            EnvironmentDto::Reference(program) => environments
                .get(program.trim_start_matches("#/"))
                .map(|v| Environment::Reference(v.clone())),
            EnvironmentDto::Value(hash_map) => Some(Environment::Value(hash_map.clone())),
        })
        .flatten()
        .collect()
}
fn extract_tags(
    parent: &Readonly<Tag>,
    parent_path: &String,
    tags: &HashMap<String, TagDto>,
) -> Vec<(String, String, Readonly<Tag>, Readonly<Tag>)> {
    tags.iter()
        .map(|(k, tag_dto)| {
            let program = parent_path.to_owned() + "/" + k;
            if tag_dto.tags.is_empty() {
                vec![(
                    k.clone(),
                    program,
                    parent.clone(),
                    readonly(Tag {
                        label: tag_dto.label.clone(),
                        tags: HashMap::new(),
                        other: tag_dto.other.clone(),
                    }),
                )]
            } else {
                let tag = readonly(Tag {
                    label: tag_dto.label.clone(),
                    tags: HashMap::new(),
                    other: tag_dto.other.clone(),
                });

                let mut inner_tags = extract_tags(&tag, &program, &tag_dto.tags);
                if let Ok(mut tag_value) = tag.write() {
                    tag_value.tags = inner_tags
                        .iter()
                        .filter(|(_k, _parent_path, parent_tag, _inner_tag)| {
                            Arc::ptr_eq(&parent_tag, &tag)
                        })
                        .map(|(k, _parent_path, _parent_tag, inner_tag)| {
                            (k.clone(), inner_tag.clone())
                        })
                        .collect()
                }
                inner_tags.push((k.clone(), program, parent.clone(), tag));
                inner_tags
            }
        })
        .flatten()
        .collect()
}
