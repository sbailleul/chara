use std::{collections::HashMap, path, sync::Arc};

use engine::{
    definition::{definition::{
        Definition, Install, Metadata, Tag,
    }, edge::{Edge, EdgeOverride}, foreign_definition::ForeignDefinition, input::DefinitionInput},
    processor::{Processor, ProcessorOverride},
};
use serde_json::Value;

use common::thread::{readonly, Readonly};
use uuid::Uuid;

use crate::{
    definition::{DefinitionDto, ForeignDefinitionDto},
    mappers::{
        arguments::to_arguments,
        environments::to_environments,
        processors::{to_node_processor, to_processor_override},
        tags::to_tags,
        REFERENCE_PREFIX,
    },
};

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
    pub fn map_overwrite_location(self, location: String) -> Definition {
        let location = Some(location);
        let mut definition = self.map(location.clone());
        definition.location = location;
        definition
    }
    pub fn map(self, location: Option<String>) -> Definition {
        let mut definition = Definition {
            parent: None,
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
        let tags = to_tags(&root_tag, &root_path, &self.tags);

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
                        arguments: to_arguments(&processor.arguments, &definition.arguments),
                        program: processor.program.clone(),
                        install: processor.install.as_ref().map(|install| Install {
                            arguments: to_arguments(&install.arguments, &definition.arguments),
                            environments: to_environments(
                                &install.environments,
                                &definition.environments,
                            ),
                            program: install.program.clone(),
                            current_directory: install.current_directory.clone(),
                        }),
                        environments: to_environments(
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
                                        to_processor_override(processor_override, definition)
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
                            .and_then(|processor| to_node_processor(&processor, &definition)),
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
                                                arguments: to_arguments(
                                                    &metadata_edge.arguments(),
                                                    &definition.arguments,
                                                ),
                                                environments: to_environments(
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
                            .and_then(|processor| to_node_processor(processor, definition)),
                    }),
                )
            })
            .collect()
    }
}
