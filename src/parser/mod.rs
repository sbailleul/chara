use std::{fs::File, io::BufReader};

use bootes_dto::{BootesDto, EdgeDto};
use finders::{find_all, find_by_path};

use crate::engine::bootes::Bootes;

mod bootes_dto;
mod finders;

pub fn parse(file_name: &str) {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let bootes: BootesDto = serde_json::from_reader(reader).expect("Serialization error");
    let bootes = bootes.map();
    dbg!(bootes);
    // if let Some(metadata) = bootes.metadatas {
    //     dbg!(&metadatas);
    //     if let Some(mut edges) = bootes.edges {
    //         for metadata in metadatas {
    //             for edge in metadata.edges {
    //                 let edge = find_by_path::<EdgeDto>(&mut edges, &edge);
    //                 dbg!(&edge);
    //             }
    //         }
    //     }
    // }
}
