use crate::models::{api::fetch_listens, musicbrainz::MBIDType, stats::MBIDStatCounter};

pub fn recording_stats(username: &str) {
    println!("Getting the listens...");
    let listens = fetch_listens(username).unwrap();
    println!("Calculating stats...");
    let mut sorter = MBIDStatCounter::new(MBIDType::Recording);
    sorter.extend(listens.get_mapped_listens());

    for key in sorter.into_sorted() {
        println!(
            "[{}] - {}",
            key.len(),
            key.first()
                .unwrap()
                .get_mapping_data()
                .as_ref()
                .unwrap()
                .get_recording_name()
        )
    }
}

pub fn artist_stats(username: &str) {
    println!("Getting the listens...");
    let listens = fetch_listens(username).unwrap();
    println!("Calculating stats...");

    let mut sorter = MBIDStatCounter::new(MBIDType::Artist);
    sorter.extend(listens.get_mapped_listens());

    for key in sorter.into_sorted() {
        println!(
            "[{}] - {}",
            key.len(),
            key.first()
                .unwrap()
                .get_mapping_data()
                .as_ref()
                .unwrap()
                .get_recording_name()
        )
    }
}
