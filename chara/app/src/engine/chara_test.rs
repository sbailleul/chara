#[cfg(test)]
mod chara {
    use std::{collections::HashMap, sync::Arc};

    use map_macro::hash_map;
    use serde_json::Map;

    use crate::{
        engine::{
            chara::{Chara, Edge, Enricher, Metadata},
            contexts_dto::{BootContextDto, WritePermissionsDto},
        },
        types::thread::readonly,
    };

    #[test]
    fn context_should_group_metadata_and_edge_referencing_same_enricher() {
        let reused_enricher = readonly(Enricher {
            arguments: vec![],
            environments: vec![],
            install: None,
            program: "test_program".to_string(),
        });
        let test_edge = readonly(Edge {
            definition: None,
            enricher: Some(reused_enricher.clone()),
            other: Map::new(),
        });
        let test_metadata = readonly(Metadata {
            edges: hash_map! {"test_edge".to_string() => test_edge.clone()},
            enricher: Some(reused_enricher.clone()),
            tags: HashMap::new(),
            other: Map::new(),
        });

        let chara = Chara {
            name: "Test".to_string(),
            metadata: hash_map! {
                "test_metadata".to_string() => test_metadata
            },
            edges: HashMap::new(),
            tags: HashMap::new(),
            enrichers: HashMap::new(),
            arguments: HashMap::new(),
            environments: HashMap::new(),
        };
        let contexts = chara.enrichers_contexts();
        assert_eq!(contexts.len(), 1);
        let expected_chara = BootContextDto {
            write: WritePermissionsDto::both(),
            edge: Some(("test_edge".to_string(), Map::new())),
            metadata: ("test_metadata".to_string(), Map::new()),
        };
        assert_eq!(contexts[0].chara, expected_chara);
        assert!(Arc::ptr_eq(&contexts[0].enricher, &reused_enricher));
    }
}
