use std::{fmt::Debug, hash::Hash};

use common::thread::Readonly;
#[derive(Debug, Clone)]
pub struct ReferencedValue<T> {
    pub r#ref: String,
    pub value: T,
}
#[derive(Debug, Clone)]
pub enum RefOrValue<T> {
    ReferencedValue(ReferencedValue<Readonly<T>>),
    Value(T),
}

impl<T: PartialEq> PartialEq for RefOrValue<T> {
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
impl<T: PartialEq> Eq for RefOrValue<T> {}

impl<T: Default + Clone> RefOrValue<T> {
    pub fn unwrap(&self) -> T {
        match self {
            RefOrValue::ReferencedValue(ReferencedValue { r#ref, value }) => {
                value.read().map(|value| value.clone()).unwrap_or_default()
            }
            RefOrValue::Value(v) => v.clone(),
        }
    }
}
impl<T: Hash> Hash for RefOrValue<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            RefOrValue::Value(v) => v.hash(state),
            RefOrValue::ReferencedValue(ReferencedValue { r#ref, value }) => {
                if let Ok(v) = value.read() {
                    v.hash(state);
                }
                r#ref.hash(state);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum LazyRefOrValue<T> {
    Ref(String),
    ReferencedValue(ReferencedValue<Readonly<T>>),
    Value(T),
}

impl<T> LazyRefOrValue<T> {
    pub fn reference(&self) -> Option<String>{
        match self {
            LazyRefOrValue::Ref(reference) =>Some(reference.clone()),
            LazyRefOrValue::ReferencedValue(ReferencedValue { r#ref,.. }) => Some(r#ref.clone()),
            LazyRefOrValue::Value(_) => None,
        }
    }
    pub fn referenced_value(reference: String, value: Readonly<T>) -> LazyRefOrValue<T> {
        Self::ReferencedValue(ReferencedValue {
            r#ref: reference,
            value,
        })
    }
}
#[derive(Debug, Clone)]
pub enum LazyRef<T> {
    Ref(String),
    ReferencedValue(ReferencedValue<Readonly<T>>),
}

impl <T> LazyRef<T>{
    pub fn referenced_value(reference: String, value: Readonly<T>) -> LazyRef<T> {
        Self::ReferencedValue(ReferencedValue {
            r#ref: reference,
            value,
        })
    }
    pub fn reference(&self) ->String{
        match self {
            LazyRef::Ref(reference) =>reference.clone(),
            LazyRef::ReferencedValue(ReferencedValue { r#ref,.. }) => r#ref.clone(),
        }
    }
}