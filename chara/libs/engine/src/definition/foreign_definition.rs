
use crate::draft::draft_definition::DraftDefinition;


#[derive(Debug, Clone)]
pub struct ForeignDefinition<TInput> {
    pub input: Option<TInput>,
    pub output: Option<DraftDefinition>,
}
impl <TInput> ForeignDefinition<TInput> {
    pub fn input(input: TInput ) -> Self {
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

