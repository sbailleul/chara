use std::{
    env,
    fs::{self, canonicalize, read_dir, DirEntry, File},
    io::BufReader,
    path::PathBuf,
};

use common::{thread::Readonly, ThreadError};
use engine::{
    contexts::ProcessorContext,
    definition::definition::Definition,
    definition::input::{BaseDefinitionInput, DefinedDefinitionInput},
    errors::CharaError,
    processor::{Enrichment, ProcessorResult},
    Definitions as ForeignDefinitions,
};
use log::info;
use serde::Deserialize;

use crate::{
    cli::Cli,
    dto::{
        definition::{DefinitionDto, ProcessorResultDto},
        definition_info::DefinitionSummaryDto,
    },
};

pub struct Definitions {}
pub struct ReadOutput<T> {
    output: T,
    location: Option<String>,
}
impl Definitions {
    pub fn read(input: &DefinedDefinitionInput) -> Result<DefinitionDto, CharaError> {
        Definitions::read_output::<DefinitionDto>(input).map(|def| def.output)
    }
    pub fn get_from_path(path: String) -> Result<Definition, CharaError> {
        Definitions::read_output::<DefinitionDto>(&BaseDefinitionInput::File(path.clone()))
            .map(|read_output| DefinitionDto::map_overwrite_location(read_output.output, path))
    }
    pub fn get_from_definition(definition: DefinitionDto) -> Result<Definition, CharaError> {
        Ok(definition.map())
    }
    fn read_from_file<T: for<'a> Deserialize<'a>>(
        path: &String,
        location: &mut Option<String>,
    ) -> Result<T, CharaError> {
        File::open(path).map_err(CharaError::IO).and_then(|file| {
            let absolute_location =
                canonicalize(path)
                    .map_err(CharaError::IO)
                    .and_then(|absolute_location| {
                        absolute_location
                            .to_str()
                            .map(|absolute_location| absolute_location.to_string())
                            .ok_or(CharaError::InvalidPath(path.clone()))
                    })?;
            *location = Some(absolute_location);
            serde_json::from_reader(BufReader::new(file)).map_err(CharaError::Json)
        })
    }

    fn read_output<T: for<'a> Deserialize<'a>>(
        input: &DefinedDefinitionInput,
    ) -> Result<ReadOutput<T>, CharaError> {
        let mut location = None;
        dbg!(input);
        match input {
            BaseDefinitionInput::Id(id) => Definitions::result_path(id.as_str())
                .and_then(|path| Definitions::read_from_file(&path, &mut location)),
            BaseDefinitionInput::File(path) => Definitions::read_from_file(path, &mut location),
            BaseDefinitionInput::Text(content) => {
                serde_json::from_str(&content).map_err(CharaError::Json)
            }
            BaseDefinitionInput::Processor(processor) => {
                info!("Run definition processor");
                processor
                    .output_stdout(None)
                    .and_then(|stdout| serde_json::from_str(&stdout).map_err(CharaError::Json))
            }
            BaseDefinitionInput::Value(value) => {
                serde_json::from_value(value.clone()).map_err(CharaError::Json)
            }
        }
        .map(|output| ReadOutput { output, location })
    }

    fn result_path(id: &str) -> Result<String, CharaError> {
        create_path("chara_results", Some(id))
    }
    pub fn all_definitions() -> Result<Vec<DefinitionSummaryDto>, CharaError> {
        read_dir(get_directory("chara_results")?)
            .map_err(CharaError::IO)
            .map(|res| {
                res.map(|e| {
                    e.map_err(CharaError::IO)
                        .map(|e| {
                            e.path().to_str().map(|path| {
                                dbg!(path);
                                Self::read_from_file::<DefinitionSummaryDto>(&path.to_string(), &mut None)
                            })
                        })
                        .transpose()
                })
            })?
            .flatten()
            .flatten()
            .collect::<Result<Vec<DefinitionSummaryDto>, CharaError>>()
    }
}
impl ForeignDefinitions for Definitions {
    fn get(&self, input: &DefinedDefinitionInput) -> Result<Definition, CharaError> {
        Definitions::read_output::<DefinitionDto>(input).map(|read_output| {
            DefinitionDto::map_with_location(read_output.output, read_output.location, None)
        })
    }

    fn save(&self, definition: &Definition) -> Result<(), CharaError> {
        let path = Definitions::result_path(definition.id.as_str())?;
        info!("Save result at {path}");
        serde_json::to_writer(
            File::create(path).map_err(CharaError::IO)?,
            &DefinitionDto::from_definition(definition),
        )
        .map_err(CharaError::Json)?;
        Ok(())
    }
    fn enrich(
        &self,
        context: &ProcessorContext,
        parent: Readonly<Definition>,
    ) -> Result<ProcessorResult, CharaError> {
        context
            .processor
            .processor
            .value
            .read()
            .or(Err(CharaError::Thread(ThreadError::Poison)))
            .and_then(|processor| {
                if let Some(install) = &processor.install {
                    info!("Run installation");
                    let install_output = install.output_stdout(None)?;
                    info!("Installation done : {install_output}");
                }
                let context =
                    serde_json::to_string(&context.definition).map_err(CharaError::Json)?;

                let path = create_path("processor_outputs", None)?;
                processor
                    .output_stdout(Some(vec![
                        "--context".to_string(),
                        context,
                        "--output".to_string(),
                        path.clone(),
                    ]))
                    .and_then(|_output| {
                        Definitions::read_output::<ProcessorResultDto>(&BaseDefinitionInput::File(
                            path,
                        ))
                    })
                    .map(|result| ProcessorResult {
                        definition: result
                            .output
                            .definition
                            .map(|def| def.map_with_location(result.location, Some(parent))),
                        enrichment: result.output.enrichment.map(|enrichment| Enrichment {
                            edge: enrichment.edge,
                            metadata: enrichment.metadata,
                        }),
                    })
            })
    }
}

pub fn create_path(name: &str, file_name: Option<&str>) -> Result<String, CharaError> {
    let mut uniq_path =
        get_directory(name)?.join(file_name.unwrap_or(&uuid::Uuid::new_v4().to_string()));
    uniq_path.set_extension("json");
    uniq_path
        .to_str()
        .ok_or(CharaError::ParsePath)
        .map(|path| path.to_string())
}

fn get_directory(name: &str) -> Result<PathBuf, CharaError> {
    let path = env::current_dir().map_err(CharaError::IO)?.join(name);
    if !path.exists() {
        fs::create_dir(&path).map_err(CharaError::IO)?;
    }
    Ok(path)
}
