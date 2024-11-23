
use common::merge::{Merge, Overwrite};


use super::definition::Definition;

impl Merge for Definition {
    fn merge(&mut self, other: &Definition) {
        self.arguments.merge(&other.arguments);
        self.environments.merge(&other.environments);
        self.edges.merge(&other.edges);
        self.foreign_definitions.merge(&other.foreign_definitions);
        self.location.overwrite(&other.location);
        self.metadata.merge(&other.metadata);
        self.processors.merge(&other.processors);
        self.tags.merge(&other.tags);
    }
}
