use std::{collections::HashMap, sync::Arc};

use engine::{definition::Tag, errors::CharaError};
use types::{thread::{readonly, Readonly}, ThreadError};

use crate::definition::TagDto;

pub fn to_tags(
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
                    path.clone(),
                    parent.clone(),
                    readonly(Tag {
                        reference: k.clone(),
                        label: tag_dto.label.clone(),
                        tags: HashMap::new(),
                        other: tag_dto.other.clone(),
                    }),
                )]
            } else {
                let tag = readonly(Tag {
                    reference: k.clone(),
                    label: tag_dto.label.clone(),
                    tags: HashMap::new(),
                    other: tag_dto.other.clone(),
                });

                let mut inner_tags = to_tags(&tag, &path, &tag_dto.tags);
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



pub fn from_tags(tags: &HashMap<String, Readonly<Tag>>) -> HashMap<String, TagDto> {
    tags.iter()
        .map(|(_, tag)| {
            let tag = tag
                .read()
                .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
            Ok::<(String, TagDto), CharaError>((
                tag.reference.clone(),
                TagDto {
                    tags: from_tags(&tag.tags),
                    label: tag.label.clone(),
                    other: tag.other.clone(),
                },
            ))
        })
        .flatten()
        .collect()
}