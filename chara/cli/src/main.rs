use std::sync::Arc;

use definitions::definitions::Definitions as DefinitionsImpl;
use engine::Definitions;
use engine::{definition::input::DefinitionInput, run};
use graph::create_graph;
fn main() {
    colog::init();

    let definitions: Arc<dyn Definitions> = Arc::new(DefinitionsImpl {});

    let definition = DefinitionsImpl::get(
        "/home/sbailleul/code/chara/examples/chara.json".to_string(),
        // "/home/sbailleul/code/chara/chara/cli/chara_results/test.json".to_string(),
    );
    if let Ok(definition) = definition {
        run(definition, definitions);
    }
}

fn graph() {
    create_graph(&DefinitionInput::File(
        "chara_results/536d8481-8203-46df-b008-554ce25329ff.json".to_string(),
    ))
    .unwrap();
}
