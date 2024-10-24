use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Debug, Serialize)]
pub struct WritePermissionsDto {
    pub metadata: bool,
    pub edge: bool,
}
#[derive(Debug, Serialize)]
pub struct BootContextDto {
    pub write: WritePermissionsDto,
    pub metadata: (String, Map<String, Value>),
    pub edge: Option<(String, Map<String, Value>)>,
}
