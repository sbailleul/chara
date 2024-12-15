use std::{collections::HashMap, fs::canonicalize, process::Command};

use common::ThreadError;
use engine::{
    clean::clean_definition::{CleanInstall, CleanProcessor, CleanProcessorOverride},
    cli::{Arguments, Environment},
    errors::CharaError,
};
use log::info;
pub trait Inputs {
    fn arguments(&self) -> Vec<Arguments>;
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
    fn program(&self) -> Result<String, CharaError>;
    fn current_directory(&self) -> Result<Option<String>, CharaError>;
    fn command(&self, additional_arguments: Option<Vec<String>>) -> Result<Command, CharaError> {
        self.program().and_then(|program| {
            let mut cmd = Command::new(&program);
            info!("Run program {program}");
            if let Some(current_directory) = self.current_directory()?.as_ref() {
                info!("Current directory {current_directory}");
                let current_directory = canonicalize(current_directory).map_err(CharaError::IO)?;
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
    ) -> Result<String, CharaError> {
        self.command(additional_arguments).and_then(|mut cmd| {
            info!("Run command");
            cmd.output().map_err(CharaError::IO).and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout)
                        .map_err(CharaError::ParseUtf8)
                        .inspect(|stdout| {
                            info!("Stdout {stdout}");
                        })
                } else {
                    String::from_utf8(output.stderr)
                        .map_err(CharaError::ParseUtf8)
                        .and_then(|stderr| {
                            Err(CharaError::Process(format!(
                                "Processor execution failed [Error : {stderr}]"
                            )))
                        })
                }
            })
        })
    }
}

impl Inputs for CleanInstall {
    fn arguments(&self) -> Vec<Arguments> {
        self.arguments.clone()
    }
    fn environments(&self) -> Vec<Environment> {
        self.environments.clone()
    }
}
impl Cli for CleanInstall {
    fn program(&self) -> Result<String, CharaError> {
        Ok(self.program.clone())
    }
    fn current_directory(&self) -> Result<Option<String>, CharaError> {
        Ok(self.current_directory.clone())
    }
}

impl Inputs for CleanProcessor {
    fn arguments(&self) -> Vec<Arguments> {
        self.arguments.clone()
    }
    fn environments(&self) -> Vec<Environment> {
        self.environments.clone()
    }
}
impl Cli for CleanProcessor {
    fn program(&self) -> Result<String, CharaError> {
        Ok(self.program.clone())
    }

    fn current_directory(&self) -> Result<Option<String>, CharaError> {
        Ok(self.current_directory.clone())
    }
}

impl Inputs for CleanProcessorOverride {
    fn arguments(&self) -> Vec<Arguments> {
        self.value
            .processor
            .read()
            .map_or(vec![], |processor| processor.arguments())
            .into_iter()
            .chain(self.value.arguments.clone().into_iter())
            .collect()
    }

    fn environments(&self) -> Vec<Environment> {
        self.value
            .environments
            .clone()
            .into_iter()
            .chain(
                self.value
                    .processor
                    .read()
                    .map_or(vec![], |processor| processor.environments())
                    .into_iter(),
            )
            .collect()
    }
}
impl Cli for CleanProcessorOverride {
    fn program(&self) -> Result<String, CharaError> {
        self.value
            .processor
            .read()
            .or(Err(CharaError::Thread(ThreadError::Poison)))
            .and_then(|processor| processor.program())
    }

    fn current_directory(&self) -> Result<Option<String>, CharaError> {
        self.value
            .processor
            .read()
            .map(|processor| processor.current_directory.clone())
            .or(Err(CharaError::Thread(ThreadError::Poison)))
    }
}
