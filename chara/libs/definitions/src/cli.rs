use std::{collections::HashMap, process::Command};

use engine::{
    cli::{Argument, Environment},
    definition::{Install, Processor, ProcessorOverride},
};

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
    fn program(&self) -> Option<String>;
    fn command(&self) -> Option<Command> {
        self.program().map(|program| {
            let mut cmd = Command::new(program);
            cmd.args(self.flatten_arguments())
                .envs(self.flatten_environments());
            cmd
        })
    }
    fn output_stdout(&self) -> Option<String> {
        self.command().and_then(|mut cmd| {
            cmd.output()
                .inspect_err(|err| println!("{err}"))
                .ok()
                .and_then(|output| {
                    let stdout = String::from_utf8(output.stdout);
                    if let Ok(stdout) = stdout {
                        Some(stdout)
                    } else {
                        String::from_utf8(output.stderr)
                            .inspect_err(|err| println!("parse error {err}")).ok()
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
    fn program(&self) -> Option<String> {
        Some(self.program.clone())
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
    fn program(&self) -> Option<String> {
        Some(self.program.clone())
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
    fn program(&self) -> Option<String> {
        self.processor
            .read()
            .ok()
            .and_then(|processor| processor.program())
    }
}
