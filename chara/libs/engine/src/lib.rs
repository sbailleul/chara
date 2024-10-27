use std::{
    ffi::OsStr, sync::Arc, thread::{self}
};

use contexts_dto::DefinitionContextDto;
use definition::{Definition, DefinitionInput, EnricherContext, ForeignDefinition};

pub mod definition;
mod definition_test;
pub mod cli;
mod contexts_dto;


pub trait Definitions: Send + Sync{
     fn get(&self, definition: &DefinitionInput) -> Option<Definition>;
     fn enrich(&self, context: &EnricherContext) -> Option<Definition>;
}
pub fn run(chara: Definition, definitions:  Arc< dyn Definitions>) {
    chara
        .enrichers_contexts()
        .into_iter()
        .map(|context| {
            let definitions =definitions.clone();
            thread::spawn(move || {
                definitions.enrich(&context);
            })
        })
        .for_each(|handler| handler.join().unwrap());
}

