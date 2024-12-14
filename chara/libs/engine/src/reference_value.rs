use std::{fmt::Debug, hash::Hash};

use common::thread::Readonly;
#[derive(Debug, Clone)]
pub struct ReferencedValue<T> {
    pub r#ref: String,
    pub value: T,
}
#[derive(Debug, Clone)]
pub enum RefValue<T> {
    ReferencedValue(ReferencedValue<Readonly<T>>),
    Value(T),
}

impl<T: PartialEq> PartialEq for RefValue<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (
                Self::ReferencedValue(ReferencedValue { r#ref: ref0, .. }),
                Self::ReferencedValue(ReferencedValue { r#ref: ref1, .. }),
            ) => ref0.eq(ref1),
            _ => false,
        }
    }
}
impl<T: PartialEq> Eq for RefValue<T> {}

impl<T: Default + Clone> RefValue<T> {
    pub fn unwrap(&self) -> T {
        match self {
            RefValue::ReferencedValue(ReferencedValue { r#ref, value }) => {
                value.read().map(|value| value.clone()).unwrap_or_default()
            }
            RefValue::Value(v) => v.clone(),
        }
    }
}
impl<T: Hash> Hash for RefValue<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            RefValue::Value(v) => v.hash(state),
            RefValue::ReferencedValue(ReferencedValue { r#ref, value }) => {
                if let Ok(v) = value.read() {
                    v.hash(state);
                }
                r#ref.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum LazyRefValue<T> {
    Ref(String),
    ReferencedValue(ReferencedValue<Readonly<T>>),
    Value(T),
}

impl<T> LazyRefValue<T> {
    pub fn reference(&self) -> Option<String>{
        match self {
            LazyRefValue::Ref(reference) =>Some(reference.clone()),
            LazyRefValue::ReferencedValue(ReferencedValue { r#ref,.. }) => Some(r#ref.clone()),
            LazyRefValue::Value(_) => None,
        }
    }
    pub fn referenced_value(reference: String, value: Readonly<T>) -> LazyRefValue<T> {
        Self::ReferencedValue(ReferencedValue {
            r#ref: reference,
            value,
        })
    }
}
