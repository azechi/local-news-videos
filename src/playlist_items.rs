use crate::fetch::fetch;
use chrono::FixedOffset;
use serde::Deserialize;
use worker::Request;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PlayListItems {
    items: Vec<PlayListItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayListItem {
    pub content_details: ContentDetails,
    pub snippet: Snippet,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentDetails {
    pub video_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub published_at: chrono::DateTime<FixedOffset>,
}

/*
fn from_isostring<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<chrono::DateTime<FixedOffset>, D::Error> {
    let s = String::deserialize(deserializer)?;
    Ok(chrono::DateTime::parse_from_rfc3339(&s).unwrap())
}
*/

pub async fn get_playlist_items(api: &impl Fn(&str, &[(&str, &str)]) -> Request, playlist_id: &str) -> Vec<PlayListItem> {
    fetch::<PlayListItems>(api(
        "playlistItems",
        &[
            ("part", "contentDetails,snippet,id,status"),
            ("playlistId", playlist_id),
            ("maxResults", "15")
        ],
    ))
    .await
    .items
}

