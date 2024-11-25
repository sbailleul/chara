use std::{collections::HashMap, hash::Hash, sync::Arc};

use serde_json::{Map, Value};

use crate::thread::Readonly;

pub trait Merge {
    fn merge(&mut self, other: &Self);
}
pub trait Overwrite {
    fn overwrite(&mut self, other: &Self);
}
impl<T: Clone> Merge for Vec<T> {
    fn merge(&mut self, other: &Self) {
        self.append(&mut other.clone());
    }
}

impl<Key: Eq + Clone + Hash, Value: Clone + Merge> Merge for HashMap<Key, Value> {
    fn merge(&mut self, other: &Self) {
        for (other_key, other_value) in other.iter() {
            if let Some(value) = self.get_mut(other_key) {
                value.merge(other_value);
            } else {
                self.insert(other_key.clone(), other_value.clone());
            }
        }
    }
}

impl Merge for HashMap<String, String> {
    fn merge(&mut self, other: &Self) {
        for (other_key, other_value) in other.iter() {
            self.insert(other_key.clone(), other_value.clone());
        }
    }
}

impl Merge for Value {
    fn merge(&mut self, other: &Self) {
        match (self, other) {
            (Value::Object(a), Value::Object(b)) => {
                a.merge(b);
            }
            (a, b) => *a = b.clone(),
        }
    }
}
impl Merge for Map<String, Value> {
    fn merge(&mut self, other: &Self) {
        for (k, v) in other {
            self.entry(k.clone()).or_insert(Value::Null).merge(v);
        }
    }
}
impl<T: Merge + Clone> Merge for Readonly<T> {
    fn merge(&mut self, other: &Self) {
        if Arc::ptr_eq(self, other) {
            return;
        }
        if let (Ok(mut value), Ok(other_value)) = (self.write(), other.read()) {
            value.merge(&other_value);
        }
    }
}
impl<T: Merge + Clone> Merge for Option<T> {
    fn merge(&mut self, other: &Self) {
        match (self.as_mut(), other) {
            (None, Some(other_value)) => *self = Some(other_value.clone()),
            (Some(value), Some(other_value)) => {
                value.merge(other_value);
            }
            _ => (),
        }
    }
}

impl<T: Clone> Overwrite for Option<T> {
    fn overwrite(&mut self, other: &Self) {
        match (self.as_mut(), other) {
            (None, Some(other_value)) => *self = Some(other_value.clone()),
            _ => (),
        }
    }
}
