use std::sync::Arc;

use engine::{definition::DefinitionInput, run, Definitions};
use definitions::Definitions as DefinitionsImpl;


#[tokio::main]
async fn main() {
    let definitions = Arc::new(DefinitionsImpl{});
    let chara =definitions.get(&DefinitionInput::File("chara.json".to_string()));
    if let Some(chara) = chara{
        run(chara,definitions);
    }
}
