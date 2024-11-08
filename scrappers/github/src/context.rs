use std::{
    fs::File,
    io::Read,
    path::Path,
};

use definitions::definition::{DefinitionContextDto, WritePermissionsDto};
use log::info;

use crate::{
    dtos::{EdgeDto, MetadataDto},
    errors::Error,
    github::{Github, GithubContext, Repository},
};

#[derive(Debug)]
pub struct Metadata {
    pub file: String,
    pub repository: Option<Repository>,
}
#[derive(Debug)]
pub struct Edge {}

#[derive(Debug)]
pub struct WritePermission {
    pub metadata: bool,
    pub edge: bool,
}
impl From<WritePermissionsDto> for WritePermission {
    fn from(value: WritePermissionsDto) -> Self {
        Self {
            metadata: value.metadata,
            edge: value.edge,
        }
    }
}
#[derive(Debug)]
pub struct DefinitionContext {
    pub location: Option<String>,
    pub github: Github,
    pub metadata: Metadata,
    pub edge: Option<Edge>,
    pub write: WritePermission,
}
impl DefinitionContext {
    pub fn new(value: DefinitionContextDto, github: GithubContext) -> Result<Self, Error> {
        let edge = value
            .edge
            .map(|edge| serde_json::from_value::<EdgeDto>(edge.value).map_err(Error::Json))
            .transpose()?;
        let metadata =
            serde_json::from_value::<MetadataDto>(value.metadata.value).map_err(Error::Json)?;
        Ok(Self {
            edge: edge.map(EdgeDto::into),
            metadata: metadata.into(),
            github: Github::new(Some(github))?,
            location: value.location,
            write: WritePermission::from(value.write),
        })
    }
    pub async fn workflow_content(&self) -> Result<String, Error> {
        if let Some(repository) = &self.metadata.repository {
            info!("Get workflow content for {repository}");
            return self
                .github
                .get_content(repository, &self.metadata.file)
                .await?
                .take_items()
                .first()
                .and_then(|content| content.decoded_content().clone())
                .ok_or(Error::MissingLocation);
        }
        if let Some(location) = &self.location {
            Path::new(&location)
                .parent()
                .ok_or(Error::InvalidParentDirectory(location.clone()))
                .and_then(|parent| {
                    let path = parent.join(&self.metadata.file);
                    if let Some(path) = path.to_str() {
                        info!("Open workflow file {path}");
                    }
                    File::open(path).map_err(Error::IO).and_then(|mut file| {
                        let mut text = String::new();
                        file.read_to_string(&mut text).map_err(Error::IO)?;
                        Ok(text)
                    })
                })
        } else {
            Err(Error::NoWorkspaceContentAvailable)
        }
    }
}
