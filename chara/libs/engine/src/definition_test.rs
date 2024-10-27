#[cfg(test)]
mod definition {
    use std::{collections::HashMap, sync::Arc};

    use map_macro::hash_map;
    use serde_json::Map;
    use types::thread::readonly;

    use crate::{contexts_dto::{DefinitionContextDto, WritePermissionsDto}, definition::{Definition, Edge, Processor, Metadata}};


    #[test]
    fn context_should_group_metadata_and_edge_referencing_same_processor() {
        let reused_processor = readonly(Processor {
            arguments: vec![],
            environments: vec![],
            install: None,
            program: "test_program".to_string(),
        });
        let test_edge = readonly(Edge {
            definition: None,
            processor: Some(reused_processor.clone()),
            other: Map::new(),
        });
        let test_metadata = readonly(Metadata {
            edges: hash_map! {"test_edge".to_string() => test_edge.clone()},
            processor: Some(reused_processor.clone()),
            tags: HashMap::new(),
            other: Map::new(),
        });

        let definition = Definition {
            name: "Test".to_string(),
            metadata: hash_map! {
                "test_metadata".to_string() => test_metadata
            },
            edges: HashMap::new(),
            tags: HashMap::new(),
            processors: HashMap::new(),
            arguments: HashMap::new(),
            environments: HashMap::new(),
            foreign_definitions: HashMap::new()
        };
        let contexts = definition.processors_contexts();
        assert_eq!(contexts.len(), 1);
        let expected_definition = DefinitionContextDto {
            write: WritePermissionsDto::both(),
            edge: Some(("test_edge".to_string(), Map::new())),
            metadata: ("test_metadata".to_string(), Map::new()),
        };
        assert_eq!(contexts[0].definition, expected_definition);
        assert!(Arc::ptr_eq(&contexts[0].processor, &reused_processor));
    }
}
