use std::{
    process::Command,
    thread::{self, JoinHandle},
};

use bootes::{Bootes, Metadata};
use contexts::{EdgeContext, ScrapperContext};

use crate::types::thread::Readonly;

pub mod bootes;
pub mod contexts;

pub fn run(bootes: Bootes) {
    bootes
        .metadatas
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
            if let Some(scrapper) = edge_lock.scrapper.clone() {
                handle_scrapper(ScrapperContext {
                    edge: context.edge.clone(),
                    metadata: context.metadata.clone(),
                    scrapper,
                })
            }
        }
    })
}

fn handle_scrapper(context: ScrapperContext) {
    if let Ok(scrapper_lock) = context.scrapper.read() {
        if let Some(install) = &scrapper_lock.install {
            match Command::new(&install.path)
                .args(&install.arguments)
                .output()
            {
                Ok(output) => {
                    if let Ok(stdout) = String::from_utf8(output.stdout) {
                        print!("{stdout}");
                    }
                }
                Err(_) => todo!(),
            }
        }
    }
}
