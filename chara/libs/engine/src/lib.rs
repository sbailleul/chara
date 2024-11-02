use std::{
    sync::Arc,
    thread::{self},
};

use contexts::ProcessorContext;
use definition::{Definition, DefinitionInput};
use errors::{DefinitionError, Error};
use log::error;
use types::ThreadError;
pub mod cli;
pub mod contexts;
pub mod definition;
mod definition_test;
pub mod errors;


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
                error!("{err}");
            }
        });
    let contexts = definition
        .processors_contexts();
    dbg!(&contexts);
    contexts
        .into_iter()
        .map(|context| {
            let definitions = definitions.clone();
            thread::spawn(move || {
                let _ = definitions.enrich(&context);
            })
        })
        .for_each(|handler| handler.join().unwrap());
}
