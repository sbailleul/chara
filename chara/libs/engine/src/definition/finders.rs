use std::collections::HashMap;

use common::thread::Readonly;

use crate::{definition, processor::Processor};

use super::{definition::Definition, edge::Edge, metadata::Metadata, tag::RefTag};

impl Definition {
    fn find_root(&self) -> Definition {
        if let Some(parent) = self.parent.as_ref().and_then(|parent| parent.read().ok()) {
            parent.find_root()
        } else {
            self.clone()
        }
    }
    fn find_definition_by_segments(
        &self,
        segments: &[&str],
    ) -> Option<(Option<String>, Definition)> {
        dbg!(&segments);
        match &segments[..] {
            [] => Some((None, self.clone())),
            ["#"] => Some((None, self.find_root())),
            [".."] => self
                .parent
                .as_ref()
                .and_then(|parent| parent.read().ok().map(|parent| (None, parent.clone()))),
            [token] => Some((Some(token.to_string()), self.clone())),
            [".", tail @ ..] => self.find_definition_by_segments(tail),
            ["#", tail @ ..] => self.find_root().find_definition_by_segments(tail),
            ["..", tail @ ..] => self
                .parent
                .as_ref()
                .and_then(|parent| parent.read().ok())
                .and_then(|parent| parent.find_definition_by_segments(tail)),
            [metadata, edge, tail @ ..] => self
                .metadata
                .get(&metadata.to_string())
                .as_ref()
                .and_then(|metadata| metadata.read().ok())
                .and_then(|metadata| {
                    metadata
                        .edges
                        .get(&edge.to_string())
                        .as_ref()
                        .and_then(|edge| {
                            edge.definition
                                .as_ref()
                                .and_then(|definition| definition.find_definition_by_segments(tail))
                        })
                }),
        }
    }
    pub fn find_definition_by_path(&self, path: &String) -> Option<(Option<String>, Definition)> {
        let segments = path
            .split("/")
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>();
        self.find_definition_by_segments(&segments)
    }
    pub fn find_processor(&self, path: &String) -> Option<Readonly<Processor>> {
        self.find_definition_by_path(path)
            .and_then(|(processor, definition)| {
                processor.and_then(|processor| definition.processors.get(&processor).cloned())
            })
    }
    pub fn find_tag(&self, path: &String) -> Option<Readonly<RefTag>> {
        self.find_definition_by_path(path)
            .and_then(|(tag, definition)| tag.and_then(|tag| definition.tags.get(&tag).cloned()))
    }

    pub fn find_metadata(&self, path: &String) -> Option<Readonly<Metadata>> {
        self.find_definition_by_path(path)
            .and_then(|(metadata, definition)| {
                metadata.and_then(|metadata| definition.metadata.get(&metadata).cloned())
            })
    }

    pub fn find_edge(&self, path: &String) -> Option<Readonly<Edge>> {
        self.find_definition_by_path(path)
            .and_then(|(edge, definition)| {
                edge.and_then(|edge| definition.edges.get(&edge).cloned())
            })
    }

    pub fn find_argument(&self, path: &String) -> Option<Readonly<Vec<String>>> {
        self.find_definition_by_path(path)
            .and_then(|(argument, definition)| {
                argument.and_then(|argument| definition.arguments.get(&argument).cloned())
            })
    }

    pub fn find_environment(&self, path: &String) -> Option<Readonly<HashMap<String, String>>> {
        self.find_definition_by_path(path)
            .and_then(|(environment, definition)| {
                environment
                    .and_then(|environment| definition.environments.get(&environment).cloned())
            })
    }
}
