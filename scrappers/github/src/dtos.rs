use std::collections::BTreeMap;

use serde::Deserialize;

use crate::{
    context::{Edge, Metadata}, github::Repository, workflow::{Job, Workflow}
};

#[derive(Debug, Deserialize)]
pub struct MetadataDto {
    pub file: String,
    pub owner: Option<String>,
    pub repository: Option<String>,
}
impl Into<Metadata> for MetadataDto {
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

impl Into<Edge> for EdgeDto {
    fn into(self) -> Edge {
        Edge {}
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct JobDto {
    pub uses: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct WorkflowDto {
    #[serde(default)]
    pub jobs: BTreeMap<String, JobDto>,
}

impl WorkflowDto {
    pub fn into(self) -> Workflow {
        Workflow {
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
