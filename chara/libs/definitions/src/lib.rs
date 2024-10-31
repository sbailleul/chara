use std::{ffi::OsStr, fs::File, io::BufReader};

use cli::Cli;
use definition::DefinitionDto;
use engine::{
    definition::{Definition, DefinitionInput, ProcessorContext},
    Definitions as ForeignDefinitions,
};
mod cli;
pub mod definition;
mod map;
pub struct Definitions {}
impl ForeignDefinitions for Definitions {
    fn get(&self, definition: &DefinitionInput) -> Option<Definition> {
        let definition = match definition {
            DefinitionInput::File(path) => File::open(path)
                .inspect_err(|err| println!("Cannot open [File: {path}] [Error: {err}]"))
                .ok()
                .and_then(|file| {
                    serde_json::from_reader(BufReader::new(file))
                        .inspect_err(|err| {
                            println!("Cannot read json from [File:{path}] [Error: {err}]")
                        })
                        .ok()
                }),
            DefinitionInput::Text(content) => serde_json::from_str(&content)
                .inspect_err(|err| println!("Cannot parse text definition {err}"))
                .ok(),
            DefinitionInput::Processor(processor) => processor.output_stdout().and_then(|stdout| {
                serde_json::from_str(&stdout)
                    .inspect_err(|err| println!("Cannot parse [Value:{stdout}] [Error: {err}]"))
                    .ok()
            }),
            DefinitionInput::Value(value) => serde_json::from_value(value.clone())
                .inspect_err(|err| println!("Cannot parse [Value:{value}] [Error: {err}]"))
                .ok(),
        };
        // dbg!(&result);
        let definition: DefinitionDto = definition.expect("Cannot parse definition");
        Some(definition.map())
    }

    fn enrich(&self, context: &ProcessorContext) -> Option<Definition> {
        if let Some(output) = context.processor.output_stdout() {
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
        }
        None
    }
}
