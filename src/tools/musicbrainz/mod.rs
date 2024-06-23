use std::collections::VecDeque;
use std::sync::Arc;

use itertools::Itertools;
use tokio::sync::{RwLock, Semaphore};
use tokio::task::JoinSet;

use crate::core::entity_traits::mbid::IsMbid;
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;

pub async fn search_link() {
    //let start: RecordingMBID = "5fed738b-1e5c-4a1b-9f66-b3fd15dbc8ef".to_string().into();
    //let end: RecordingMBID = "8bf05ed3-2267-46cd-b297-e72625f60cc9".to_string().into();

    let start: WorkMBID = "1919e988-9619-45fc-a2dc-91dbf52a85c2".to_string().into();
    let end: RecordingMBID = "8bf05ed3-2267-46cd-b297-e72625f60cc9".to_string().into();

    let finder = LinkFinder::new(
        Node::new(start.into_mbid(), None),
        Node::new(end.into_mbid(), None),
    );
    Arc::new(finder).main().await;
}

pub struct LinkFinder {
    start_pool: Arc<RwLock<VecDeque<Arc<Node>>>>,
    end_pool: Arc<RwLock<VecDeque<Arc<Node>>>>,

    start_spent_nodes: Arc<RwLock<Vec<Arc<Node>>>>,
    end_spent_nodes: Arc<RwLock<Vec<Arc<Node>>>>,
}

