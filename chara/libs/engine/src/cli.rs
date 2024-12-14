use std::collections::HashMap;


use crate::reference_value::RefOrValue;

pub type Arguments = RefOrValue<Vec<String>>;

pub type Environment = RefOrValue<HashMap<String, String>>;
