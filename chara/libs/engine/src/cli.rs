use std::collections::HashMap;


use crate::reference_value::{LazyRefOrValue, RefOrValue};

pub type Arguments = RefOrValue<Vec<String>>;

pub type Environment = RefOrValue<HashMap<String, String>>;

pub type DraftArguments = LazyRefOrValue<Vec<String>>;

pub type DraftEnvironments = LazyRefOrValue<HashMap<String, String>>;