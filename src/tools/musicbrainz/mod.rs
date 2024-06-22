use std::collections::VecDeque;
use std::sync::Arc;

use itertools::Itertools;

use crate::core::entity_traits::mbid::IsMbid;
use crate::core::entity_traits::relations::has_artist_credits::HasArtistCredits;
use crate::core::entity_traits::relations::has_relationships::HasRelationShips;
use crate::core::entity_traits::relations::has_release_group::HasReleaseGroup;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;

pub async fn search_link() {
    let start: RecordingMBID = "5fed738b-1e5c-4a1b-9f66-b3fd15dbc8ef".to_string().into();
    let end: RecordingMBID = "8bf05ed3-2267-46cd-b297-e72625f60cc9".to_string().into();

    let mut searched_mbids = Vec::new();
    let mut pool = VecDeque::new();
    pool.push_back(Arc::new(Node::new(start.into_mbid(), None)));
    pool.push_back(Arc::new(Node::new(end.clone().into_mbid(), None)));

    let mut current_iter = 0;
    loop {
        let Some(current_node) = pool.pop_front() else {
            println!("No more nodes. Canceling");
            break;
        };

        if searched_mbids.contains(&current_node.current) {
            continue;
        }

        current_iter += 1;
        println!(
            "Current iteration: {} ({}) - Layer {} ",
            current_iter,
            current_node.current,
            current_node.layer_count()
        );

        if current_node.current == end.clone().into_mbid() {
            let mut curr = current_node.clone();
            println!("Found!")
            //loop {
            //    println!("{:?} - {}", curr.current, curr.current.title());
            //    curr = curr.previous.as_ref().unwrap().clone();
            //}
        }

        let childrens = Node::get_childrens(current_node.clone()).await;

        for child in childrens {
            pool.push_back(Arc::new(child));
        }

        searched_mbids.push(current_node.current.clone());
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub current: MBID,
    pub previous: Option<Arc<Self>>,
}

impl Node {
    pub fn new(id: MBID, previous: Option<Arc<Self>>) -> Self {
        Self {
            current: id,
            previous,
        }
    }

    pub async fn get_childrens(this: Arc<Self>) -> Vec<Self> {
        let ids = match this.current {
            MBID::Artist(_) => Vec::new(),
            MBID::Release(_) => {
                Self::get_release_childrens(this.current.clone().unwrap_release()).await
            }
            MBID::Work(_) => Self::get_work_childrens(this.current.clone().unwrap_work()).await,
            MBID::ReleaseGroup(_) => Vec::new(),
            MBID::Recording(_) => {
                Self::get_recording_childrens(this.current.clone().unwrap_recording()).await
            }
        };

        let mut results = Vec::new();
        for id in ids {
            results.push(Self::new(id, Some(this.clone())));
        }

        results
    }

    pub async fn get_recording_childrens(recording_mbid: RecordingMBID) -> Vec<MBID> {
        let mut ids = Vec::new();
        let recording = recording_mbid.get_or_fetch_entity().await.unwrap();

        for artist_id in recording
            .get_or_fetch_artist_credits()
            .await
            .unwrap()
            .get_artist_ids()
        {
            ids.push(artist_id.into_mbid())
        }

        for work_id in recording.get_or_fetch_work_ids().await.unwrap() {
            ids.push(work_id.into_mbid())
        }

        for release_id in recording.get_or_fetch_releases_ids().await.unwrap() {
            ids.push(release_id.into_mbid())
        }

        for relation in recording.get_or_fetch_relationships().await.unwrap() {
            if let Some(id) = relation.content().clone().into_mbid_safe() {
                ids.push(id.into_mbid())
            }
        }

        ids
    }

    pub async fn get_release_childrens(release_mbid: ReleaseMBID) -> Vec<MBID> {
        let mut ids = Vec::new();
        let release = release_mbid.get_or_fetch_entity().await.unwrap();

        for artist_id in release
            .get_or_fetch_artist_credits()
            .await
            .unwrap()
            .get_artist_ids()
        {
            ids.push(artist_id.into_mbid())
        }

        ids.push(
            release
                .get_or_fetch_release_group()
                .await
                .unwrap()
                .into_mbid(),
        );

        for recording_id in release.get_or_fetch_recording_ids().await.unwrap() {
            ids.push(recording_id.into_mbid());
        }

        for relation in release.get_or_fetch_relationships().await.unwrap() {
            if let Some(id) = relation.content().clone().into_mbid_safe() {
                ids.push(id.into_mbid())
            }
        }

        ids
    }

    pub async fn get_work_childrens(mbid: WorkMBID) -> Vec<MBID> {
        let mut ids = Vec::new();
        let entity = mbid.get_or_fetch_entity().await.unwrap();

        for relation in entity.get_or_fetch_relationships().await.unwrap() {
            if let Some(id) = relation.content().clone().into_mbid_safe() {
                ids.push(id.into_mbid())
            }
        }

        ids
    }

    pub fn layer_count(&self) -> u64 {
        if let Some(parent) = &self.previous {
            return parent.layer_count() + 1;
        }

        0
    }
}
