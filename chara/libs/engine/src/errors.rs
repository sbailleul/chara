use thiserror::Error;
use types::ThreadError;

#[derive(Debug, Error)]
pub enum DefinitionError {
    #[error("Parse error {0}")]
    Parse(String),
    #[error("Access error {0}")]
    Access(String),
    #[error("Process error {0}")]
    Process(String),
    #[error("Thread error {0}")]
    Thread(ThreadError),
    #[error("Cli error {0}")]
    Cli(CliError),
}
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Thread error {0}")]
    Thread(ThreadError),
    #[error("Path not found {0}")]
    PathNotFound(String),
}
#[derive(Debug, Error)]
pub enum Error {
    #[error("Thread error {0}")]
    Thread(ThreadError),
    #[error("Process error {0}")]
    Process(DefinitionError),
}