impl LinkFinder {
    pub fn new(start_node: Node, end_node: Node) -> Self {
        let mut start_node_pool = VecDeque::new();
        start_node_pool.push_back(Arc::new(start_node));

        let mut end_node_pool = VecDeque::new();
        end_node_pool.push_back(Arc::new(end_node));

        Self {
            start_pool: Arc::new(RwLock::new(start_node_pool)),
            end_pool: Arc::new(RwLock::new(end_node_pool)),
            start_spent_nodes: Arc::new(RwLock::new(Vec::new())),
            end_spent_nodes: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn main(self: Arc<Self>) {
        let mut num_iter = 0;
        let concurents = Arc::new(Semaphore::new(50));
        let mut set = JoinSet::new();

        loop {
            if concurents.available_permits() == 0 {
                set.join_next().await;
                println!("Task done")
            }

            num_iter += 1;
            let this = self.clone();
            let sema = concurents.clone();
            set.spawn(async move {
                let permit = sema.acquire().await.unwrap();
                this.iter_once(num_iter).await;
                drop(permit)
            });
        }
    }

    fn get_pool(&self, num_iter: u64) -> Arc<RwLock<VecDeque<Arc<Node>>>> {
        if num_iter % 2 == 0 {
            return self.start_pool.clone();
        } else {
            return self.end_pool.clone();
        }
    }

    fn get_pool_invert(&self, num_iter: u64) -> Arc<RwLock<VecDeque<Arc<Node>>>> {
        if num_iter % 2 != 0 {
            return self.start_pool.clone();
        } else {
            return self.end_pool.clone();
        }
    }

    fn get_spent_pool(&self, num_iter: u64) -> Arc<RwLock<Vec<Arc<Node>>>> {
        if num_iter % 2 == 0 {
            return self.start_spent_nodes.clone();
        } else {
            return self.end_spent_nodes.clone();
        }
    }

    fn get_spent_pool_invert(&self, num_iter: u64) -> Arc<RwLock<Vec<Arc<Node>>>> {
        if num_iter % 2 != 0 {
            return self.start_spent_nodes.clone();
        } else {
            return self.end_spent_nodes.clone();
        }
    }

    async fn find_node_id_in_other_pool(&self, id: &MBID, num_iter: u64) -> Option<Arc<Node>> {
        let pool = self.get_pool_invert(num_iter).clone();
        let other_pool_read = pool.read().await;
        other_pool_read
            .iter()
            .find(|node| &node.current == id)
            .cloned()
    }

    async fn find_node_id_in_other_spent_pool(
        &self,
        id: &MBID,
        num_iter: u64,
    ) -> Option<Arc<Node>> {
        let pool = self.get_spent_pool_invert(num_iter).clone();
        let other_spent_pool_read = pool.read().await;
        other_spent_pool_read
            .iter()
            .find(|node| &node.current == id)
            .cloned()
    }

    async fn check_win(&self, check_node: Arc<Node>, num_iter: u64) -> bool {
        let node_in_other_pending = self
            .find_node_id_in_other_pool(&check_node.current, num_iter)
            .await;
        let node_spent = self
            .find_node_id_in_other_spent_pool(&check_node.current, num_iter)
            .await;

        let node_everywhere = node_in_other_pending.or(node_spent);

        if node_everywhere.is_some() {
            println!();
            println!();
            println!();
            println!("WIN!");
            println!();
            println!();
            println!();

            let final_path = check_node.merge(node_everywhere.unwrap().as_ref());

            println!("Layer count: {}", final_path.layer_count());

            println!("{:#?}", final_path);

            todo!()
        }

        return false;
    }

    pub async fn iter_once(self: Arc<Self>, num_iter: u64) {
        let pool = self.get_pool(num_iter);
        let Some(current_node) = pool.write().await.pop_front() else {
            println!("No more nodes. Canceling");
            todo!();
            return;
        };

        if self
            .get_spent_pool(num_iter)
            .read()
            .await
            .contains(&current_node)
        {
            return;
        }

        println!(
            "Current iteration: {} ({}) - Layer {} ",
            num_iter,
            current_node.current,
            current_node.layer_count()
        );

        if self.check_win(current_node.clone(), num_iter).await {
            let mut curr = current_node.clone();
            println!("Found!")
            //loop {
            //    println!("{:?} - {}", curr.current, curr.current.title());
            //    curr = curr.previous.as_ref().unwrap().clone();
            //}
        }

        let childrens = Node::get_childrens(current_node.clone()).await;

        let mut pool_write = pool.write().await;
        for child in childrens {
            pool_write.push_back(Arc::new(child));
        }
        drop(pool_write);

        self.get_spent_pool(num_iter)
            .write()
            .await
            .push(current_node);
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
            MBID::Artist(_) => {
                Self::get_artist_childrens(this.current.clone().unwrap_artist()).await
            }
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

    pub async fn get_artist_childrens(mbid: ArtistMBID) -> Vec<MBID> {
        let mut ids = Vec::new();
        let entity = mbid.get_or_fetch_entity().await.unwrap();

        for relation in entity.get_or_fetch_relations_misc().await.unwrap() {
            if let Some(id) = relation.content().clone().into_mbid_safe() {
                ids.push(id.into_mbid())
            }
        }

        ids
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

        for release_id in recording.get_or_fetch_releases_ids().await.unwrap() {
            ids.push(release_id.into_mbid())
        }

        for relation in recording.get_or_fetch_relations_misc().await.unwrap() {
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
                .get_or_fetch_release_group_mbid()
                .await
                .unwrap()
                .into_mbid(),
        );

        for recording_id in release.get_or_fetch_recording_ids().await.unwrap() {
            ids.push(recording_id.into_mbid());
        }

        for relation in release.get_or_fetch_relations_misc().await.unwrap() {
            if let Some(id) = relation.content().clone().into_mbid_safe() {
                ids.push(id.into_mbid())
            }
        }

        ids
    }

    pub async fn get_work_childrens(mbid: WorkMBID) -> Vec<MBID> {
        let mut ids = Vec::new();
        let entity = mbid.get_or_fetch_entity().await.unwrap();

        for relation in entity.get_or_fetch_relations_misc().await.unwrap() {
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

    pub fn merge(self: Arc<Self>, other: &Self) -> Self {
        if self.current == other.current {
            if let Some(prev) = other.previous.as_ref() {
                return self.clone().merge(prev.as_ref());
            } else {
                return self.as_ref().clone();
            }
        }

        let next = Self {
            current: other.current.clone(),
            previous: Some(self.clone()),
        };

        return Arc::new(next).merge(other);
    }
}
