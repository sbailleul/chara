use std::{
    ffi::OsStr, sync::Arc, thread::{self}
};

use contexts_dto::DefinitionContextDto;
use definition::{Definition, EnricherContext};

pub mod definition;
mod definition_test;
pub mod cli;
mod contexts_dto;
#[derive(Debug)]
pub enum DefinitionInput {
    File(String),
    Text(String),
}

pub trait Definitions: Send + Sync{
     fn get(&self, definition: &DefinitionInput) -> Definition;
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

