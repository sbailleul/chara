use ::common::thread::readonly;
use common::{
    definition::DefinitionBuilder, edge::EdgeBuilder, edge_override::EdgeOverrideBuilder,
    metadata::MetadataBuilder, processor::ProcessorBuilder,
};

mod common;
#[test]
pub fn should_not_find_parent_on_empty_path() {
    let definition = DefinitionBuilder::new()
        .build()
        .find_definition_by_path(&"".to_string());
    assert!(definition.is_none());
}

#[test]
pub fn should_find_root_definition() {
    let parent = DefinitionBuilder::new().with_name("parent").build();
    let definition = DefinitionBuilder::new()
        .with_parent(parent.clone())
        .build()
        .find_definition_by_path(&"#/".to_string())
        .unwrap();

    assert_eq!(parent.name, definition.1.name);
}

#[test]
pub fn should_find_relative_definition() {
    let edge = readonly(EdgeBuilder::new().build());
    let edge_override = EdgeOverrideBuilder::new()
        .with_definition(
            DefinitionBuilder::new()
                .with_name("relative_definition")
                .build(),
        )
        .with_referenced_edge("referenced_edge", edge.clone())
        .build();
    let metadata = MetadataBuilder::new()
        .with_edge(("referenced_edge", edge_override))
        .build();
    let parent: engine::definition::definition::Definition = DefinitionBuilder::new()
        .with_name("parent")
        .with_metadata_linked_to_edge(
            ("relative_metadata", readonly(metadata)),
            ("relative_edge", edge.clone()),
        )
        .build();
    let relative_definition = DefinitionBuilder::new()
        .with_parent(parent.clone())
        .build()
        .find_definition_by_path(&"../relative_metadata/referenced_edge".to_string())
        .unwrap();

    assert_eq!("relative_definition", relative_definition.1.name);
}

#[test]
pub fn should_find_parent_definition() {
    let parent: engine::definition::definition::Definition =
        DefinitionBuilder::new().with_name("parent").build();
    let parent_definition = DefinitionBuilder::new()
        .with_parent(parent.clone())
        .build()
        .find_definition_by_path(&"..".to_string())
        .unwrap();

    assert_eq!("parent", parent_definition.1.name);
}

#[test]
pub fn should_find_inner_definition() {
    let inner_edge = readonly(EdgeBuilder::new().build());
    let inner_edge_override = EdgeOverrideBuilder::new()
        .with_definition(
            DefinitionBuilder::new()
                .with_name("inner_definition")
                .build(),
        )
        .with_referenced_edge("referenced_edge", inner_edge.clone())
        .build();
    let metadata = MetadataBuilder::new()
        .with_edge(("referenced_edge", inner_edge_override))
        .build();

    let relative_definition = DefinitionBuilder::new()
        .with_metadata_linked_to_edge(
            ("inner_metadata", readonly(metadata)),
            ("referenced_edge", inner_edge),
        )
        .build()
        .find_definition_by_path(&"./inner_metadata/referenced_edge".to_string())
        .unwrap();

    assert_eq!("inner_definition", relative_definition.1.name);
}

#[test]
pub fn should_find_parent_processor() {
    let parent: engine::definition::definition::Definition = DefinitionBuilder::new()
        .with_name("parent")
        .with_processor((
            "parent_processor",
            readonly(ProcessorBuilder::new().with_program("test_program").build()),
        ))
        .build();
    let processor = DefinitionBuilder::new()
        .with_parent(parent.clone())
        .build()
        .find_processor(&"../parent_processor".to_string())
        .unwrap();

    assert_eq!("test_program", processor.read().unwrap().program);
}
