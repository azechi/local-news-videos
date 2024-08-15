use worker::*;

mod fetch;
mod playlist_items;
mod videos;

#[event(fetch)]
async fn fetch(_req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let api_key = env.var("GOOGLE_API_KEY").unwrap().to_string();

    let api = build_api(&api_key);

    let videos = playlist_items::get_video_ids(&api, "UULFxiRdfyH0FtFCRZTRfRsdsA").await;

    let duration = videos::get_duration(&api, &videos.join(",")).await;
    console_log!("{:#?}", duration);

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