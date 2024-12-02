use std::sync::Arc;

use serde_json::{Map, Value};
use common::{merge::Merge, thread::Readonly};

use crate::{cli::{Arguments, Environment}, definition::definition::{Definition, Install}};

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

#[derive(Debug, Clone)]
pub struct Processor {
    pub arguments: Vec<Arguments>,
    pub program: String,
    pub install: Option<Install>,
    pub environments: Vec<Environment>,
    pub current_directory: Option<String>,
}
impl Merge for Processor{
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.program = other.program.clone();
        self.program = other.program.clone();
    }
}

#[derive(Debug, Clone)]
pub struct ProcessorOverride {
    pub arguments: Vec<Arguments>,
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

    pub fn from_with(&self, arguments: Vec<Arguments>, environments: Vec<Environment>) -> Self {
        let mut processor = self.clone();
        processor.arguments = [arguments, processor.arguments].concat();
        processor.environments = [environments, processor.environments].concat();
        processor
    }
    
}
impl Merge for ProcessorOverride{
    fn merge(&mut self, other: &Self) {
        self.arguments.append(&mut other.arguments.clone());
        self.environments.append(&mut other.environments.clone());
        self.reference = other.reference.clone();
        // self.processor.read().unwrap().
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