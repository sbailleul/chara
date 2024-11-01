use std::{collections::HashMap, fs::canonicalize, process::Command};

use engine::{
    cli::{Argument, Environment}, definition::{Install, Processor, ProcessorOverride}, CliError, DefinitionError
};
use types::ThreadError;


pub trait Inputs {
    fn arguments(&self) -> Vec<Argument>;
    fn environments(&self) -> Vec<Environment>;
    fn flatten_arguments(&self) -> Vec<String> {
        self.arguments()
            .iter()
            .map(|arg| arg.unwrap())
            .flatten()
            .collect()
    }
    fn flatten_environments(&self) -> HashMap<String, String> {
        self.environments()
            .iter()
            .map(|environment| environment.unwrap())
            .flatten()
            .collect()
    }
}

pub trait Cli: Inputs {
    fn program(&self) -> Result<String, ThreadError>;
    fn current_directory(&self) -> Result<Option<String>, ThreadError>;
    fn command(&self) -> Result<Command, CliError> {
        self.program()
            .map_err(|err| CliError::Thread(err))
            .and_then(|program| {
                let mut cmd = Command::new(program);
                if let Some(current_directory) = self
                    .current_directory()
                    .map_err(|err| CliError::Thread(err))?.as_ref()
                {
                    let current_directory = canonicalize(current_directory)
                        .map_err(|_| CliError::PathNotFound(current_directory.clone()))?;
                    cmd.current_dir(current_directory);
                }

                cmd.args(self.flatten_arguments())
                    .envs(self.flatten_environments());
                Ok(cmd)
            })
    }
    fn output_stdout(&self) -> Result<String, DefinitionError> {
        self.command()
            .map_err(|err| DefinitionError::Cli(err))
            .and_then(|mut cmd| {
                cmd.output()
                    .map_err(|err| DefinitionError::Process(format!("{err}")))
                    .and_then(|output| {
                        if output.stderr.len() > 0 {
                            String::from_utf8(output.stdout)
                                .map_err(|err| {
                                    dbg!(&err);
                                    DefinitionError::Parse(format!(
                                        "Cannot convert stderr to string [Error : {err}]"
                                    ))
                                })
                                .and_then(|stderr| {
                                    Err(DefinitionError::Process(format!(
                                        "Processor execution failed [Error : {stderr}]"
                                    )))
                                })
                        } else {
                            String::from_utf8(output.stdout).map_err(|err| {
                                DefinitionError::Parse(format!(
                                    "Cannot convert stdout to string [Error : {err}]"
                                ))
                            })
                        }
                    })
            })
    }
}

impl Inputs for Install {
    fn arguments(&self) -> Vec<Argument> {
        self.arguments.clone()
    }
    fn environments(&self) -> Vec<Environment> {
        self.environments.clone()
    }
}
impl Cli for Install {
    fn program(&self) -> Result<String, ThreadError> {
        Ok(self.program.clone())
    }
    fn current_directory(&self) -> Result<Option<String>, ThreadError> {
        Ok(self.current_directory.clone())
    }
}

impl Inputs for Processor {
    fn arguments(&self) -> Vec<Argument> {
        self.arguments.clone()
    }
    fn environments(&self) -> Vec<Environment> {
        self.environments.clone()
    }
}
impl Cli for Processor {
    fn program(&self) -> Result<String, ThreadError> {
        Ok(self.program.clone())
    }

    fn current_directory(&self) -> Result<Option<String>, ThreadError> {
        Ok(self.current_directory.clone())
    }
}

impl Inputs for ProcessorOverride {
    fn arguments(&self) -> Vec<Argument> {
        self.arguments
            .clone()
            .into_iter()
            .chain(
                self.processor
                    .read()
                    .map_or(vec![], |processor| processor.arguments())
                    .into_iter(),
            )
            .collect()
    }

    fn environments(&self) -> Vec<Environment> {
        self.environments
            .clone()
            .into_iter()
            .chain(
                self.processor
                    .read()
                    .map_or(vec![], |processor| processor.environments())
                    .into_iter(),
            )
            .collect()
    }
}
impl Cli for ProcessorOverride {
    fn program(&self) -> Result<String, ThreadError> {
        self.processor
            .read()
            .or(Err(ThreadError::Poison))
            .and_then(|processor| processor.program())
    }

    fn current_directory(&self) -> Result<Option<String>, ThreadError> {
        self.processor
            .read()
            .map(|processor| processor.current_directory.clone())
            .or(Err(ThreadError::Poison))
    }
}
