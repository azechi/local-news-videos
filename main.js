function doGet(e) {

  const ids = [...Iterator.from(main()).filter((item) => {
    if (item.title.includes("ビシバシ天気")) {
      item.duration = 0;
      item.publishedAt = new Date();
    }
    return item.duration <= 150;
  })].sort((a, b) => b.publishedAt - a.publishedAt);

  for (const i of ids) {
    Logger.log("%s %s %s", i.publishedAt, i.duration, i.title);
  }

  const tmpl = HtmlService.createTemplateFromFile("index.html");
  tmpl.idArray = ids.map(i => i.id);
  return tmpl.evaluate();
}

function* getPlaylistItems(playlistId, lastDate = new Date(Date.now() - (24 * 60 * 60 * 1000)/* 1day(ms) */)) {
  const MAXRESULTS = 50;

  // TODO: ページの最後がlastDateより新しかったときには次のページを取得する
  const result = YouTube.PlaylistItems.list("contentDetails", { "playlistId": playlistId, "maxResults": MAXRESULTS });

  const items = result.items;

  const newest = items[0].contentDetails.videoPublishedAt;
  const oldest = items[items.length - 1].contentDetails.videoPublishedAt;
  Logger.log("%s %s/%s %s %s %s", playlistId, result.pageInfo.resultsPerPage.toString(), result.pageInfo.totalResults.toString(), lastDate.toISOString(), newest, oldest);

  // TODO: assert 対象のプレイリストはpublishedlAtの降順のはず、result.items配列の順序も同じか確認したい
  if (false) {
    GmailApp.sendEmail(
      Session.getActiveUser().getEmail(),
      "subject",
      "body");
  }

  for (const item of result.items) {
    if (new Date(item.contentDetails.videoPublishedAt) <= lastDate) {
      break;
    }
    Logger.log("%s %s %s", playlistId, item.contentDetails.videoId, item.contentDetails.videoPublishedAt)
    yield item.contentDetails.videoId;
  }
}


function* getVideoIds(playlistIds) {
  for (const playlistId of playlistIds) {
    // todo: チャンネルごとの最終取得日
    yield* getPlaylistItems(playlistId);
  }
}

function* main() {

  const eventId = Utilities.getUuid();
  const file = SpreadsheetApp.openById("1yQRmeVcRUwVeDhzTdfgFdzQQAhcnbqGLxA3XA6X_vXk");
  // const sheet =
  //   sheet.getRange(sheet.getLastRow() + 1, 1, rows.length, rows[0].length)
  //     .setValues(rows);

  // チャンネルの全動画(自動)プレイリスト、時間降順(のはず)
  const playlistIds = [
    "UULFh5mvJtIWycou5b8smpBuxA", // @cbctv_news CBCニュース【CBCテレビ公式】
    "UULFxiRdfyH0FtFCRZTRfRsdsA", // @CHUKYOTV_NEWS 中京テレビNEWS
    "UULFWnOKASPkCBEL-_O8odMvtg", // @NagoyaTVnewsCH メ〜テレニュース
    "UULFUQ0AtI3k10_CyLYcu4WRVA", // @newsone4365 東海テレビ NEWS ONE
    "UULFk6SzG4qmA7J6CI-QAtWoOg", // @aichi-news 愛知のニュース【テレビ愛知　ニュース・スポーツ公式チャンネル】
    "UULFD8zqZumr5CzsmXCMPZVA2g", // @Mietv_news 三重テレビNEWS
    "UULFpNa9Nyxza1BrF0hUWTuj1Q", // @user-gl9gf8qo2w ぎふチャン公式チャンネル
  ];

  for (const ids of chunk(getVideoIds(playlistIds), 50)) {
    const result = YouTube.Videos.list("id,snippet,contentDetails,status", { "id": [...ids].join(",") });
    Logger.log("items:%s page:%s", result.items.length, result.pageInfo);

    // 入力パラメーターのvideoIdと結果のitemsの並び順は同じ？？

    // 全動画playlistのplayliteItem.contentDetails.videoPublishedAt と video.snippet.publishedAt は同じ？？

    const rows = result.items.map(i => {
      return [
        new Date(i.snippet.publishedAt).valueOf().toString(),
        eventId,
        i.snippet.channelId, i.snippet.channelTitle,
        i.kind, i.etag, i.id,
        i.status.embeddable, i.status.madeForKids,
        i.contentDetails.duration,
        i.snippet.thumbnails["default"].url, i.snippet.thumbnails["default"].height, i.snippet.thumbnails["default"].width,
        i.snippet.publishedAt, i.snippet.title, i.snippet.description,
        i.snippet.tags, i.snippet.categoryId
      ];
    })

    const range = file.getSheets()[0].getRange(file.getLastRow() + 1, 1, rows.length, rows[0].length)
    range.setValues(rows);

    yield* result.items.values().map(item => ({
      "publishedAt": new Date(item.snippet.publishedAt),
      "id": item.id,
      "title": item.snippet.title,
      "duration": parse_ISO8601DurationFormat(item.contentDetails.duration),
      item
    }));
  }
}


function* chunk(/** @type Iterable.<*> */ iterable, size) {

  const iter = iterable[Symbol.iterator]();

  let result = iter.next();
  while (!result.done) {
    yield function* () {
      let i = 0;
      while (!result.done) {
        if (i >= size) {
          break;
        }
        yield result.value;
        i++;
        result = iter.next();
      }
    }();
  }
}

function parse_ISO8601DurationFormat(s) {
  // P**Dとかは対象にしない: nullを返す
  // 整数値のみ対象: 仕様的には0.5とかもアリらしい
  const matches = /PT(\d+H)?(\d+M)?(\d+S)?/.exec(s);
  if (!matches) {
    Logger.log("Parsing failed: '%s'", s)
    return null;
  }

  /* 
    https://developer.mozilla.org/ja/docs/Web/JavaScript/Reference/Global_Objects/parseInt
    > parseInt は、入力文字列の中で、指定された radix で有効な数字ではない文字を見つけた場合、その文字とそれ以降のすべての文字を無視し、その時点までに構文解析した整数値返します。
  */
  const hours = parseInt(matches[1] || 0, 10);
  const minutes = parseInt(matches[2] || 0, 10);
  const seconds = parseInt(matches[3] || 0, 10);

  return (hours * 60 * 60) + (minutes * 60) + seconds;
}
