use std::sync::Arc;
use itertools::Itertools;
use crate::core::entity_traits::fetchable::FetchableAndCachable;
use crate::core::entity_traits::has_id::HasID;
use crate::core::entity_traits::mbid::VecIExt;
use crate::models::data::musicbrainz::work::Work;

pub async fn search_link() {
    let start = "1919e988-9619-45fc-a2dc-91dbf52a85c2";
    let end = "c768f5dc-7ebb-434d-89c0-3473224af906";
    
    let star_node = Node::new(Work::get_cached_or_fetch(start).await.unwrap(), None);
    let mut node_to_searche = vec![Arc::new(star_node)];
    
    loop {
        let current_node = node_to_searche.pop().unwrap();
        
        if current_node.current.get_id().as_str() == end {
            let mut curr = current_node.clone();
            loop {
                println!("{:?} - {}", curr.current.get_id(), curr.current.title());
                curr = curr.previous.as_ref().unwrap().clone();
            }
        }
        
        let next_nodes = current_node.current.get_parent_works_ids().await.unwrap().get_or_fetch_entities().await.unwrap().into_iter().map(|work| Arc::new(Node::new(work, Some(current_node.clone())))).collect_vec();
        
        node_to_searche.extend(next_nodes);
    }
}

pub struct Node {
    pub current: Work,
    pub previous: Option<Arc<Self>>
}

impl Node {
    pub fn new(work: Work, previous: Option<Arc<Self>>) -> Self {
        Self {
            current: work,
            previous
        }
    }
}