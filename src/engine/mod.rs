use std::{
    ffi::OsStr,
    thread::{self},
};

use chara::{Chara, EnricherContext};
use cli::Cli;

pub mod chara;
mod chara_test;
pub mod cli;
mod contexts_dto;

pub fn run(chara: Chara) {
    chara
        .enrichers_contexts()
        .into_iter()
        .map(|context| {
            thread::spawn(move || {
                handle_enricher(context);
            })
        })
        .for_each(|handler| handler.join().unwrap());
}

fn handle_enricher(context: EnricherContext) {
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
        if let Ok(serialized_context) = serde_json::to_string(&context.chara) {
            let mut command = enricher.command();
            command.args(vec!["--context".to_string(), serialized_context]);
            print!("{:?}", &command.get_args().collect::<Vec<&OsStr>>());
            let _ = command.output().inspect_err(|err| print!("{err}"));
        }
    }
}
