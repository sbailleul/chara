use std::{collections::HashMap, path, sync::Arc};

use engine::{
    definition::definition::Definition,
    definition::{
        edge::{Edge, EdgeOverride},
        foreign_definition::ForeignDefinition,
        input::{BaseDefinitionInput, DraftDefinitionInput},
        install::Install,
        metadata::Metadata,
        tag::{RefTag, Tag},
    },
    processor::{DraftProcessorOverride, Processor},
    reference_value::{LazyRef, LazyRefOrValue, ReferencedValue},
};

use common::thread::{readonly, Readonly};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    definition::{DefinitionDto, ForeignDefinitionDto, ReferenceOrObjectDto},
    mappers::{
        arguments::to_arguments,
        environments::to_environments,
        processors::{to_node_processor, to_processor_override},
        tags::to_tags,
    },
};

impl DefinitionDto {
    pub fn map(self) -> Definition {
        self.map_with_location(None, None)
    }
    pub fn map_overwrite_location(self, location: String) -> Definition {
        let location = Some(location);
        let mut definition = self.map_with_location(location.clone(), None);
        definition.location = location;
        definition
    }
    pub fn map_with_location(
        self,
        location: Option<String>,
        parent: Option<Readonly<Definition>>,
    ) -> Definition {
        let mut definition = Definition::new(
            parent,
            self.name.clone(),
            self.id.clone().unwrap_or(Uuid::new_v4().to_string()),
            self.location.clone().or(location),
            self.arguments(),
            self.environments(),
        );
        self.set_processors(&mut definition);
        self.set_edges(&mut definition);
        definition.tags = self.list_tags();
        self.set_metadata(&mut definition);
        definition
    }

    pub fn arguments(&self) -> HashMap<String, Readonly<Vec<String>>> {
        self.arguments
            .iter()
            .map(|(key, value)| (key.clone(), readonly(value.clone())))
            .collect()
    }
    pub fn environments(&self) -> HashMap<String, Readonly<HashMap<String, String>>> {
        self.environments
            .iter()
            .map(|(key, value)| (key.clone(), readonly(value.clone())))
            .collect()
    }

    pub fn list_tags(&self) -> HashMap<String, Readonly<RefTag>> {
        let root_path = "#".to_string();
        let root_tag = readonly(RefTag {
            r#ref: root_path.clone(),
            value: Tag {
                label: None,
                tags: HashMap::new(),
                other: Value::Null,
            },
        });
        let tags = to_tags(&root_tag, &root_path, &self.tags);

        let mut all_tags = tags
            .iter()
            .map(|(_key, path, _parent_tag, tag)| (path.clone(), tag.clone()))
            .collect::<HashMap<String, Readonly<RefTag>>>();
        all_tags.insert(root_path, root_tag.clone());
        let tags: HashMap<String, Arc<std::sync::RwLock<RefTag>>> = tags
            .into_iter()
            .filter(|(_key, _path, parent_tag, _tag)| Arc::ptr_eq(parent_tag, &root_tag))
            .map(|(_key, path, _parent_tag, tag)| (path.clone(), tag.clone()))
            .collect::<HashMap<String, Readonly<RefTag>>>();
        if let Ok(mut root_tag) = root_tag.write() {
            root_tag.value.tags = tags;
        };
        all_tags
    }

    fn set_processors(&self, definition: &mut Definition) {
        definition.processors = self
            .processors
            .iter()
            .map(|(key, processor)| {
                (
                    key.clone(),
                    readonly(Processor {
                        arguments: to_arguments(&processor.arguments, &definition),
                        program: processor.program.clone(),
                        install: processor.install.as_ref().map(|install| Install {
                            arguments: to_arguments(&install.arguments, &definition),
                            environments: to_environments(&install.environments, &definition),
                            program: install.program.clone(),
                            current_directory: install.current_directory.clone(),
                        }),
                        environments: to_environments(&processor.environments, &definition),
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
                                    ready_definition.clone().map(),
                                ));
                                definition
                                    .foreign_definitions
                                    .insert(key, foreign_definition.clone());
                                Some(foreign_definition)
                            } else {
                                let definition_input = match foreign_definition {
                                    ForeignDefinitionDto::String(text_definition) => {
                                        if path::Path::new(text_definition).exists() {
                                            Some(BaseDefinitionInput::File(text_definition.clone()))
                                        } else if let Ok(content) =
                                            serde_json::from_str(text_definition)
                                        {
                                            Some(BaseDefinitionInput::Value(content))
                                        } else if let Some(processor) =
                                            definition.find_processor(text_definition)
                                        {
                                            Some(BaseDefinitionInput::Processor(
                                                DraftProcessorOverride::processor(&Some(
                                                    LazyRef::new_referenced_value(
                                                        text_definition.clone(),
                                                        processor.clone(),
                                                    ),
                                                )),
                                            ))
                                        } else {
                                            Some(BaseDefinitionInput::Processor(
                                                DraftProcessorOverride::processor(&Some(
                                                    LazyRef::Ref(text_definition.clone()),
                                                )),
                                            ))
                                        }
                                    }
                                    ForeignDefinitionDto::Processor(processor_override) => {
                                        Some(DraftDefinitionInput::Processor(
                                            to_processor_override(processor_override, definition),
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
                    readonly(Edge {
                        definition: foreign_definition,
                        processor: edge
                            .processor
                            .as_ref()
                            .map(|processor| to_node_processor(&processor, definition)),
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
                            .map(|metadata_edge| match metadata_edge {
                                ReferenceOrObjectDto::Reference(reference) => (
                                    reference.clone(),
                                    definition
                                        .find_edge(reference)
                                        .map(|edge| {
                                            EdgeOverride::edge(LazyRefOrValue::ReferencedValue(
                                                ReferencedValue {
                                                    r#ref: reference.clone(),
                                                    value: edge.clone(),
                                                },
                                            ))
                                        })
                                        .unwrap_or(EdgeOverride::edge(LazyRefOrValue::Ref(
                                            reference.clone(),
                                        ))),
                                ),
                                ReferenceOrObjectDto::Object(metadata_edge) => (
                                    metadata_edge.r#ref.clone(),
                                    definition
                                        .find_edge(&metadata_edge.r#ref)
                                        .map(|edge| EdgeOverride {
                                            arguments: to_arguments(
                                                &metadata_edge.arguments,
                                                definition,
                                            ),
                                            environments: to_environments(
                                                &metadata_edge.environments,
                                                definition,
                                            ),
                                            edge: LazyRefOrValue::to_referenced_value(
                                                metadata_edge.r#ref.clone(),
                                                edge.clone(),
                                            ),
                                            other: metadata_edge.other.clone(),
                                            definition: metadata_edge
                                                .definition
                                                .clone()
                                                .map(Self::map),
                                        })
                                        .unwrap_or(EdgeOverride {
                                            arguments: to_arguments(
                                                &metadata_edge.arguments,
                                                definition,
                                            ),
                                            environments: to_environments(
                                                &metadata_edge.environments,
                                                definition,
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
                                    .find_tag(tag)
                                    .map(|found_tag| {
                                        (
                                            tag.clone(),
                                            LazyRefOrValue::to_referenced_value(
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
                            .map(|processor| to_node_processor(processor, definition)),
                    }),
                )
            })
            .collect()
    }
}
