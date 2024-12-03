use crate::models::data::musicbrainz::work::mbid::WorkMBID;
use crate::models::data::musicbrainz::work::Work;
use itertools::Itertools;
use std::sync::Arc;

pub async fn search_link() {
    let start: WorkMBID = "1919e988-9619-45fc-a2dc-91dbf52a85c2".to_string().into();
    let end: WorkMBID = "c768f5dc-7ebb-434d-89c0-3473224af906".to_string().into();

    let star_node = Node::new(Work::get_cached_or_fetch(&start).await.unwrap(), None);
    let mut node_to_searche = vec![Arc::new(star_node)];

    loop {
        let current_node = node_to_searche.pop().unwrap();

        if current_node.current.get_mbid() == end {
            let mut curr = current_node.clone();
            loop {
                println!("{:?} - {}", curr.current.get_id(), curr.current.title());
                curr = curr.previous.as_ref().unwrap().clone();
            }
        }

        let next_nodes = current_node
            .current
            .get_parent_works_ids()
            .await
            .unwrap()
            .get_or_fetch_entities()
            .await
            .unwrap()
            .into_iter()
            .map(|work| Arc::new(Node::new(work, Some(current_node.clone()))))
            .collect_vec();

        node_to_searche.extend(next_nodes);
    }
}

pub struct Node {
    pub current: Work,
    pub previous: Option<Arc<Self>>,
}

impl Node {
    pub fn new(work: Work, previous: Option<Arc<Self>>) -> Self {
        Self {
            current: work,
            previous,
        }
    }
}
