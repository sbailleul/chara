use std::{collections::HashMap, sync::Arc};

use engine::{
    cli::{Argument, Environment},
    definition::{
        Definition, DefinitionInput, Edge, Enricher, ForeignDefinition, HttpDefinition, Install,
        Metadata, Tag,
    },
};
use serde_json::Map;

use types::thread::{readonly, Readonly};

use crate::definition::{
    DefinitionDto, EnvironmentDto, ForeignDefinitionDto, HttpDefinitionDto, TagDto,
};

impl HttpDefinitionDto {
    pub fn map(
        &self,
        arguments: &HashMap<String, Readonly<Vec<String>>>,
        environments: &HashMap<String, Readonly<HashMap<String, String>>>,
    ) -> HttpDefinition {
        HttpDefinition {
            arguments: map_arguments(&self.arguments, arguments),
            environments: map_environments(&self.environments, environments),
            uri: self.uri.clone(),
        }
    }
}
impl ForeignDefinitionDto {
    pub fn key(&self) -> String {
        match self {
            ForeignDefinitionDto::Http(http_definition_dto) => http_definition_dto.uri.clone(),
        }
    }
}

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
            enrichers: HashMap::new(),
            tags: HashMap::new(),
            foreign_definitions: HashMap::new(),
        };
        self.set_enrichers(&mut chara);
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

    fn set_enrichers(&self, chara: &mut Definition) {
        chara.enrichers = self
            .enrichers
            .iter()
            .map(|(key, enricher)| {
                (
                    key.clone(),
                    readonly(Enricher {
                        arguments: map_arguments(&enricher.arguments, &chara.arguments),
                        program: enricher.program.clone(),
                        install: enricher.install.as_ref().map(|install| Install {
                            arguments: map_arguments(&install.arguments, &chara.arguments),
                            environments: map_environments(
                                &install.environments,
                                &chara.environments,
                            ),
                            program: install.program.clone(),
                        }),
                        environments: map_environments(&enricher.environments, &chara.environments),
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
                let definition = edge.definition.as_ref().map(|definition| {
                    if let Some(definition) = chara.foreign_definitions.get(&definition.key()) {
                        definition.clone()
                    } else {
                        readonly(ForeignDefinition {
                            input: match definition {
                                ForeignDefinitionDto::Http(http_definition_dto) => {
                                    DefinitionInput::Http(
                                        http_definition_dto
                                            .map(&chara.arguments, &chara.environments),
                                    )
                                }
                            },
                            output: None,
                        })
                    }
                });
                (
                    key.clone(),
                    readonly(Edge {
                        definition,
                        enricher: edge.enricher.as_ref().and_then(|program| {
                            chara
                                .enrichers
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
                        enricher: metadata.enricher.as_ref().and_then(|program| {
                            chara
                                .enrichers
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
