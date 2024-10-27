use std::{
    sync::Arc,
    thread::{self},
};

use definition::{Definition, DefinitionInput, ProcessorContext};

pub mod cli;
mod contexts_dto;
pub mod definition;
mod definition_test;

pub trait Definitions: Send + Sync {
    fn get(&self, definition: &DefinitionInput) -> Option<Definition>;
    fn enrich(&self, context: &ProcessorContext) -> Option<Definition>;
}
pub fn run(chara: Definition, definitions: Arc<dyn Definitions>) {
    chara
        .foreign_definitions
        .iter()
        .map(|definition| {
            let definition = definition.1.clone();
            let definitions = definitions.clone();
            thread::spawn(move || {
                if let Ok(mut definition) = definition.write() {
                    definition.output = definitions.get(&definition.input);
                }
            })
        })
        .for_each(|handler| handler.join().unwrap());
    chara
        .processors_contexts()
        .into_iter()
        .map(|context| {
            let definitions = definitions.clone();
            thread::spawn(move || {
                definitions.enrich(&context);
            })
        })
        .for_each(|handler| handler.join().unwrap());
}
