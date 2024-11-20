use std::string::FromUtf8Error;

use thiserror::Error;
use types::ThreadError;

#[derive(Debug, Error)]
pub enum CharaError {
    #[error("Parse json error {0}")]
    Json(serde_json::Error),
    #[error("Parse utf8 {0}")]
    ParseUtf8(FromUtf8Error),
    #[error("File error {0}")]
    IO(std::io::Error),
    #[error("Process error {0}")]
    Process(String),
    #[error("Thread error {0}")]
    Thread(ThreadError),
    #[error("Cli error {0}")]
    Cli(String),
    #[error("Definition location is invalid {0}")]
    InvalidPath(String),
    #[error("Path parsing failed")]
    ParsePath,
}
