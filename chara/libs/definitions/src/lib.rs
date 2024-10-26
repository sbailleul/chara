use std::{ffi::OsStr, fs::File, io::BufReader};

use cli::Cli;
use definition::DefinitionDto;
use engine::{
    definition::{Definition, EnricherContext},
    DefinitionInput, Definitions as ForeignDefinitions,
};
mod cli;
mod definition;
mod map;
pub struct Definitions {}
impl ForeignDefinitions for Definitions {
    fn get(&self, definition: &DefinitionInput) -> Definition {
        let result = match definition {
            DefinitionInput::File(path) => {
                let file =
                    File::open(path).expect(format!("File {} isn't readable", path).as_str());
                let reader = BufReader::new(file);
                serde_json::from_reader(reader)
            }
            DefinitionInput::Text(content) => serde_json::from_str(&content),
        };
        let definition: DefinitionDto =
            result.expect(format!("Format {:?} isn't valid", definition).as_str());
        dbg!(&definition);
        definition.map()
    }

    fn enrich(&self, context: &EnricherContext) -> Option<Definition> {
        if let Ok(enricher) = context.enricher.read() {
            if let Some(install) = &enricher.install {
                match install.command().output() {
                    Ok(output) => {
                        if let Ok(stdout) = String::from_utf8(output.stdout) {
                            print!("{stdout}");
                        }
                    }
                    Err(_) => todo!(),
                }
            }
            if let Ok(serialized_context) = serde_json::to_string(&context.definition) {
                let mut command = enricher.command();
                command.args(vec!["--context".to_string(), serialized_context]);
                print!("{:?}", &command.get_args().collect::<Vec<&OsStr>>());
                let _ = command.output().inspect_err(|err| print!("{err}"));
            }
        }
        None
    }
}
