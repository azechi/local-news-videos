use serde::{de::DeserializeOwned, Deserialize};
use worker::*;



#[event(fetch)]
async fn fetch(_req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let api_key = env.var("GOOGLE_API_KEY").unwrap().to_string();

    let api = build_api(&api_key);

    let videos = get_video_ids(api).await;

    let dest = Url::parse_with_params(
        "https://www.youtube-nocookie.com/embed",
        &[
            ("autoplay", "1"),
            ("rel", "0"),
            ("playlist", &videos.join(",")), 
        ]
    )?;

    Response::redirect_with_status(dest, 303)
}

async fn fetch<T>(req: Request) -> T 
    where 
        T: DeserializeOwned
{
    let mut res = Fetch::Request(req).send().await.unwrap();
    res.json::<T>().await.unwrap()
}

async fn get_video_ids(api: impl Fn(&str, &[(&str, &str)]) -> Request ) -> Vec<String> {
    
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct ContentDetails {
        video_id: String,
    }
    
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct PlayListItem {
        content_details: ContentDetails,
    }
    
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct PlayListItems {
        items: Vec<PlayListItem>,
    }

    let dat = fetch::<PlayListItems>(
        api(
            "playlistItems",
            &[
                ("part", "contentDetails,snippet,id,status"),
                ("playlistId", "UULFxiRdfyH0FtFCRZTRfRsdsA"),
            ],
        )
    ).await;

    dat.items.into_iter().map(|x| x.content_details.video_id).collect()

}

fn build_api(api_key: &str) -> impl Fn(&str, &[(&str, &str)]) -> Request
 {
    let mut headers = Headers::new();
    headers.set("X-goog-api-key", api_key).unwrap();
    headers.set("Accept", "application/json").unwrap();
    let headers = headers;

    let base = Url::parse("https://youtube.googleapis.com/youtube/v3/").unwrap();

    let func = move |resource: &str, params: &[(&str, &str)]| -> Request
     {
        let mut url = base.join(resource).unwrap();
        url.query_pairs_mut().extend_pairs(params).finish();

        Request::new_with_init(
            url.as_str(),
            RequestInit::new().with_headers(headers.clone()),
        )
        .unwrap()
    };

    func
}