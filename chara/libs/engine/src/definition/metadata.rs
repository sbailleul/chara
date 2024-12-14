use std::collections::HashMap;

use common::{merge::Merge};
use serde_json::{Map, Value};



#[derive(Debug, Clone)]
pub struct Metadata<TEdge, TProcessor, TTag> {
    pub edges: HashMap<String, TEdge>,
    pub tags: HashMap<String, TTag>,
    pub other: Map<String, Value>,
    pub processor: Option<TProcessor>,
}
impl<TEdge: Merge + Clone, TProcessor: Merge + Clone, TTag: Merge + Clone> Merge
    for Metadata<TEdge, TProcessor, TTag>
{
    fn merge(&mut self, other: &Self) {
        self.edges.merge(&other.edges);
        self.tags.merge(&other.tags);
        self.other.merge(&other.other);
        self.processor.merge(&other.processor);
    }
}
