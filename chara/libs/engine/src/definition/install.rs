use common::merge::{Merge, Overwrite};

use crate::cli::{DraftArguments, DraftEnvironments};


#[derive(Debug, Clone)]
pub struct Install {
    pub arguments: Vec<DraftArguments>,
    pub program: String,
    pub environments: Vec<DraftEnvironments>,
    pub current_directory: Option<String>,
}
impl Merge for Install {
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.program = other.program.clone();
        self.environments.merge(&other.environments);
        self.current_directory.overwrite(&other.current_directory);
    }
}
