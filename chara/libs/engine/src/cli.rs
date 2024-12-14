use std::{collections::HashMap, hash::Hash};

use common::thread::Readonly;

use crate::reference_value::{RefValue, ReferencedValue};

pub type Arguments = RefValue<Vec<String>>;

pub type Environment = RefValue<HashMap<String, String>>;
