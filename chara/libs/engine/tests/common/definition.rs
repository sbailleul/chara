use std::collections::HashMap;

use common::thread::{readonly, Readonly};
use engine::{
    definition::{
        definition::Definition,
        edge::{Edge, EdgeOverride},
        foreign_definition::ForeignDefinition,
        metadata::Metadata,
        tag::RefTag,
    },
    processor::Processor,
};

pub struct DefinitionBuilder {
    definition: Definition,
}

impl DefinitionBuilder {
    pub fn new() -> DefinitionBuilder {
        Self {
            definition: empty_definition(),
        }
    }
    pub fn with_parent(&mut self, parent: Definition) -> &mut DefinitionBuilder {
        self.definition.parent = Some(readonly(parent));
        self
    }
    pub fn with_name(&mut self, name: &str) -> &mut DefinitionBuilder {
        self.definition.name = name.to_string();
        self
    }
    pub fn with_id(&mut self, id: &str) -> &mut DefinitionBuilder {
        self.definition.id = id.to_string();
        self
    }
    pub fn with_location(&mut self, location: &str) -> &mut DefinitionBuilder {
        self.definition.location = Some(location.to_string());
        self
    }
    pub fn with_metadata_map(
        &mut self,
        metadata: HashMap<String, Readonly<Metadata>>,
    ) -> &mut DefinitionBuilder {
        self.definition.metadata = metadata;
        self
    }

    pub fn with_metadata_linked_to_edge(
        &mut self,
        (metadata_key, metadata): (&str, Readonly<Metadata>),
        (edge_key, edge): (&str, Readonly<Edge>),
    ) -> &mut DefinitionBuilder {
        self.definition.edges.insert(edge_key.to_string(), edge);
        self.definition
            .metadata
            .insert(metadata_key.to_string(), metadata);
        self
    }
    pub fn with_edges(&mut self, edges: HashMap<String, Readonly<Edge>>) -> &mut DefinitionBuilder {
        self.definition.edges = edges;
        self
    }
    pub fn with_tags(&mut self, tags: HashMap<String, Readonly<RefTag>>) -> &mut DefinitionBuilder {
        self.definition.tags = tags;
        self
    }
    pub fn with_processors(
        &mut self,
        processors: HashMap<String, Readonly<Processor>>,
    ) -> &mut DefinitionBuilder {
        self.definition.processors = processors;
        self
    }
    pub fn with_processor(
        &mut self,
        (processor_key, processor): (&str, Readonly<Processor>),
    ) -> &mut DefinitionBuilder {
        self.definition
            .processors
            .insert(processor_key.to_string(), processor);
        self
    }
    pub fn with_arguments(
        &mut self,
        arguments: HashMap<String, Readonly<Vec<String>>>,
    ) -> &mut DefinitionBuilder {
        self.definition.arguments = arguments;
        self
    }
    pub fn with_environments(
        &mut self,
        environments: HashMap<String, Readonly<HashMap<String, String>>>,
    ) -> &mut DefinitionBuilder {
        self.definition.environments = environments;
        self
    }
    pub fn with_foreign_definitions(
        &mut self,
        foreign_definitions: HashMap<String, Readonly<ForeignDefinition>>,
    ) -> &mut DefinitionBuilder {
        self.definition.foreign_definitions = foreign_definitions;
        self
    }
    pub fn build(&mut self) -> Definition {
        let definition: Definition = self.definition.clone();
        self.definition = empty_definition();
        definition
    }
}

fn empty_definition() -> Definition {
    Definition {
        parent: None,
        name: "".to_string(),
        id: "".to_string(),
        location: None,
        metadata: HashMap::new(),
        edges: HashMap::new(),
        tags: HashMap::new(),
        processors: HashMap::new(),
        arguments: HashMap::new(),
        environments: HashMap::new(),
        foreign_definitions: HashMap::new(),
    }
}
