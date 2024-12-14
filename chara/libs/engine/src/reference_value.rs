use std::{alloc::Layout, borrow::BorrowMut, fmt::Debug, hash::Hash};

use common::{
    merge::Merge,
    thread::{readonly, Readonly},
};
#[derive(Debug, Clone)]
pub struct ReferencedValue<T> {
    pub r#ref: String,
    pub value: T,
}

impl<T: Merge + Clone> Merge for ReferencedValue<T> {
    fn merge(&mut self, other: &Self) {
        self.r#ref = other.r#ref.clone();
        self.value.merge(&other.value);
    }
}
#[derive(Debug, Clone)]
pub enum RefOrValue<T> {
    ReferencedValue(ReferencedValue<Readonly<T>>),
    Value(T),
}
impl<T: Merge + Clone> Merge for RefOrValue<T> {
    fn merge(&mut self, other: &Self) {
        match (self.borrow_mut(), other) {
            (
                RefOrValue::ReferencedValue(referenced_value1),
                RefOrValue::ReferencedValue(referenced_value2),
            ) => referenced_value1.merge(referenced_value2),
            (RefOrValue::ReferencedValue(referenced_value), RefOrValue::Value(v2)) => {
                if let Ok(mut v1) = referenced_value.value.write() {
                    v1.merge(&v2);
                }
            }
            (RefOrValue::Value(v1), RefOrValue::ReferencedValue(referenced_value)) => {
                if let Ok(v2) = referenced_value.value.read() {
                    v1.merge(&v2);
                }
                *self = RefOrValue::ReferencedValue(ReferencedValue {
                    r#ref: referenced_value.r#ref.clone(),
                    value: readonly(v1.clone()),
                })
            }
            (RefOrValue::Value(v1), RefOrValue::Value(v2)) => v1.merge(v2),
        }
    }
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
    pub fn reference(&self) -> Option<String> {
        match self {
            LazyRefOrValue::Ref(reference) => Some(reference.clone()),
            LazyRefOrValue::ReferencedValue(ReferencedValue { r#ref, .. }) => Some(r#ref.clone()),
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

impl<T> LazyRef<T> {
    pub fn referenced_value(reference: String, value: Readonly<T>) -> LazyRef<T> {
        Self::ReferencedValue(ReferencedValue {
            r#ref: reference,
            value,
        })
    }
    pub fn reference(&self) -> String {
        match self {
            LazyRef::Ref(reference) => reference.clone(),
            LazyRef::ReferencedValue(ReferencedValue { r#ref, .. }) => r#ref.clone(),
        }
    }
}

impl<T: Merge + Clone> Merge for LazyRef<T> {
    fn merge(&mut self, other: &Self) {
        match (self.borrow_mut(), other) {
            (LazyRef::Ref(ref1), LazyRef::Ref(ref2)) => *ref1 = ref2.clone(),
            (LazyRef::Ref(_), LazyRef::ReferencedValue(referenced_value)) => {
                *self = LazyRef::ReferencedValue(referenced_value.clone())
            }
            (LazyRef::ReferencedValue(referenced_value), LazyRef::Ref(ref2)) => {
                referenced_value.r#ref = ref2.clone()
            }
            (
                LazyRef::ReferencedValue(referenced_value1),
                LazyRef::ReferencedValue(referenced_value2),
            ) => referenced_value1.merge(referenced_value2),
        }
    }
}
