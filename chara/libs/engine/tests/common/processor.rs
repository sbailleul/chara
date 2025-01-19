use std::vec;

use engine::{
    cli::{DraftArguments, DraftEnvironments},
    definition::install::Install,
    processor::Processor,
};

pub struct ProcessorBuilder {
    processor: Processor,
}

impl ProcessorBuilder {
    pub fn new() -> Self {
        Self {
            processor: empty_processor(),
        }
    }
    pub fn with_arguments(&mut self, arguments: Vec<DraftArguments>) -> &mut Self {
        self.processor.arguments = arguments;
        self
    }
    pub fn with_program(&mut self, program: &str) -> &mut Self {
        self.processor.program = program.to_string();
        self
    }
    pub fn with_install(&mut self, install: Install) -> &mut Self {
        self.processor.install = Some(install);
        self
    }
    pub fn with_environments(&mut self, environments: Vec<DraftEnvironments>) -> &mut Self {
        self.processor.environments = environments;
        self
    }
    pub fn with_current_directory(&mut self, current_directory: &str) -> &mut Self {
        self.processor.current_directory = Some(current_directory.to_string());
        self
    }
    pub fn build(&mut self) -> Processor {
        let processor = self.processor.clone();
        self.processor = empty_processor();
        processor
    }
}

fn empty_processor() -> Processor {
    Processor {
        arguments: vec![],
        current_directory: None,
        environments: vec![],
        install: None,
        program: "".to_string(),
    }
}
