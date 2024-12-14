use common::{merge::Merge, thread::Readonly};
use serde_json::{Map, Value};

use crate::clean::clean_definition::CleanDefinition;


#[derive(Debug, Clone)]
pub struct Edge<TProcessorOverride, TForeignDefinition> {
    pub definition: Option<Readonly<TForeignDefinition>>,
    pub processor: Option<TProcessorOverride>,
    pub other: Map<String, Value>,
}


impl<TProcessorOverride:  Merge + Clone, TForeignDefinition:  Merge + Clone> Merge for Edge<TProcessorOverride, TForeignDefinition> {
    fn merge(&mut self, other: &Self) {
        self.definition.merge(&other.definition);
        self.processor.merge(&other.processor);
        self.other.merge(&other.other);
    }
}

#[derive(Debug, Clone)]
pub struct EdgeOverride<TArguments, TEnvironment, TEdge> {
    pub arguments: Vec<TArguments>,
    pub environments: Vec<TEnvironment>,
    pub edge: TEdge,
    pub other: Map<String, Value>,
    pub definition: Option<CleanDefinition>,
}


impl<TArguments: Merge + Clone, TEnvironment: Merge + Clone, TEdge: Merge> Merge for EdgeOverride<TArguments, TEnvironment, TEdge> {
    fn merge(&mut self, other: &Self) {
        self.arguments.merge(&other.arguments);
        self.environments.merge(&other.environments);
        self.edge.merge(&other.edge);
        self.other.merge(&other.other);
        self.definition.merge(&other.definition);
    }
}
