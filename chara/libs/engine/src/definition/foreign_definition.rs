use common::merge::Merge;

use crate::draft::draft_definition::{DraftDefinition};

use super::input::DraftDefinitionInput;

#[derive(Debug, Clone)]
pub struct ForeignDefinition{
    pub input: Option<DraftDefinitionInput>,
    pub output: Option<DraftDefinition>,
}
impl ForeignDefinition{
    pub fn input(input: DraftDefinitionInput) -> Self {
        ForeignDefinition {
            input: Some(input),
            output: None,
        }
    }
    pub fn output(output: DraftDefinition) -> Self {
        ForeignDefinition {
            input: None,
            output: Some(output),
        }
    }
}

impl Merge for ForeignDefinition {
    fn merge(&mut self, other: &Self) {
        self.input.merge(&other.input);
        self.output.merge(&other.output);
    }
}
