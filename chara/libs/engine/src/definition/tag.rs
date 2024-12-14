use std::collections::HashMap;

use common::{merge::{Merge, Overwrite}, thread::Readonly};
use serde_json::Value;

use crate::reference_value::ReferencedValue;

#[derive(Debug, Clone)]
pub struct Tag {
    pub label: Option<String>,
    pub tags: HashMap<String, Readonly<RefTag>>,
    pub other: Value,
}
impl Merge for Tag{
    fn merge(&mut self, other: &Self) {
        self.label.overwrite(&other.label);
        self.other.merge(&other.other);
        self.tags.merge(&other.tags);
    }
}