use std::{collections::HashMap, process::Command};

use engine::{cli::{Argument, Environment}, definition::{Enricher, Install}};

pub trait Cli {
    fn arguments(&self) -> Vec<Argument>;
    fn environments(&self) -> Vec<Environment>;
    fn program(&self) -> String;
    fn command(&self) -> Command {
        let mut cmd = Command::new(self.program());
        cmd.args(self.flatten_arguments())
            .envs(self.flatten_environments());
        cmd
    }

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

impl Cli for Install {
    fn arguments(&self) -> Vec<Argument> {
        self.arguments.clone()
    }
    fn environments(&self) -> Vec<Environment> {
        self.environments.clone()
    }
    fn program(&self) -> String {
        self.program.clone()
    }
}
impl Cli for Enricher {
    fn arguments(&self) -> Vec<Argument> {
        self.arguments.clone()
    }
    fn environments(&self) -> Vec<Environment> {
        self.environments.clone()
    }
    fn program(&self) -> String {
        self.program.clone()
    }
}
