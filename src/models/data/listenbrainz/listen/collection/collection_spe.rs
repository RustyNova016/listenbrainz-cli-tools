use std::sync::Arc;

use extend::ext;

use crate::models::data::listenbrainz::listen::listen_spe::ListenSpe;
use crate::models::data::listenbrainz::listen::listen_spe::Mapped;
use crate::models::data::listenbrainz::listen::listen_spe::MappingState;



#[ext]
pub impl<S> Vec<Arc<ListenSpe<S>>> where S: MappingState {

}