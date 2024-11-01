use std::sync::Arc;

use definitions::Definitions as DefinitionsImpl;
use engine::{definition::DefinitionInput, run, Definitions};

fn main() {
    // let mut output = Command::new("cargo").current_dir("../../scrappers/http").args(vec!["run"]).output();
    // dbg!(&output);
    colog::init();
    let definitions = Arc::new(DefinitionsImpl {});
    let chara = definitions.get(&DefinitionInput::File("chara.json".to_string()));
    if let Ok(chara) = chara {
        run(chara, definitions);
    }
}
