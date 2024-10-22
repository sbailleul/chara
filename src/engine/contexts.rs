use crate::types::thread::Readonly;

use super::bootes::{Edge, Enricher, Metadata};

pub struct EdgeContext {
    pub metadata: (String, Readonly<Metadata>),
    pub edge: (String, Readonly<Edge>),
}
#[derive(Debug)]
pub struct EdgeEnricherContext {
    pub metadata: (String, Readonly<Metadata>),
    pub edge: (String, Readonly<Edge>),
    pub enricher: Readonly<Enricher>,
}
