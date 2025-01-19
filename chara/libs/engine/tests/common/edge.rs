use common::thread::Readonly;
use engine::{
    definition::{definition::Definition, edge::Edge, foreign_definition::ForeignDefinition},
    processor::{DraftProcessorOverride, ProcessorOverride},
};
use serde_json::{Map, Value};

pub struct EdgeBuilder {
    edge: Edge,
}

impl EdgeBuilder {
   pub  fn new() -> Self {
        Self { edge: empty_edge() }
    }
   pub  fn with_definition(&mut self, definition: Readonly<ForeignDefinition>) -> &mut Self {
        self.edge.definition = Some(definition);
        self
    }
   pub  fn with_other(&mut self, other: Map<String, Value>) -> &mut Self {
        self.edge.other = other;
        self
    }
   pub  fn with_processor(&mut self, processor: DraftProcessorOverride) -> &mut Self {
        self.edge.processor = Some(processor);
        self
    }
   pub  fn build(&mut self) -> Edge {
        let edge = self.edge.clone();
        self.edge = empty_edge();
        edge
    }
}
fn empty_edge() -> Edge {
    Edge {
        definition: None,
        other: Map::new(),
        processor: None,
    }
}
