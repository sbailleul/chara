#[cfg(test)]
mod definition {
    use std::{collections::HashMap, sync::Arc, vec};

    use map_macro::hash_map;
    use serde_json::Value;
    use types::thread::readonly;

    use crate::{
        contexts::{ContextDto, DefinitionContextDto, WritePermissionsDto},
        definition::{Definition, Edge, EdgeOverride, Metadata, Processor, ProcessorOverride},
    };

    #[test]
    fn context_should_group_metadata_and_edge_referencing_same_processor() {
        let reused_processor = readonly(Processor {
            arguments: vec![],
            environments: vec![],
            install: None,
            program: "test_program".to_string(),
            current_directory: None,
        });
        let test_edge = readonly(Edge {
            definition: None,
            processor: Some(ProcessorOverride::processor(&reused_processor)),
            other: serde_json::Value::Null,
        });
        let test_metadata = readonly(Metadata {
            edges: hash_map! {"test_edge".to_string() =>EdgeOverride{edge: test_edge.clone(), arguments: vec![], environments: vec![]} },
            processor: Some(ProcessorOverride::processor(&reused_processor)),
            tags: HashMap::new(),
            other: serde_json::Value::Null,
        });

        let definition = Definition {
            location: None,
            name: "Test".to_string(),
            metadata: hash_map! {
                "test_metadata".to_string() => test_metadata
            },
            edges: HashMap::new(),
            tags: HashMap::new(),
            processors: HashMap::new(),
            arguments: HashMap::new(),
            environments: HashMap::new(),
            foreign_definitions: HashMap::new(),
        };
        let contexts = definition.processors_contexts();
        assert_eq!(contexts.len(), 1);
        let expected_definition = DefinitionContextDto {
            location: None,
            write: WritePermissionsDto::both(),
            edge: Some(ContextDto::new("test_edge".to_string(), Value::Null)),
            metadata: ContextDto::new("test_metadata".to_string(), Value::Null),
        };
        assert_eq!(contexts[0].definition, expected_definition);
        assert!(Arc::ptr_eq(
            &contexts[0].processor.processor,
            &reused_processor
        ));
    }
}
