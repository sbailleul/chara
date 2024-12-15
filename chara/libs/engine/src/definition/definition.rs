use std::collections::HashMap;

use common::thread::Readonly;
use log::MetadataBuilder;

use crate::{clean::clean_definition::RefTag, processor::{Processor, ProcessorOverride}};

use super::{edge::Edge, foreign_definition::ForeignDefinition, install::Install, metadata::Metadata};

// pub struct Definition<TArguments, TEnvironment>{
//     pub name: String,
//     pub id: String,
//     pub location: Option<String>,
//     pub tags: HashMap<String, Readonly<RefTag>>,
//     pub metadata: HashMap<String, Readonly<Metadata<Edge<ProcessorOverride<>>>>>,
//     pub processors: HashMap<String, Readonly<Processor<TArguments, Install<TArguments, TEnvironment>, TEnvironment>>>,
//     pub arguments: HashMap<String, Readonly<Vec<String>>>,
//     pub environments: HashMap<String, Readonly<HashMap<String, String>>>,
// }