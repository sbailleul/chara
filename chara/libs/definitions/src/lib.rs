use std::{fs::File, io::BufReader};

use cli::Cli;
use definition::DefinitionDto;
use engine::{
    contexts::ProcessorContext, definition::{Definition, DefinitionInput}, errors::DefinitionError, Definitions as ForeignDefinitions
};
mod cli;
pub mod definition;
mod map;

pub struct Definitions {}
impl ForeignDefinitions for Definitions {
    fn get(&self, definition: &DefinitionInput) -> Result<Definition, DefinitionError> {
        match definition {
            DefinitionInput::File(path) => File::open(path)
                .map_err(|err| {
                    DefinitionError::Access(format!("Cannot open [File: {path}] [Error: {err}]"))
                })
                .and_then(|file| {
                    serde_json::from_reader(BufReader::new(file))
                        .map_err(|err| DefinitionError::Parse(err.to_string()))
                }),
            DefinitionInput::Text(content) => serde_json::from_str(&content).map_err(|err| {
                DefinitionError::Parse(format!("Cannot parse text definition {err}"))
            }),
            DefinitionInput::Processor(processor) => processor.output_stdout().and_then(|stdout| {
                dbg!(&stdout);
                serde_json::from_str(&stdout).map_err(|err| {
                    DefinitionError::Parse(format!("Cannot parse [Value:{stdout}] [Error: {err}]"))
                })
            }),
            DefinitionInput::Value(value) => serde_json::from_value(value.clone()).map_err(|err| {
                DefinitionError::Parse(format!("Cannot parse [Value:{value}] [Error: {err}]"))
            }),
        }
        .map(DefinitionDto::map)
    }

    fn enrich(&self, _context: &ProcessorContext) -> Result<Definition, DefinitionError> {
        Err(DefinitionError::Process("".to_string()))
        // if let Some(output) = context.processor.output_stdout() {
        // dbg!(&output);
        // if let Some(install) = &processor.install {
        //     match install.command().output() {
        //         Ok(output) => {
        //             if let Ok(stdout) = String::from_utf8(output.stdout) {
        //                 print!("{stdout}");
        //             }
        //         }
        //         Err(_) => todo!(),
        //     }
        // }
        // if let Ok(serialized_context) = serde_json::to_string(&context.definition) {
        //     let mut command = processor.command();
        //     command.args(vec!["--context".to_string(), serialized_context]);
        //     print!("{:?}", &command.get_args().collect::<Vec<&OsStr>>());
        //     let _ = command.output().inspect_err(|err| print!("{err}"));
        // }
        // }
        // None
    }
}
