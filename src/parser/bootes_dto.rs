use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::Deserialize;
use serde_json::{de::Read, Map, Value};

use crate::{
    engine::bootes::{Arguments, Bootes, Edge, Metadata, Scrapper, Tag},
    types::thread::{readonly, Readonly},
};
#[derive(Debug, Deserialize, Clone)]
pub struct TagDto {
    label: Option<String>,
    #[serde(default)]
    tags: HashMap<String, TagDto>,
    #[serde(flatten)]
    other: Map<String, Value>,
}
#[derive(Debug, Deserialize)]
pub struct MetadataDto {
    #[serde(default)]
    pub edges: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    other: Map<String, Value>,
}

#[derive(Debug, Deserialize)]
pub struct EdgeDto {
    definition: Option<String>,
    scrapper: Option<String>,
    #[serde(flatten)]
    other: Map<String, Value>,
}

#[derive(Debug, Deserialize)]
struct ScrapperDto {
    #[serde(default)]
    use_context: bool,
    #[serde(default)]
    arguments: Vec<String>,
    path: String,
    install: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BootesDto {
    name: String,
    #[serde(default)]
    metadatas: HashMap<String, MetadataDto>,
    #[serde(default)]
    edges: HashMap<String, EdgeDto>,
    #[serde(default)]
    tags: HashMap<String, TagDto>,
    #[serde(default)]
    scrappers: HashMap<String, ScrapperDto>,
    #[serde(default)]
    arguments: HashMap<String, Vec<String>>,
}

impl BootesDto {
    fn arguments(&self) -> HashMap<String, Readonly<Vec<String>>> {
        self.arguments
            .iter()
            .map(|(key, value)| (key.clone(), readonly(value.clone())))
            .collect()
    }
    pub fn map(self) -> Bootes {
        let arguments = self.arguments();
        let mut bootes = Bootes {
            name: self.name.clone(),
            arguments,
            edges: HashMap::new(),
            metadatas: HashMap::new(),
            scrappers: HashMap::new(),
            tags: HashMap::new(),
        };
        self.set_scrappers(&mut bootes);
        self.set_edges(&mut bootes);
        self.set_tags(&mut bootes);
        self.set_metadatas(&mut bootes);
        bootes
    }

    fn set_tags(&self, bootes: &mut Bootes) {
        let tags = extract_tags(&"#".to_string(), &self.tags);
        bootes.tags = tags.into_iter().collect();
    }

    fn set_scrappers(&self, bootes: &mut Bootes) {
        bootes.scrappers = self
            .scrappers
            .iter()
            .map(|(key, scrapper)| {
                (
                    key.clone(),
                    readonly(Scrapper {
                        use_context: scrapper.use_context,
                        arguments: map_scrapper_arguments(scrapper, bootes),
                        path: scrapper.path.clone(),
                        install: scrapper.install.clone(),
                    }),
                )
            })
            .collect()
    }

    fn set_edges(&self, bootes: &mut Bootes) {
        bootes.edges = self
            .edges
            .iter()
            .map(|(key, edge)| {
                (
                    key.clone(),
                    readonly(Edge {
                        definition: edge.definition.clone(),
                        scrapper: edge.scrapper.as_ref().and_then(|path| {
                            bootes.scrappers.get(path.trim_start_matches("#/")).cloned()
                        }),
                        other: edge.other.clone(),
                    }),
                )
            })
            .collect()
    }
    fn set_metadatas(&self, bootes: &mut Bootes) {
        bootes.metadatas = self
            .metadatas
            .iter()
            .map(|(key, metadata)| {
                (
                    key.clone(),
                    readonly(Metadata {
                        edges: metadata
                            .edges
                            .iter()
                            .map(|path| {
                                bootes
                                    .edges
                                    .get(path.trim_start_matches("#/"))
                                    .cloned()
                                    .map(|edge| (path.clone(), edge))
                            })
                            .flatten()
                            .collect(),
                        tags: HashMap::new(),
                        other: metadata.other.clone(),
                    }),
                )
            })
            .collect()
    }
}

fn map_scrapper_arguments(
    scrapper: &ScrapperDto,
    bootes: &Bootes,
) -> Vec<Arc<RwLock<Vec<String>>>> {
    scrapper
        .arguments
        .iter()
        .map(|path| {
            bootes
                .arguments
                .get(path.trim_start_matches("#/"))
                .map(|v| v.clone())
        })
        .flatten()
        .collect()
}
fn extract_tags(
    parent_path: &String,
    tags: &HashMap<String, TagDto>,
) -> Vec<(String, Readonly<Tag>)> {
    tags.iter()
        .map(|(k, tag)| {
            let path = parent_path.to_owned() + "/" + k;
            if tag.tags.is_empty() {
                vec![(
                    path,
                    readonly(Tag {
                        label: tag.label.clone(),
                        tags: HashMap::new(),
                        other: tag.other.clone(),
                    }),
                )]
            } else {
                extract_tags(&path, &tag.tags)
            }
        })
        .flatten()
        .collect()
}
pub trait StringExt {
    fn trim_path(val: &String) -> &str {
        val.trim_matches('/')
    }
}
impl StringExt for String {}
