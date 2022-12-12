use serde::{Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Song {
    pub channel: String,
    pub description: String,
    pub thumbnail: String,
    pub fulltitle: String,
    #[serde(skip)]
    pub filename: String,
    #[serde(default, rename = "track")]
    pub title: String,
    #[serde(default)]
    pub album: String,
    #[serde(default)]
    pub artist: String,
    #[serde(default)]
    pub release_year: Option<i32>,
    #[serde(skip)]
    pub genre: String,
    #[serde(skip)]
    pub track_no: Option<u32>,
    #[serde(skip)]
    pub total_tracks: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Playlist {
    pub title: String,
    pub channel: String,
    pub entries: Vec<Value>,
}