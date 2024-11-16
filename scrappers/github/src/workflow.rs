use std::collections::{BTreeMap, HashMap};

use definitions::definition::{
    DefinitionDto, MetadataDto, ProcessorResultDto, ReferenceOrObjectDto,
};
use serde_json::{Map, Value};

use crate::{context::DefinitionContext, dtos::OtherMetadataDto, errors::Error};

#[derive(Debug)]
pub struct Job {
    pub uses: String,
}

impl Job {
    pub fn to_other_metadata(&self, ctx: &DefinitionContext) -> OtherMetadataDto {
        if self.uses.starts_with("./") {
            let repo = ctx
                .metadata
                .repository
                .clone()
                .map(|r| (r.owner, r.name))
                .unzip();
            OtherMetadataDto {
                file: self.uses.clone(),
                owner: repo.0,
                repository: repo.1,
            }
        } else {
            let file_segments = self
                .uses
                .splitn(3, '/')
                .map(String::from)
                .collect::<Vec<String>>();
            OtherMetadataDto {
                file: file_segments[2].clone(),
                owner: file_segments.first().cloned(),
                repository: file_segments.get(1).cloned(),
            }
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    pub name: String,
    pub jobs: BTreeMap<String, Job>,
}

impl Workflow {
    pub fn to_processor_result(
        self,
        context: DefinitionContext,
    ) -> Result<ProcessorResultDto, Error> {
        self.to_definition(context)
            .map(|definition| ProcessorResultDto {
                definition: Some(definition),
                enrichment: None,
            })
    }
    pub fn to_definition(self, context: DefinitionContext) -> Result<DefinitionDto, Error> {
        Ok(DefinitionDto {
            name: self.name,
            metadata: self
                .jobs
                .iter()
                .map(|(name, job)| -> Result<(String, MetadataDto), Error> {
                    Ok((
                        name.clone(),
                        MetadataDto {
                            other: serde_json::to_value(job.to_other_metadata(&context))
                                .map(|value| {
                                    value
                                        .as_object()
                                        .unwrap_or(&Map::<String, Value>::new())
                                        .clone()
                                })
                                .map_err(Error::Json)?,
                            processor: None,
                            tags: vec![],
                            edges: context
                                .edge
                                .clone()
                                .map(|e| ReferenceOrObjectDto::Reference(e.name))
                                .into_iter()
                                .collect(),
                        },
                    ))
                })
                .collect::<Result<HashMap<String, MetadataDto>, Error>>()?,
            edges: HashMap::new(),
            arguments: HashMap::new(),
            environments: HashMap::new(),
            location: None,
            tags: HashMap::new(),
            processors: HashMap::new(),
        })
    }
}
