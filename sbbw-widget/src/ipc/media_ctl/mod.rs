use std::time::Duration;

use mpris::LoopStatus;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "linux")]
mod linux;

#[derive(Default, Debug, Serialize, Deserialize)]
struct SbbwMediaMetadata {
    pub track_id: String,
    pub title: String,
    pub album_name: String,
    pub album_artists: Vec<String>,
    pub artists: Vec<String>,
    pub art_url: Option<String>,
    // microseconds
    pub track_length: Option<u64>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct SbbwMediaState {
    pub id: String,
    pub player_name: String,
    pub metadata: Option<SbbwMediaMetadata>,
    pub volume: Option<f64>,
    pub track_progress: Option<u64>,
    pub shuffle: bool,
}

pub mod prelude {
    #[cfg(target_os = "linux")]
    pub use super::linux::*;
}
