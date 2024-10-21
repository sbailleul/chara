use crate::types::thread::Readonly;

use super::bootes::{Edge, Metadata, Scrapper};

pub struct EdgeContext {
    pub metadata: (String, Readonly<Metadata>),
    pub edge: (String, Readonly<Edge>),
}
#[derive(Debug)]
pub struct ScrapperContext {
    pub metadata: (String, Readonly<Metadata>),
    pub edge: (String, Readonly<Edge>),
    pub scrapper: Readonly<Scrapper>,
}
