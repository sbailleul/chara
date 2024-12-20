use common::merge::{Merge, Overwrite};


#[derive(Debug, Clone)]
pub struct Install<TArguments, TEnvironment> {
    pub arguments: Vec<TArguments>,
    pub program: String,
    pub environments: Vec<TEnvironment>,
    pub current_directory: Option<String>,
}
impl<TArguments: Merge + Clone, TEnvironment: Merge + Clone> Merge
    for Install<TArguments, TEnvironment>
{
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.program = other.program.clone();
        self.environments.merge(&other.environments);
        self.current_directory.overwrite(&other.current_directory);
    }
}
