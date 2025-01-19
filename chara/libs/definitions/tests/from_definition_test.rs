
use definitions::definitions::Definitions;
use engine::definition::definition::Definition;

#[test]
fn should_map_foreign_definitions() {
    let result = Definitions::get_from_path("./tests/definitions/chara.json".to_string()).unwrap();
    dbg!(&result.foreign_definitions);
    assert_eq!(result.foreign_definitions.len(), 1)
}

