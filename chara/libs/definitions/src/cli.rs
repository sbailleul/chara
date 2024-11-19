use std::{collections::HashMap, fs::canonicalize, process::Command};

use engine::{
    cli::{Argument, Environment},
    definition::Install,
    errors::DefinitionError, processor::{Processor, ProcessorOverride},
};
use log::info;
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
    fn program(&self) -> Result<String, DefinitionError>;
    fn current_directory(&self) -> Result<Option<String>, DefinitionError>;
    fn command(
        &self,
        additional_arguments: Option<Vec<String>>,
    ) -> Result<Command, DefinitionError> {
        self.program().and_then(|program| {
            let mut cmd = Command::new(&program);
            info!("Run program {program}");
            if let Some(current_directory) = self.current_directory()?.as_ref() {
                info!("Current directory {current_directory}");
                let current_directory =
                    canonicalize(current_directory).map_err(DefinitionError::IO)?;
                cmd.current_dir(current_directory);
            } else {
                info!("No current directory")
            }
            let additional_arguments = additional_arguments.unwrap_or(vec![]);
            cmd.args(&additional_arguments);
            let arguments = [self.flatten_arguments(), additional_arguments].concat();
            let environments = self.flatten_environments();
            info!("Arguments {}", arguments.join(" ").escape_default());
            info!(
                "Environments {}",
                environments
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
            cmd.args(self.flatten_arguments())
                .envs(self.flatten_environments());
            Ok(cmd)
        })
    }
    fn output_stdout(
        &self,
        additional_arguments: Option<Vec<String>>,
    ) -> Result<String, DefinitionError> {
        self.command(additional_arguments).and_then(|mut cmd| {
            cmd.output()
                .map_err(DefinitionError::IO)
                .and_then(|output| {
                    if output.status.success() {
                        String::from_utf8(output.stdout)
                            .map_err(DefinitionError::ParseUtf8)
                            .inspect(|stdout| {
                                info!("Stdout {stdout}");
                            })
                    } else {
                        String::from_utf8(output.stderr)
                            .map_err(DefinitionError::ParseUtf8)
                            .and_then(|stderr| {
                                Err(DefinitionError::Process(format!(
                                    "Processor execution failed [Error : {stderr}]"
                                )))
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
    fn program(&self) -> Result<String, DefinitionError> {
        Ok(self.program.clone())
    }
    fn current_directory(&self) -> Result<Option<String>, DefinitionError> {
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
    fn program(&self) -> Result<String, DefinitionError> {
        Ok(self.program.clone())
    }

    fn current_directory(&self) -> Result<Option<String>, DefinitionError> {
        Ok(self.current_directory.clone())
    }
}

impl Inputs for ProcessorOverride {
    fn arguments(&self) -> Vec<Argument> {
        self.processor
            .read()
            .map_or(vec![], |processor| processor.arguments())
            .into_iter()
            .chain(self.arguments.clone().into_iter())
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
    fn program(&self) -> Result<String, DefinitionError> {
        self.processor
            .read()
            .or(Err(DefinitionError::Thread(ThreadError::Poison)))
            .and_then(|processor| processor.program())
    }

    fn current_directory(&self) -> Result<Option<String>, DefinitionError> {
        self.processor
            .read()
            .map(|processor| processor.current_directory.clone())
            .or(Err(DefinitionError::Thread(ThreadError::Poison)))
    }
}
