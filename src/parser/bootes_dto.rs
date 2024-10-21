use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::Deserialize;
use serde_json::{Map, Value};

use crate::{
    engine::bootes::{Bootes, Edge, Install, Metadata, Scrapper, Tag},
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
#[derive(Debug, Deserialize, Clone)]
struct InstallDto {
    path: String,
    #[serde(default)]
    arguments: Vec<String>,
}
#[derive(Debug, Deserialize)]
struct ScrapperDto {
    #[serde(default)]
    use_context: bool,
    #[serde(default)]
    arguments: Vec<String>,
    path: String,
    install: Option<InstallDto>,
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
        let tags = extract_tags(
            &readonly(Tag {
                label: None,
                tags: HashMap::new(),
                other: Map::new(),
            }),
            &"#".to_string(),
            &self.tags,
        );
        bootes.tags = tags
            .into_iter()
            .map(|(_key, path, _parent_tag, tag)| (path, tag))
            .collect();
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
                        install: scrapper.install.as_ref().map(|install| Install {
                            arguments: install.arguments.clone(),
                            path: install.path.clone(),
                        }),
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
                        tags: metadata
                            .tags
                            .iter()
                            .map(|tag| {
                                bootes
                                    .tags
                                    .get(tag)
                                    .map(|found_tag| (tag.clone(), found_tag.clone()))
                            })
                            .flatten()
                            .collect(),
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
    parent: &Readonly<Tag>,
    parent_path: &String,
    tags: &HashMap<String, TagDto>,
) -> Vec<(String, String, Readonly<Tag>, Readonly<Tag>)> {
    tags.iter()
        .map(|(k, tag_dto)| {
            let path = parent_path.to_owned() + "/" + k;
            if tag_dto.tags.is_empty() {
                vec![(
                    k.clone(),
                    path,
                    parent.clone(),
                    readonly(Tag {
                        label: tag_dto.label.clone(),
                        tags: HashMap::new(),
                        other: tag_dto.other.clone(),
                    }),
                )]
            } else {
                let tag = readonly(Tag {
                    label: tag_dto.label.clone(),
                    tags: HashMap::new(),
                    other: tag_dto.other.clone(),
                });

                let mut inner_tags = extract_tags(&tag, &path, &tag_dto.tags);
                if let Ok(mut tag_value) = tag.write() {
                    tag_value.tags = inner_tags
                        .iter()
                        .filter(|(_k, _parent_path, parent_tag, _inner_tag)| {
                            Arc::ptr_eq(&parent_tag, &tag)
                        })
                        .map(|(k, _parent_path, _parent_tag, inner_tag)| {
                            (k.clone(), inner_tag.clone())
                        })
                        .collect()
                }
                inner_tags.push((k.clone(), path, parent.clone(), tag));
                inner_tags
            }
        })
        .flatten()
        .collect()
}
