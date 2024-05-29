//! This module contain all the prompts we could have about a recording.
//!
//! It's mostly full of "is_QUESTION(&self) -> bool" methods

use std::cell::Ref;

use futures::prelude::stream;
use futures::prelude::stream::StreamExt;
use regex::Regex;
use tokio_stream::StreamExt;

use crate::core::entity_traits::mbid::IsMbid;

use super::Relation;
use super::RelationTarget;
use extend::ext;

impl Relation {
    pub async fn is_streamable_on(&self, domain: &str) -> color_eyre::Result<bool> {
        match self.content {
            RelationTarget::Url(url) => {
                let url = url.get_or_fetch_entity().await?.resource();

                Ok(get_domain_from_url(&url) == Some(domain))
            }
            _ => Ok(false),
        }
    }
}

fn get_domain_from_url(url: &str) -> Option<&str> {
    let regex = Regex::new(r"(?im)^(?:https?://)?(?:[^@/\n]+@)?(?:www\.)?([^:/\n]+)").unwrap();
    if let Some(captures) = regex.captures(&url) {
        return captures.get(1).map(|mat| mat.as_str());
    }

    None
}

#[ext]
pub impl Vec<Relation> {
    async fn is_streamable_on(&self, domain: &str) -> color_eyre::Result<bool> {
        for rel in self {
            if rel.is_streamable_on(domain).await? {
                return Ok(true);
            }
        }

        return Ok(false);
    }
}
