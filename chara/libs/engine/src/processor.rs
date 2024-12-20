
use common::merge::Merge;
use serde_json::{Map, Value};

use crate::{
    definition::install::Install,
    draft::draft_definition::DraftDefinition,
};

#[derive(Debug)]
pub struct Enrichment {
    pub edge: Option<Map<String, Value>>,
    pub metadata: Option<Map<String, Value>>,
}
#[derive(Debug)]
pub struct ProcessorResult {
    pub enrichment: Option<Enrichment>,
    pub definition: Option<DraftDefinition>,
}

#[derive(Debug, Clone)]
pub struct Processor<TArguments, TEnvironment> {
    pub arguments: Vec<TArguments>,
    pub program: String,
    pub install: Option<Install<TArguments, TEnvironment>>,
    pub environments: Vec<TEnvironment>,
    pub current_directory: Option<String>,
}
impl<TArguments: Merge + Clone, TEnvironment: Merge + Clone> Merge
    for Processor<TArguments, TEnvironment>
{
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.program = other.program.clone();
        self.program = other.program.clone();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessorOverride<TArguments, TEnvironment, TProcessor> {
    pub arguments: Vec<TArguments>,
    pub environments: Vec<TEnvironment>,
    pub processor: TProcessor,
}


impl<TArguments: Merge + Clone, TEnvironment: Merge + Clone, TProcessor: Merge + Clone> Merge
    for ProcessorOverride<TArguments, TEnvironment, TProcessor>
{
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.environments.merge(&other.environments);
        self.processor.merge(&other.processor);
    }
}

impl<TArguments: Clone, TEnvironment: Clone, TProcessor: Clone>
    ProcessorOverride<TArguments, TEnvironment, TProcessor>
{
    pub fn processor(processor: &TProcessor) -> Self {
        Self {
            arguments: vec![],
            environments: vec![],
            processor: processor.clone(),
        }
    }
    pub fn from_with(&self, arguments: Vec<TArguments>, environments: Vec<TEnvironment>) -> Self {
        let mut processor = self.clone();
        processor.arguments = [arguments, processor.arguments].concat();
        processor.environments = [environments, processor.environments].concat();
        processor
    }
}
