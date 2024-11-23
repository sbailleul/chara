use common::merge::Merge;

use super::{definition::Definition, input::DefinitionInput};

#[derive(Debug, Clone)]
pub struct ForeignDefinition {
    pub input: Option<DefinitionInput>,
    pub output: Option<Definition>,
}
impl ForeignDefinition {
    pub fn input(input: DefinitionInput) -> Self {
        ForeignDefinition {
            input: Some(input),
            output: None,
        }
    }
    pub fn output(output: Definition) -> Self {
        ForeignDefinition {
            input: None,
            output: Some(output),
        }
    }
}


impl Merge for ForeignDefinition{
    fn merge(&mut self, other: &Self) {
        self.input.merge(&other.input);
        self.output.merge(&other.output);
    }
}