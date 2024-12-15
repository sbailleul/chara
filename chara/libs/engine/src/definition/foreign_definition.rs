use common::merge::Merge;

use crate::draft::draft_definition::DraftDefinition;

#[derive(Debug, Clone)]
pub struct ForeignDefinition<TInput> {
    pub input: Option<TInput>,
    pub output: Option<DraftDefinition>,
}
impl<TInput> ForeignDefinition<TInput> {
    pub fn input(input: TInput) -> Self {
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

impl<T: Merge + Clone> Merge for ForeignDefinition<T> {
    fn merge(&mut self, other: &Self) {
        self.input.merge(&other.input);
        self.output.merge(&other.output);
    }
}
