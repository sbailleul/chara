use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    context::{Edge, Metadata},
    github::Repository,
    workflow::{Job, Workflow},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct OtherMetadataDto {
    pub file: String,
    pub owner: Option<String>,
    pub repository: Option<String>,
}
impl Into<Metadata> for OtherMetadataDto {
    fn into(self) -> Metadata {
        Metadata {
            repository: self
                .owner
                .zip(self.repository)
                .map(|(owner, repository)| Repository {
                    name: repository,
                    owner,
                }),
            file: self.file,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct EdgeDto {}

impl EdgeDto {
    pub fn to_edge(self, name: String) -> Edge {
        Edge { name }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct JobDto {
    pub uses: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct WorkflowDto {
    pub name: String,
    #[serde(default)]
    pub jobs: BTreeMap<String, JobDto>,
}

impl WorkflowDto {
    pub fn into(self) -> Workflow {
        Workflow {
            name: self.name,
            jobs: self
                .jobs
                .into_iter()
                .map(|(k, job)| {
                    job.uses.map(|reusable_workflow| {
                        (
                            k,
                            Job {
                                uses: reusable_workflow,
                            },
                        )
                    })
                })
                .flatten()
                .collect(),
        }
    }
}
