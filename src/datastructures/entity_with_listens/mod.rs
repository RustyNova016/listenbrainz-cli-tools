pub mod recording_with_listens;
pub mod release_with_listens;

macro_rules! impl_entity_with_listens {
    ($row_struct: ty) => {
        impl $row_struct {
            /// Return the listen count
            pub fn len(&self) -> usize {
                self.listens.len()
            }

            
        }
    };
}

pub(crate) use impl_entity_with_listens;
