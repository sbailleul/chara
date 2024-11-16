use std::{collections::HashMap, path, sync::Arc};

use engine::{
    cli::{Argument, Environment},
    definition::{
        Definition, DefinitionInput, Edge, EdgeOverride, ForeignDefinition, Install, Metadata,
        Processor, ProcessorOverride, Tag,
    },
};
use serde_json::Value;

use types::thread::{readonly, Readonly};

use crate::definition::{
    DefinitionDto, EnvironmentDto, ForeignDefinitionDto, ProcessorOverrideDto,
    ReferenceOrObjectDto, TagDto,
};
const REFERENCE_PREFIX: &str = "#/";
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
    pub fn map(self, location: Option<String>) -> Definition {
        let mut definition = Definition {
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
        self.set_processors(&mut definition);
        self.set_edges(&mut definition);
        self.set_tags(&mut definition);
        self.set_metadata(&mut definition);
        definition
    }

    fn set_tags(&self, definition: &mut Definition) {
        let root_path = "#".to_string();
        let root_tag = readonly(Tag {
            reference: root_path.clone(),
            label: None,
            tags: HashMap::new(),
            other: Value::Null,
        });
        let tags = extract_tags(&root_tag, &root_path, &self.tags);

        definition.tags = tags
            .iter()
            .map(|(_key, path, _parent_tag, tag)| (path.clone(), tag.clone()))
            .collect::<HashMap<String, Readonly<Tag>>>();
        definition.tags.insert(root_path, root_tag.clone());
        let tags: HashMap<String, Arc<std::sync::RwLock<Tag>>> = tags
            .into_iter()
            .filter(|(_key, _path, parent_tag, _tag)| Arc::ptr_eq(parent_tag, &root_tag))
            .map(|(_key, path, _parent_tag, tag)| (path.clone(), tag.clone()))
            .collect::<HashMap<String, Readonly<Tag>>>();
        if let Ok(mut root_tag) = root_tag.write() {
            root_tag.tags = tags;
        };
    }

    fn set_processors(&self, definition: &mut Definition) {
        definition.processors = self
            .processors
            .iter()
            .map(|(key, processor)| {
                (
                    key.clone(),
                    readonly(Processor {
                        arguments: map_arguments(&processor.arguments, &definition.arguments),
                        program: processor.program.clone(),
                        install: processor.install.as_ref().map(|install| Install {
                            arguments: map_arguments(&install.arguments, &definition.arguments),
                            environments: map_environments(
                                &install.environments,
                                &definition.environments,
                            ),
                            program: install.program.clone(),
                            current_directory: install.current_directory.clone(),
                        }),
                        environments: map_environments(
                            &processor.environments,
                            &definition.environments,
                        ),
                        current_directory: processor.current_directory.clone(),
                    }),
                )
            })
            .collect()
    }

    fn set_edges(&self, definition: &mut Definition) {
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
                                    ready_definition.clone().map(None),
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
                                                ProcessorOverride::processor(
                                                    processor,
                                                    text_definition,
                                                ),
                                            ))
                                        } else {
                                            None
                                        }
                                    }
                                    ForeignDefinitionDto::Processor(processor_override) => {
                                        map_processor_override(processor_override, definition)
                                            .map(DefinitionInput::Processor)
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
                    readonly(Edge {
                        definition: foreign_definition,
                        processor: edge
                            .processor
                            .as_ref()
                            .and_then(|processor| map_node_processor(&processor, &definition)),
                        other: edge.other.clone(),
                    }),
                )
            })
            .collect()
    }
    fn set_metadata(&self, definition: &mut Definition) {
        definition.metadata = self
            .metadata
            .iter()
            .map(|(key, metadata)| {
                (
                    key.clone(),
                    readonly(Metadata {
                        edges: metadata
                            .edges
                            .iter()
                            .map(|metadata_edge| {
                                let reference = metadata_edge.reference();

                                definition
                                    .edges
                                    .get(reference.trim_start_matches(REFERENCE_PREFIX))
                                    .cloned()
                                    .map(|edge| {
                                        (
                                            reference,
                                            EdgeOverride {
                                                edge,
                                                arguments: map_arguments(
                                                    &metadata_edge.arguments(),
                                                    &definition.arguments,
                                                ),
                                                environments: map_environments(
                                                    &metadata_edge.environments(),
                                                    &definition.environments,
                                                ),
                                                definition: metadata_edge
                                                    .definition()
                                                    .map(|def| def.map(None)),
                                                other: metadata_edge.other(),
                                            },
                                        )
                                    })
                            })
                            .flatten()
                            .collect(),
                        tags: metadata
                            .tags
                            .iter()
                            .map(|tag| {
                                definition
                                    .tags
                                    .get(tag)
                                    .map(|found_tag| (tag.clone(), found_tag.clone()))
                            })
                            .flatten()
                            .collect(),
                        other: metadata.other.clone(),
                        processor: metadata
                            .processor
                            .as_ref()
                            .and_then(|processor| map_node_processor(processor, definition)),
                    }),
                )
            })
            .collect()
    }
}
fn map_node_processor(
    node_processor: &ReferenceOrObjectDto<ProcessorOverrideDto>,
    definition: &Definition,
) -> Option<ProcessorOverride> {
    match node_processor {
        ReferenceOrObjectDto::Reference(reference) => definition
            .processors
            .get(reference.trim_start_matches(REFERENCE_PREFIX))
            .map(|processor| ProcessorOverride::processor(processor, reference)),
        ReferenceOrObjectDto::Object(processor_override) => {
            map_processor_override(processor_override, definition)
        }
    }
}

fn map_processor_override(
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
            arguments: map_arguments(&processor_override.arguments, &definition.arguments),
            environments: map_environments(
                &processor_override.environments,
                &definition.environments,
            ),
            processor: processor.clone(),
        })
}
fn map_arguments(
    dto_arguments: &Vec<String>,
    arguments: &HashMap<String, Readonly<Vec<String>>>,
) -> Vec<Argument> {
    dto_arguments
        .iter()
        .map(|argument| {
            if argument.starts_with(REFERENCE_PREFIX) {
                arguments
                    .get(argument.trim_start_matches(REFERENCE_PREFIX))
                    .map(|v| v.clone())
                    .map(|reference| Argument::Reference {
                        name: argument.clone(),
                        arguments: reference,
                    })
            } else {
                Some(Argument::Value(argument.clone()))
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
            EnvironmentDto::Reference(reference) => environments
                .get(reference.trim_start_matches(REFERENCE_PREFIX))
                .map(|v| Environment::Reference {
                    name: reference.clone(),
                    environment: v.clone(),
                }),
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
            let path = parent_path.to_owned() + "/" + k;
            if tag_dto.tags.is_empty() {
                vec![(
                    k.clone(),
                    path.clone(),
                    parent.clone(),
                    readonly(Tag {
                        reference: k.clone(),
                        label: tag_dto.label.clone(),
                        tags: HashMap::new(),
                        other: tag_dto.other.clone(),
                    }),
                )]
            } else {
                let tag = readonly(Tag {
                    reference: k.clone(),
                    label: tag_dto.label.clone(),
                    tags: HashMap::new(),
                    other: tag_dto.other.clone(),
                });

                let mut inner_tags = extract_tags(&tag, &path, &tag_dto.tags);
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
                inner_tags.push((k.clone(), path, parent.clone(), tag));
                inner_tags
            }
        })
        .flatten()
        .collect()
}
