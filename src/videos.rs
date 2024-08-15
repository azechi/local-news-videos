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
struct Video {
    content_details: ContentDetails,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ContentDetails {
    duration: String,
}

pub async fn get_duration(api: &impl Fn(&str, &[(&str, &str)]) -> Request, video_ids: &str) -> Vec<String> {
    let dat = fetch::<VideosListResult>(api(
        "videos",
        &[
            ("part", "contentDetails"),
            ("id", video_ids), 
        ],
    ))
    .await;

    dat.items
        .into_iter()
        .map(|x| x.content_details.duration)
        .collect()
}
