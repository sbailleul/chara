use std::{collections::HashMap, process::Command};

use engine::{
    cli::{Argument, Environment},
    definition::{Processor, Install},
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
    fn program(&self) -> String;
    fn command(&self) -> Command {
        let mut cmd = Command::new(self.program());
        cmd.args(self.flatten_arguments())
            .envs(self.flatten_environments());
        cmd
    }
    fn output_stdout(&self) -> Option<String >{
        self.command().output().ok().and_then(|output| String::from_utf8(output.stdout).ok())
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
impl Cli  for Install {

    fn program(&self) -> String {
        self.program.clone()
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

    fn program(&self) -> String {
        self.program.clone()
    }
}

