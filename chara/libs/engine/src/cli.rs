use std::{collections::HashMap, hash::Hash};

use common::thread::Readonly;

use crate::reference_value::{RefOrValue, ReferencedValue};

pub type Arguments = RefOrValue<Vec<String>>;

pub type Environment = RefOrValue<HashMap<String, String>>;
