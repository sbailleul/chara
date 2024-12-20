use std::fs::File;

use definitions::definitions::{create_path, Definitions};
use engine::{draft::draft_definition::DefinedDefinitionInput, errors::CharaError};
use graph::Graph;

mod from_definition;
pub mod graph;

pub fn create_graph(input: &DefinedDefinitionInput) -> Result<(), CharaError> {
    let file_path = create_path("../../app/src/assets", Some("graph"))?;
    Definitions::read(input).and_then(|definition| {
        serde_json::to_writer(
            File::create(file_path).map_err(CharaError::IO)?,
            &Graph::from_definition(definition),
        )
        .map_err(CharaError::Json)
    })?;
    Ok(())
}
