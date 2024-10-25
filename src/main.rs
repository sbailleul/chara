mod engine;
mod parser;
mod types;
use engine::run;
use types::input::Definition;

fn main() {
    let chara = parser::parse(&Definition::File("chara.json".to_string()));
    run(chara);
}
