pub mod cooldown;
pub mod timeouts;
pub mod min_listens;

pub enum RadioFilters {
    MinListens(u64),
    Timeouts,
}

// impl RadioFilters {
//     pub fn filter(self, recordings: impl StreamExt<Item = RecordingWithListens>) -> impl Stream<Item = RecordingWithListens> {
        
//     }
// }