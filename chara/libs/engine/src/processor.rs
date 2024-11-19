use std::sync::Arc;

use serde_json::{Map, Value};
use types::thread::Readonly;

use crate::{cli::{Argument, Environment}, definition::{Definition, Install}};

#[derive(Debug)]
pub struct Enrichment {
    pub edge: Option<Map<String, Value>>,
    pub metadata: Option<Map<String, Value>>,
}
#[derive(Debug)]
pub struct ProcessorResult {
    pub enrichment: Option<Enrichment>,
    pub definition: Option<Definition>,
}

#[derive(Debug)]
pub struct Processor {
    pub arguments: Vec<Argument>,
    pub program: String,
    pub install: Option<Install>,
    pub environments: Vec<Environment>,
    pub current_directory: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessorOverride {
    pub arguments: Vec<Argument>,
    pub environments: Vec<Environment>,
    pub processor: Readonly<Processor>,
    pub reference: String,
}
impl ProcessorOverride {
    pub fn processor(processor: &Readonly<Processor>, reference: &String) -> Self {
        Self {
            arguments: vec![],
            environments: vec![],
            reference: reference.clone(),
            processor: processor.clone(),
        }
    }

    pub fn with(&self, arguments: Vec<Argument>, environments: Vec<Environment>) -> Self {
        let mut processor = self.clone();
        processor.arguments = [arguments, processor.arguments].concat();
        processor.environments = [environments, processor.environments].concat();
        processor
    }
}
impl PartialEq for ProcessorOverride {
    fn eq(&self, other: &Self) -> bool {
        self.arguments == other.arguments
            && self.environments == other.environments
            && Arc::ptr_eq(&self.processor, &other.processor)
    }
}
impl Eq for ProcessorOverride {}