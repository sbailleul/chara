use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DefinitionSummaryDto {
    id: String,
    name: String,
}
