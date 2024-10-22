use std::thread::{self, JoinHandle};

use bootes::{Bootes, Metadata};
use cli::Cli;
use contexts::{EdgeContext, EdgeEnricherContext};
use contexts_dto::EdgeEnricherContextDto;

use crate::types::thread::Readonly;

pub mod bootes;
pub mod cli;
mod contexts;
mod contexts_dto;
pub fn run(bootes: Bootes) {
    bootes
        .metadata
        .into_iter()
        .map(|metadata| handle_metadata(metadata))
        .for_each(|handle| {
            handle.join().unwrap();
        });
}
fn handle_metadata(metadata: (String, Readonly<Metadata>)) -> JoinHandle<()> {
    thread::spawn(move || {
        let _ = metadata.1.read().map(|metadata_lock| {
            metadata_lock
                .edges
                .iter()
                .map(|(key, edge)| {
                    handle_edge(EdgeContext {
                        metadata: metadata.clone(),
                        edge: (key.clone(), edge.clone()),
                    })
                })
                .for_each(|handle| handle.join().unwrap())
        });
    })
}
fn handle_edge(context: EdgeContext) -> JoinHandle<()> {
    thread::spawn(move || {
        if let Ok(edge_lock) = context.edge.1.read() {
            if let Some(enricher) = edge_lock.enricher.clone() {
                handle_enricher(EdgeEnricherContext {
                    edge: context.edge.clone(),
                    metadata: context.metadata.clone(),
                    enricher,
                })
            }
        }
    })
}

fn handle_enricher(context: EdgeEnricherContext) {
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
        if let Some(serialized_context) =
            EdgeEnricherContextDto::from(&context).and_then(|context| context.serialize().ok())
        {
            let _ = enricher
                .command()
                .args(vec!["--context".to_string(), serialized_context])
                .output()
                .inspect_err(|err| print!("{err}"));
        }
    }
}
