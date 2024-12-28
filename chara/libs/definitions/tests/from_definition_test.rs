use std::path::{self, Path};

use definitions::definitions::Definitions;

#[test]
fn should_map_foreign_definitions() {
    let result = Definitions::get("./tests/definitions/chara.json".to_string()).unwrap();
    dbg!(&result.foreign_definitions);
    assert_eq!(result.foreign_definitions.len(), 2)
}
