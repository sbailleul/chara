use std::fmt::Debug;

use common::thread::Readonly;

#[derive(Debug, Clone)]
pub enum RefValue<T: Debug + Clone> {
    Ref(String),
    ReferencedValue { r#ref: String, value: Readonly<T> },
    Value(T),
}
