use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

pub fn find_all<T>(object: &Map<String, Value>) -> Vec<T>
where
    T: DeserializeOwned,
{
    let mut leafs = vec![];
    for (_property, value) in object {
        if let Ok(leaf) = serde_json::from_value::<T>(value.clone()) {
            leafs.push(leaf);
        } else if let Value::Object(properties) = value {
            leafs.append(&mut find_all::<T>(properties));
        }
    }
    return leafs;
}

pub fn find_by_path<T>(mut object: &Map<String, Value>, path: &String) -> Option<T>
where
    T: DeserializeOwned,
{
    let segments = path
        .trim_start_matches("#/")
        .split('/')
        .collect::<Vec<&str>>();
    for i in 0..segments.len() {
        if let (true, Some(value)) = (
            i == segments.len() - 1,
            object
                .get(segments[i])
                .and_then(|v| serde_json::from_value(v.clone()).ok()),
        ) {
            return value;
        } else if let Some(Value::Object(properties)) = object.get(segments[i]) {
            object = properties
        } else {
            break;
        }
    }
    None
}
