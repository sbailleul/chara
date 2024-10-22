use serde::Serialize;
use serde_json::{Map, Value};


use super::contexts::EdgeEnricherContext;

#[derive(Debug, Serialize)]
pub struct EdgeEnricherContextDto {
    pub metadata: (String, Map<String, Value>),
    pub edge: (String, Map<String, Value>),
}
impl EdgeEnricherContextDto {
    pub fn from(context: &EdgeEnricherContext) -> Option<Self> {
        let edge = context.edge.1.read().ok().map(|edge| edge.other.clone());
        let metadata = context
            .metadata
            .1
            .read()
            .ok()
            .map(|metadata| metadata.other.clone());
        if let Some((edge, metadata)) = edge.zip(metadata) {
            Some(EdgeEnricherContextDto {
                edge: (context.edge.0.clone(), edge),
                metadata: (context.metadata.0.clone(), metadata),
            })
        } else {
            None
        }
    }
    pub fn serialize(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}
