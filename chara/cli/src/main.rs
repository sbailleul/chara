use std::sync::Arc;

use definitions::definitions::Definitions as DefinitionsImpl;
use engine::{definition::DefinitionInput, run, Definitions};
use graph::process;
fn main() {
    colog::init();

    process(&DefinitionInput::File(
        "chara_results/536d8481-8203-46df-b008-554ce25329ff.json".to_string(),
    )).unwrap();

    // let definitions = Arc::new(DefinitionsImpl {});
    // let definition = definitions.get(&DefinitionInput::File(
    //     "../../examples/chara.json".to_string(),
    // ));
    // if let Ok(definition) = definition {
    //     run(definition, definitions);
    // }
}
