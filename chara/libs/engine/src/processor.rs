use std::sync::Arc;

use common::{merge::Merge, thread::Readonly};
use serde_json::{Map, Value};

use crate::{
    clean::clean_definition::{CleanInstall, CleanProcessor},
    cli::{Arguments, Environment},
    draft::draft_definition::DraftDefinition,
    reference_value::ReferencedValue,
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
impl<TArguments: Merge + Clone, TInstall: Merge + Clone, TEnvironment: Merge + Clone> Merge
    for Processor<TArguments, TInstall, TEnvironment>
{
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
impl<TArguments: Merge + Clone, TEnvironment: Merge + Clone, TProcessor: Merge + Clone> Merge
    for ProcessorOverride<TArguments, TEnvironment, TProcessor>
{
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.environments.merge(&other.environments);
        self.processor.merge(&other.processor);
    }
}

pub type ProcessorOverrideWithRef<TArguments, TEnvironment, TProcessor> =
    ReferencedValue<ProcessorOverride<TArguments, TEnvironment, TProcessor>>;

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
