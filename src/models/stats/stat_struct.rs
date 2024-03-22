use std::{any::Any, rc::Rc};

use musicbrainz_rs::entity::{artist, recording};

use crate::models::{cli::stats::GroupByTarget, data::listens::UserListen, musicbrainz::MBIDType};

pub struct EntityStats {
    mbid: String,
    entity_type: GroupByTarget,
    listens: Vec<Rc<UserListen>>,
}

impl EntityStats {
    pub fn push(&mut self, value: Rc<UserListen>) {
        match self.entity_type {
            GroupByTarget::Recording => self.push_recording(value),
            GroupByTarget::Artist => self.push_artist(value),
        }
    }

    fn push_recording(&mut self, value: Rc<UserListen>) {
        if value.is_mapped_to_recording(&self.mbid) {
            self.listens.push(value)
        }
    }

    fn push_artist(&mut self, value: Rc<UserListen>) {
        let Some(recording) = value.get_recording_data() else {
            return;
        };

        if recording
            .artist_credit
            .is_some_and(|credit| credit.iter().any(|artist| artist.artist.id == self.mbid))
        {
            self.listens.push(value)
        }
    }

    pub fn get_mbid(&self) -> &str {
        &self.mbid
    }

    pub fn new(mbid: String, entity_type: GroupByTarget) -> Self {
        Self {
            mbid,
            entity_type,
            listens: Vec::new(),
        }
    }
}

pub trait StatStruct {
    fn push(&mut self, value: Rc<UserListen>);

    fn get_mbid(&self) -> &str;

    fn new(mbid: String) -> Self;
}
