use std::{
    sync::Arc,
    thread::{self},
};

use definition::{Definition, DefinitionInput, ProcessorContext};
use log::{error, info};
use thiserror::Error;
use types::ThreadError;
pub mod cli;
mod contexts_dto;
pub mod definition;
mod definition_test;

#[derive(Debug, Error)]
pub enum DefinitionError {
    #[error("Parse error {0}")]
    Parse(String),
    #[error("Access error {0}")]
    Access(String),
    #[error("Process error {0}")]
    Process(String),
    #[error("Thread error {0}")]
    Thread(ThreadError),
    #[error("Cli error {0}")]
    Cli(CliError),
}
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Thread error {0}")]
    Thread(ThreadError),
    #[error("Path not found {0}")]
    PathNotFound(String),
}
#[derive(Debug, Error)]
enum Error {
    #[error("Thread error {0}")]
    Thread(ThreadError),
    #[error("Process error {0}")]
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
                info!("COUCOU");
                error!("{err}");
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
