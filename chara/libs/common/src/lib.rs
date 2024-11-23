use thiserror::Error;

pub mod thread;
pub mod merge;
#[derive(Debug, Error)]
pub enum ThreadError{
    #[error("Poison error")]
    Poison,
    #[error("Join error")]
    Join
}