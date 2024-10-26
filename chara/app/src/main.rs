use std::sync::Arc;

use engine::{run, DefinitionInput, Definitions};
use definitions::Definitions as DefinitionsImpl;


fn main() {
    let definitions = Arc::new(DefinitionsImpl{});
    let chara =definitions.get(&DefinitionInput::File("chara.json".to_string()));
    run(chara,definitions);
}
