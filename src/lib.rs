use worker::*;

use serde::Deserialize;

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

#[event(fetch)]
async fn fetch(_req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let api_key = env.var("GOOGLE_API_KEY").unwrap().to_string();

    let build = build(&api_key);

    let req = build(
        "playlistItems",
        &[
            ("part", "contentDetails,snippet,id,status"),
            ("playlistId", "UULFxiRdfyH0FtFCRZTRfRsdsA"),
        ],
    );
    console_log!("{}", req.url().unwrap().as_str());

    let mut res = Fetch::Request(req).send().await?;
    let dat = res.json::<PlayListItems>().await?;
    console_log!("{:#?}", dat.items[0].content_details.video_id);

    //console_log!("{:#?} {}", res.headers(), res.status_code());

    Response::empty()

    //let dest = Url::parse("https://www.youtube.com/embed?playlist=lFRyJ_sQ350,zZSw5lvG_Wg,xA8rnCmyCJM&autoplay=1&rel=0")?;
    //Response::redirect_with_status(dest, 303)
}

//fn build(api_key: &str) -> Box<dyn Fn(&str, &[(&str, &str)]) -> Request> {
fn build(api_key: &str) -> impl Fn(&str, &[(&str, &str)]) -> Request {
    let mut headers = Headers::new();
    headers.set("X-goog-api-key", api_key).unwrap();
    headers.set("Accept", "application/json").unwrap();
    let headers = headers;

    let base = Url::parse("https://youtube.googleapis.com/youtube/v3/").unwrap();

    move |resource: &str, params: &[(&str, &str)]| -> Request {
        let mut url = base.join(resource).unwrap();
        url.query_pairs_mut().extend_pairs(params).finish();

        Request::new_with_init(
            url.as_str(),
            RequestInit::new().with_headers(headers.clone()),
        )
        .unwrap()
    }
}
