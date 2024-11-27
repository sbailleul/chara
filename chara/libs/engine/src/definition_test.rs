#[cfg(test)]
mod definition {
    use std::{collections::HashMap, sync::Arc, vec};

    use common::thread::readonly;
    use map_macro::hash_map;
    use serde_json::{Map, Value};

    use crate::{
        contexts::{ContextDto, DefinitionContextDto, WritePermissionsDto},
        definition::{
            definition::{Definition, Metadata},
            edge::{Edge, EdgeOverride},
        },
        processor::{Processor, ProcessorOverride},
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
            processor: Some(ProcessorOverride::processor(
                &reused_processor,
                &"reference".to_string(),
            )),
            other: Map::<String, Value>::new(),
        });
        let test_metadata = readonly(Metadata {
            edges: hash_map! {"test_edge".to_string() =>EdgeOverride{edge: test_edge.clone(), arguments: vec![], environments: vec![], definition: None, other: Map::<String, Value>::new()} },
            processor: Some(ProcessorOverride::processor(
                &reused_processor,
                &"reference".to_string(),
            )),
            tags: HashMap::new(),
            other: Map::<String, Value>::new(),
        });

        let definition = Definition {
            parent: None,
            id: "123".to_string(),
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
            processor_reference: "reference".to_string(),
            edge: Some(ContextDto::new(
                "test_edge".to_string(),
                Map::<String, Value>::new(),
            )),
            metadata: ContextDto::new("test_metadata".to_string(), Map::<String, Value>::new()),
        };
        assert_eq!(contexts[0].definition, expected_definition);
        assert!(Arc::ptr_eq(
            &contexts[0].processor.processor,
            &reused_processor
        ));
    }
}
