use thiserror::Error;

pub mod thread;
#[derive(Debug, Error)]
pub enum ThreadError{
    #[error("Poison error")]
    Poison,
    #[error("Join error")]
    Join
}