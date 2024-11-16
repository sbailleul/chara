use std::sync::{Arc, RwLock};


pub type Readonly<T> = Arc<RwLock<T>>;

pub fn readonly<T>(value: T) -> Readonly<T> {
    Arc::new(RwLock::new(value))
}

