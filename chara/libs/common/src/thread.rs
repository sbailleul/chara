use std::sync::{Arc, RwLock};


pub type Readonly<T> = Arc<RwLock<T>>;

pub fn readonly<T>(value: T) -> Readonly<T> {
    Arc::new(RwLock::new(value))
}


pub trait Read<T>{
    fn read_ref_or_default(&self) -> T; 
    fn read_or_default(self) -> T; 
}


impl <T: Default + Clone> Read<T> for Readonly<T>{
    fn read_ref_or_default(&self) -> T {
        if let Ok(lock) = self.read(){
            lock.clone()
        }else{
            T::default()
        }
    }
    
    fn read_or_default(self) -> T {
        if let Ok(lock) = self.read(){
            lock.clone()
        }else{
            T::default()
        }
    }
}
