use std::{
    env, fs::{self, canonicalize, File}, io::BufReader, path::Path
};

use cli::Cli;
use definition::DefinitionDto;
use engine::{
    contexts::ProcessorContext,
    definition::{Definition, DefinitionInput},
    errors::DefinitionError,
    Definitions as ForeignDefinitions,
};
mod cli;
pub mod definition;
mod map;
use log::info;
use types::ThreadError;
pub struct Definitions {}
impl ForeignDefinitions for Definitions {
    fn get(&self, input: &DefinitionInput) -> Result<Definition, DefinitionError> {
        let mut location = None;
        match input {
            DefinitionInput::File(path) => {
                File::open(path)
                    .map_err(DefinitionError::IO)
                    .and_then(|file| {
                        let absolute_location = canonicalize(path)
                            .map_err(DefinitionError::IO)
                            .and_then(|absolute_location| {
                                absolute_location
                                    .to_str()
                                    .map(|absolute_location| absolute_location.to_string())
                                    .ok_or(DefinitionError::InvalidPath(path.clone()))
                            })?;
                        location = Some(absolute_location);
                        serde_json::from_reader(BufReader::new(file)).map_err(DefinitionError::Json)
                    })
            }
            DefinitionInput::Text(content) => {
                serde_json::from_str(&content).map_err(DefinitionError::Json)
            }
            DefinitionInput::Processor(processor) => {
                info!("Run definition processor");
                processor
                    .output_stdout(None)
                    .and_then(|stdout| serde_json::from_str(&stdout).map_err(DefinitionError::Json))
            }
            DefinitionInput::Value(value) => {
                serde_json::from_value(value.clone()).map_err(DefinitionError::Json)
            }
        }
        .map(|dto| DefinitionDto::map(dto, location))
    }

    fn enrich(&self, context: &ProcessorContext) -> Result<Definition, DefinitionError> {
        context
            .processor
            .processor
            .read()
            .or(Err(DefinitionError::Thread(ThreadError::Poison)))
            .and_then(|processor| {
                if let Some(install) = &processor.install {
                    info!("Run installation");
                    let install_output = install.output_stdout(None)?;
                    info!("Installation done : {install_output}");
                }
                let context =
                    serde_json::to_string(&context.definition).map_err(DefinitionError::Json)?;

                let path = output_path()?;
                processor
                    .output_stdout(Some(vec![
                        "--context".to_string(),
                        context,
                        "--output".to_string(),
                        path.clone(),
                    ]))
                    .and_then(|_output| self.get(&DefinitionInput::File(path)))
            })
    }
}

fn output_path() -> Result<String, DefinitionError> {
    let path = env::current_dir().map_err(DefinitionError::IO)?.join("outputs");
    if !path.exists(){
        fs::create_dir(&path).map_err(DefinitionError::IO)?;
    }
    let mut uniq_path = path.join(uuid::Uuid::new_v4().to_string());
    uniq_path.set_extension("json");
    uniq_path
        .to_str()
        .ok_or(DefinitionError::ParsePath)
        .map(|path| path.to_string())
}
