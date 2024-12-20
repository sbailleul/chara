use std::{collections::HashMap, sync::Arc};

use common::{
    thread::{readonly, Readonly},
    ThreadError,
};
use engine::{
     definition::tag::{RefTag, Tag}, errors::CharaError
};

use crate::definition::TagDto;

pub fn to_tags(
    parent: &Readonly<RefTag>,
    parent_path: &String,
    tags: &HashMap<String, TagDto>,
) -> Vec<(String, String, Readonly<RefTag>, Readonly<RefTag>)> {
    tags.iter()
        .map(|(k, tag_dto)| {
            let path = parent_path.to_owned() + "/" + k;
            if tag_dto.tags.is_empty() {
                vec![(
                    k.clone(),
                    path.clone(),
                    parent.clone(),
                    readonly(RefTag {
                        r#ref: k.clone(),
                        value: Tag {
                            label: tag_dto.label.clone(),
                            tags: HashMap::new(),
                            other: tag_dto.other.clone(),
                        },
                    }),
                )]
            } else {
                let tag = readonly(RefTag {
                    r#ref: k.clone(),
                    value: Tag {
                        label: tag_dto.label.clone(),
                        tags: HashMap::new(),
                        other: tag_dto.other.clone(),
                    },
                });

                let mut inner_tags = to_tags(&tag, &path, &tag_dto.tags);
                if let Ok(mut tag_value) = tag.write() {
                    tag_value.value.tags = inner_tags
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

pub fn from_tags(tags: &HashMap<String, Readonly<RefTag>>) -> HashMap<String, TagDto> {
    tags.iter()
        .map(|(_, tag)| {
            let tag = tag
                .read()
                .map_err(|_| CharaError::Thread(ThreadError::Poison))?;
            Ok::<(String, TagDto), CharaError>((
                tag.r#ref.clone(),
                TagDto {
                    tags: from_tags(&tag.value.tags),
                    label: tag.value.label.clone(),
                    other: tag.value.other.clone(),
                },
            ))
        })
        .flatten()
        .collect()
}
