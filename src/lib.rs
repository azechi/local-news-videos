use worker::*;

mod fetch;
mod playlist_items;
mod videos;

pub const PLAYLIST_IDS: [&str; 8] = [
        "UULFh5mvJtIWycou5b8smpBuxA", // @cbctv_news CBCニュース【CBCテレビ公式】
        "UULFxiRdfyH0FtFCRZTRfRsdsA", // @CHUKYOTV_NEWS 中京テレビNEWS
        "UULFWnOKASPkCBEL-_O8odMvtg", // @NagoyaTVnewsCH メ〜テレニュース
        "UULFUQ0AtI3k10_CyLYcu4WRVA", // @newsone4365 東海テレビ NEWS ONE
        "UULFk6SzG4qmA7J6CI-QAtWoOg", // @aichi-news 愛知のニュース【テレビ愛知　ニュース・スポーツ公式チャンネル】
        "UULFD8zqZumr5CzsmXCMPZVA2g", // @Mietv_news 三重テレビNEWS
        "UULFpNa9Nyxza1BrF0hUWTuj1Q", // @user-gl9gf8qo2w ぎふチャン公式チャンネル
        "UULFfD9k0fXeyRlYiMHaBLzS3Q", // @Chunichi_Shimbun 中日新聞デジタル編集部
    ];

#[event(fetch)]
async fn fetch(_req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let api_key = env.var("GOOGLE_API_KEY").unwrap().to_string();

    let api = build_api(&api_key);

    use futures::future::join_all;

    let _milliseconds = chrono::Duration::days(-1).num_milliseconds();
    

    let mut yesterday = chrono::Utc::now();
    yesterday = yesterday + chrono::Duration::days(-1);

    let mut videos = join_all(PLAYLIST_IDS.into_iter().map(|id| playlist_items::get_playlist_items(&api, id)))
        .await
        .into_iter()
        .flatten()
        .filter(|x| x.snippet.published_at > yesterday)
        .take(20)
        .collect::<Vec<_>>();

    videos.sort_by(|a, b| a.snippet.published_at.cmp(&b.snippet.published_at).reverse());

    //let duration = videos::get_duration(&api, &videos.iter().map(|x| x.content_details.video_id.clone()).collect::<Vec<_>>().join(",")).await;
    //console_log!("{:#?}", duration);

    let video_ids = &videos.iter().map(|x| x.content_details.video_id.clone()).collect::<Vec<_>>().join(",");

    let dest = Url::parse_with_params(
        "https://www.youtube-nocookie.com/embed",
        &[
            ("autoplay", "1"),
            ("rel", "0"),
            ("playlist", video_ids), 
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
