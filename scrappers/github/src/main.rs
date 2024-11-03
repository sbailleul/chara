use std::path::Path;

use clap::{command, Parser};
use definitions::{DefinitionContextDto, WritePermissionsDto};
use log::error;
use serde::Deserialize;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    context: String,
}
#[derive(Debug, Error)]
enum Error {
    #[error("Parse error {0}")]
    Parse(serde_json::Error),
    #[error("Context should specify original definition location")]
    MissingLocation
}
#[derive(Debug, Deserialize)]
struct Metadata {
    file: String,
}
#[derive(Debug, Deserialize)]
struct Edge {}
#[derive(Debug)]
struct DefinitionContext {
    location: String,
    metadata: Metadata,
    edge: Option<Edge>,
    write: WritePermissionsDto,
}
impl TryFrom<DefinitionContextDto> for DefinitionContext {
    type Error = Error;

    fn try_from(value: DefinitionContextDto) -> Result<Self, Self::Error> {
        match value.location {
            Some(location) =>  match serde_json::from_value(value.metadata.value) {
                Ok(metadata) => match value.edge {
                    Some(edge) => match serde_json::from_value(edge.value) {
                        Ok(edge) => Ok(Self {
                            location,
                            metadata,
                            edge: Some(edge),
                            write: value.write,
                        }),
                        Err(err) => Err(Error::Parse(err)),
                    },
                    None => Ok(Self {
                        location,
                        metadata,
                        edge: None,
                        write: value.write,
                    }),
                },
                Err(err) => Err(Error::Parse(err)),
            }
            None => Err(Error::MissingLocation)
        }
       
    }
}
/*
"{\"\" \"write\":{\"metadata\":false,\"edge\":true},\"metadata\":{\"name\":\"quality\",\"value\":{\"file\":\"./.github/workflows/quality-workflow.yaml\"}},\"edge\":{\"name\":\"#/workflows\",\"value\":{}}}"
*/
#[tokio::main]
async fn main() -> Result<(), Error> {
    colog::init();
    let args = Args::parse();
    serde_json::from_str::<DefinitionContextDto>(&args.context)
        .map_err(Error::Parse)
        .and_then(DefinitionContext::try_from)
        .map(|context| {
            handle_context(context);        
        })
        .inspect_err(|err| error!("{err}"))
}

fn handle_context(context: DefinitionContext){
    Path::new(&context.location);
}
