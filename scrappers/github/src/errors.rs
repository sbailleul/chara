use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Json error {0}")]
    Json(serde_json::Error),
    #[error("Yaml error {0}")]
    Yaml(serde_yaml::Error),
    #[error("Octocrab error {0}")]
    Octocrab(octocrab::Error),
    #[error("Context should specify original definition location")]
    MissingLocation,
    #[error("Definition file {0} doesn't have valid parent directory")]
    InvalidParentDirectory(String),
    #[error("IO error")]
    IO(std::io::Error),
    #[error("No available workspace content")]
    NoWorkspaceContentAvailable,
    #[error("Json web token {0}")]
    JsonWebToken(jsonwebtoken::errors::Error),
    #[error("Invalid reusable workflow path {0}")]
    InvalidReusableWorkflowPath(String),
}
