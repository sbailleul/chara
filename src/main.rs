mod engine;
mod parser;
mod types;
use engine::run;
use types::input::Definition;

fn main() {
    let bootes = parser::parse(&Definition::File("bootes.json".to_string()));
    run(bootes);
}
