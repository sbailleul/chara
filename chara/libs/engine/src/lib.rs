use std::{
    sync::Arc,
    thread::{self},
};

use definition::{Definition, DefinitionInput, ProcessorContext};
use types::ThreadError;
pub mod cli;
mod contexts_dto;
pub mod definition;
mod definition_test;

#[derive(Debug)]
pub enum DefinitionError {
    Parse(String),
    Access(String),
    Process(String),
    Thread(ThreadError),
    Cli(CliError),
}
#[derive(Debug)]
pub enum CliError {
    Thread(ThreadError),
    PathNotFound(String),
}
#[derive(Debug)]
enum Error {
    Thread(ThreadError),
    Process(DefinitionError),
}
pub trait Definitions: Send + Sync {
    fn get(&self, definition: &DefinitionInput) -> Result<Definition, DefinitionError>;
    fn enrich(&self, context: &ProcessorContext) -> Result<Definition, DefinitionError>;
}

pub fn run(definition: Definition, definitions: Arc<dyn Definitions>) {
    definition
        .foreign_definitions
        .iter()
        .map(|definition| {
            let definition = definition.1.clone();
            let definitions = definitions.clone();
            thread::spawn(move || {
                definition
                    .write()
                    .map_err(|_| Error::Thread(ThreadError::Poison))
                    .and_then(|mut definition| {
                        definitions
                            .get(&definition.input)
                            .map(|found_definition| definition.output = Some(found_definition))
                            .or_else(|err| Err(Error::Process(err)))
                    })
            })
        })
        .for_each(|handler| {
            if let Ok(Err(err)) = handler.join() {
                dbg!(&err);
            }
        });
    definition
        .processors_contexts()
        .into_iter()
        .map(|context| {
            let definitions = definitions.clone();
            thread::spawn(move || {
                let _ = definitions.enrich(&context);
            })
        })
        .for_each(|handler| handler.join().unwrap());
}
