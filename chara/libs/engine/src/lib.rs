use std::{
    sync::Arc,
    thread::{self},
};

use contexts::ProcessorContext;
use definition::{Definition, DefinitionInput};
use errors::DefinitionError;
use log::{error, info};
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
                    .map_err(|_| DefinitionError::Thread(ThreadError::Poison))
                    .and_then(|mut definition| {
                        definitions
                            .get(&definition.input)
                            .map(|found_definition| definition.output = Some(found_definition))
                    })
            })
        })
        .for_each(|handler| {
            if let Ok(Err(err)) = handler.join() {
                error!("{err}");
            }
        });
    let contexts = definition.processors_contexts();
    enrich_multi_thread(contexts, definitions);
}

fn enrich_multi_thread(contexts: Vec<ProcessorContext>, definitions: Arc<dyn Definitions>) {
    contexts
        .into_iter()
        .map(|context| {
            let definitions = definitions.clone();
            thread::spawn(move || definitions.enrich(&context))
        })
        .for_each(|handler| {
            let result = handler.join();
            if let Ok(result) = result {
                match result {
                    Ok(definition) => info!("{:?}", definition),
                    Err(err) => error!("{err}"),
                }
            }
        });
}
