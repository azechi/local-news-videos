use crate::fetch::fetch;
use serde::Deserialize;
use worker::Request;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct VideosListResult {
    items: Vec<Video>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub content_details: ContentDetails,
    pub id: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContentDetails {
    pub duration: String,
}

pub async fn get_duration(api: &impl Fn(&str, &[(&str, &str)]) -> Request, video_ids: &str) -> Vec<Video> {
    fetch::<VideosListResult>(api(
        "videos",
        &[
            ("part", "contentDetails"),
            ("id", video_ids), 
        ],
    ))
    .await
    .items
}
