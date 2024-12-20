use common::{merge::Merge, thread::Readonly};
use serde_json::{Map, Value};

use crate::{
    cli::{DraftArguments, DraftEnvironments}, definition::install::Install, definition::definition::Definition, reference_value::{LazyRef, ReferencedValue}
};

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
    pub arguments: Vec<DraftArguments>,
    pub program: String,
    pub install: Option<Install>,
    pub environments: Vec<DraftEnvironments>,
    pub current_directory: Option<String>,
}
impl Merge for Processor {
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

pub type DraftProcessorOverride =
    ProcessorOverride<DraftArguments, DraftEnvironments, Option<LazyRef<Processor>>>;
pub type DefinedProcessorOverride =
    ProcessorOverride<DraftArguments, DraftEnvironments, ReferencedValue<Readonly<Processor>>>;
