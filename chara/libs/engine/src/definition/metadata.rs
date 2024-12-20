use std::collections::HashMap;

use common::merge::Merge;
use serde_json::{Map, Value};

use crate::{processor::DraftProcessorOverride, reference_value::LazyRefOrValue};

use super::{edge::EdgeOverride, tag::RefTag};

#[derive(Debug, Clone)]
pub struct Metadata {
    pub edges: HashMap<String, EdgeOverride>,
    pub tags: HashMap<String, LazyRefOrValue<RefTag>>,
    pub other: Map<String, Value>,
    pub processor: Option<DraftProcessorOverride>,
}
impl Merge for Metadata {
    fn merge(&mut self, other: &Self) {
        self.edges.merge(&other.edges);
        self.tags.merge(&other.tags);
        self.other.merge(&other.other);
        self.processor.merge(&other.processor);
    }
}
