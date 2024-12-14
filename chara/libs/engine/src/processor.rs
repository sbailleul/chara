use std::sync::Arc;

use common::{merge::Merge, thread::Readonly};
use serde_json::{Map, Value};

use crate::{
    cli::{Arguments, Environment}, definition::install::CleanInstall, draft::draft_definition::DraftDefinition, reference_value::ReferencedValue
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
pub struct Processor<TArguments, TInstall, TEnvironment> {
    pub arguments: Vec<TArguments>,
    pub program: String,
    pub install: Option<TInstall>,
    pub environments: Vec<TEnvironment>,
    pub current_directory: Option<String>,
}
pub type CleanProcessor = Processor<Arguments, CleanInstall, Environment>;
impl Merge for CleanProcessor {
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.program = other.program.clone();
        self.program = other.program.clone();
    }
}

#[derive(Debug, Clone)]
pub struct ProcessorOverride<TArguments, TEnvironment, TProcessor> {
    pub arguments: Vec<TArguments>,
    pub environments: Vec<TEnvironment>,
    pub processor: TProcessor,
}
type ProcessorOverrideWithRef<TArguments, TEnvironment, TProcessor> =
    ReferencedValue<ProcessorOverride<TArguments, TEnvironment, TProcessor>>;

pub type CleanProcessorOverride =
    ProcessorOverrideWithRef<Arguments, Environment, Readonly<CleanProcessor>>;

impl<TArguments: Clone, TEnvironment: Clone, TProcessor: Clone>
    ProcessorOverrideWithRef<TArguments, TEnvironment, TProcessor>
{
    pub fn processor(processor: &TProcessor, reference: &String) -> Self {
        Self {
            r#ref: reference.clone(),
            value: ProcessorOverride {
                arguments: vec![],
                environments: vec![],
                processor: processor.clone(),
            },
        }
    }
    pub fn from_with(&self, arguments: Vec<TArguments>, environments: Vec<TEnvironment>) -> Self {
        let mut processor = self.clone();
        processor.value.arguments = [arguments, processor.value.arguments].concat();
        processor.value.environments = [environments, processor.value.environments].concat();
        processor
    }
}
impl Merge for CleanProcessorOverride {
    fn merge(&mut self, other: &Self) {
        self.value
            .arguments
            .append(&mut other.value.arguments.clone());
        self.value
            .environments
            .append(&mut other.value.environments.clone());
        self.r#ref = other.r#ref.clone();
        // self.processor.read().unwrap().
    }
}
impl PartialEq for CleanProcessorOverride {
    fn eq(&self, other: &Self) -> bool {
        self.value.arguments == other.value.arguments
            && self.value.environments == other.value.environments
            && Arc::ptr_eq(&self.value.processor, &other.value.processor)
    }
}
impl Eq for CleanProcessorOverride {}
