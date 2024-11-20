use std::{collections::HashSet, hash::Hash};

use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Serialize, PartialEq, Eq)]
pub struct Edge {
    pub id: String,
    pub source: String,
    pub target: String,
}
impl Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
#[derive(Serialize, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub data: Map<String, Value>,
}
impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
#[derive(Serialize)]
pub struct Graph {
    pub nodes: HashSet<Node>,
    pub edges: HashSet<Edge>,
}
