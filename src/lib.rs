use std::collections::HashSet;

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
        .collect::<Vec<_>>();

    videos.sort_by(|a, b| b.snippet.published_at.cmp(&a.snippet.published_at));

    let duration = videos::get_duration(&api, &videos.iter().take(50).map(|x| x.content_details.video_id.clone()).collect::<Vec<_>>().join(","))
        .await
        .into_iter()
        .filter(|x| x.content_details.duration.0 > 150)
//            pred(x.content_details.duration.0.as_bytes(), 150)
//        })
        .map(|x| x.id)
        .collect::<HashSet<_>>();

    let video_ids = videos.into_iter()
        .map(|x| x.content_details.video_id)
        .filter(|x| duration.contains(x))
        .collect::<Vec<_>>().join(",");

    let dest = Url::parse_with_params(
        "https://www.youtube-nocookie.com/embed",
        &[
            ("autoplay", "1"),
            ("rel", "0"),
            ("playlist", &video_ids), 
        ]
    )?;


    Response::redirect_with_status(dest, 303)
}


fn pred(s: &[u8], limit_seconds: usize) -> bool {
    let Some((head, tail)) = s.split_first_chunk::<2>() else {
        panic!("");
    };

    if head.ne(&[b'P', b'T']) {
        panic!("");
    }

    let mut units = [(b'H', 60 * 60), (b'M', 60), (b'S', 1usize)].into_iter();

    // TODO: 指定した秒数を超えたらfoldを中断したい try_fold
    let seconds = tail.chunk_by(|a, _| a.is_ascii_digit())
        .fold(0, |total, iter| {
            let st = iter.iter().cloned().fold(0usize, |st, i| {
                if i.is_ascii_digit() {
                    (st * 10) + usize::from(i - b'0')
                } else {
                    units
                        .find(|(x, _)| *x == i)
                        .map(|(_, x)| x * st)
                        .unwrap()
                }
            });

            total + st
        });
    
    console_log!("{:#?}", seconds);
    seconds <= limit_seconds
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
