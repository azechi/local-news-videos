use crate::fetch::fetch;
use serde::Deserialize;
use worker::Request;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PlayListItems {
    items: Vec<PlayListItem>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PlayListItem {
    content_details: ContentDetails,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ContentDetails {
    video_id: String,
}

pub async fn get_video_ids(api: &impl Fn(&str, &[(&str, &str)]) -> Request, playlist_id: &str) -> Vec<String> {
    let dat = fetch::<PlayListItems>(api(
        "playlistItems",
        &[
            ("part", "contentDetails,snippet,id,status"),
            ("playlistId", playlist_id),
            ("maxResults", "15")
        ],
    ))
    .await;

    dat.items
        .into_iter()
        .map(|x| x.content_details.video_id)
        .collect()
}
