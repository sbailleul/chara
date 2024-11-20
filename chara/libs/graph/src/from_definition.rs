use definitions::definition::{DefinitionDto, ReferenceOrObjectDto};

use crate::graph::{Edge, Graph, Node};
use std::collections::HashSet;

impl Graph {
    pub fn from_definition(definition: DefinitionDto) -> Self {
        let mut edges = HashSet::<Edge>::new();
        let mut nodes = HashSet::<Node>::new();

        for (name, metadata) in definition.metadata {
            nodes.insert(Node {
                id: name.clone(),
                data: metadata.other,
            });
            for edge in metadata.edges {
                if let ReferenceOrObjectDto::Object(edge) = edge {
                    if let Some(definition) = edge.definition {
                        let graph = Self::from_definition(definition);
                        nodes.extend(graph.nodes);
                        edges.extend(nodes.iter().map(|node| Edge {
                            id: format!("[{name}]-[{}]", &node.id),
                            source: name.clone(),
                            target: node.id.clone(),
                        }));
                    }
                }
            }
        }
        Self { edges, nodes }
    }
}
