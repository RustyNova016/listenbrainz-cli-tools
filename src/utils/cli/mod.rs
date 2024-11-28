pub mod global_progress_bar;
use std::io;

use super::regex::is_string_mbid;
use crate::utils::regex::get_raw_mbid_from_url;

/// Block the current trhead until the user press enter
pub fn await_next() {
    let buf = &mut String::new();
    let _ = io::stdin().read_line(buf);
}

pub fn read_mbid_from_input(input: &str) -> Option<String> {
    if is_string_mbid(input) {
        return Some(input.to_string());
    }

    get_raw_mbid_from_url(input)
}
