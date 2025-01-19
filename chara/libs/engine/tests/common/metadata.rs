use std::collections::HashMap;

use engine::{
    definition::{edge::EdgeOverride, metadata::Metadata, tag::RefTag},
    processor::DraftProcessorOverride,
    reference_value::LazyRefOrValue,
};
use serde_json::{Map, Value};

use super::{definition::DefinitionBuilder, edge_override::EdgeOverrideBuilder};

pub struct MetadataBuilder {
    metadata: Metadata,
}

impl MetadataBuilder {
    pub fn new() -> MetadataBuilder {
        Self {
            metadata: empty_metadata(),
        }
    }
    pub fn with_edges(&mut self, edges: HashMap<String, EdgeOverride>) -> &mut Self {
        self.metadata.edges = edges;
        self
    }

    pub fn with_edge(&mut self, (edge_key, edge): (&str, EdgeOverride)) -> &mut Self {
        self.metadata
            .edges
            .insert(edge_key.to_string(), edge.clone());
        self
    }
    pub fn with_other(&mut self, other: Map<String, Value>) -> &mut Self {
        self.metadata.other = other;
        self
    }
    pub fn with_processor(&mut self, processor: Option<DraftProcessorOverride>) -> &mut Self {
        self.metadata.processor = processor;
        self
    }
    pub fn with_tags(&mut self, tags: HashMap<String, LazyRefOrValue<RefTag>>) -> &mut Self {
        self.metadata.tags = tags;
        self
    }

    pub fn build(&mut self) -> Metadata {
        let metadata = self.metadata.clone();
        self.metadata = empty_metadata();
        metadata
    }
}

fn empty_metadata() -> Metadata {
    Metadata {
        edges: HashMap::new(),
        other: Map::new(),
        processor: None,
        tags: HashMap::new(),
    }
}
