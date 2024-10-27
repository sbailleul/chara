use std::sync::Arc;

use definitions::Definitions as DefinitionsImpl;
use engine::{definition::DefinitionInput, run, Definitions};

fn main() {
    let definitions = Arc::new(DefinitionsImpl {});
    let chara = definitions.get(&DefinitionInput::File("chara.json".to_string()));
    if let Some(chara) = chara {
        run(chara, definitions);
    }
}
