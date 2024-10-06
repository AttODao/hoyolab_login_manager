use std::collections::HashMap;

use reqwest::{Method, Response};

use crate::{
  api::fetch_endpoint,
  errors::NetworkError,
  models::{
    common::game_record::{DailyNote, UserStats},
    json_wrapper::JsonWrapper,
    login_cookie::LoginCookie,
  },
  types::Game,
  utils::recognize_server,
};

const OS_GAME_RECORD_URL: [&str; 3] = [
  "https://bbs-api-os.hoyoverse.com/game_record/genshin/api/",
  "https://bbs-api-os.hoyolab.com/game_record/hkrpg/api/",
  "https://sg-public-api.hoyolab.com/event/game_record_zzz/api/zzz/",
];

async fn fetch_game_record_endpoint(
  game: Game,
  endpoint: &str,
  method: Method,
  login_cookie: LoginCookie,
  lang: String,
  query: &HashMap<String, String>,
) -> Result<Response, NetworkError> {
  fetch_endpoint(
    OS_GAME_RECORD_URL[game as usize].to_string() + endpoint,
    method,
    login_cookie,
    lang,
    None,
    Some(query),
    None,
  )
  .await
}

pub async fn get_user_stats<T: UserStats>(
  uid: String,
  login_cookie: LoginCookie,
  lang: String,
) -> Result<JsonWrapper<T>, NetworkError> {
  // println!(
  //   "{}",
  //   fetch_game_record_endpoint(
  //     T::game(),
  //     "index",
  //     Method::GET,
  //     login_cookie.clone(),
  //     lang.clone(),
  //     &vec![
  //       (
  //         "server".to_string(),
  //         recognize_server(T::game(), &uid)
  //           .ok_or(NetworkError::InvalidUidError)?
  //           .to_string(),
  //       ),
  //       ("role_id".to_string(), uid.clone())
  //     ]
  //     .into_iter()
  //     .collect::<HashMap<String, String>>(),
  //   )
  //   .await?
  //   .text()
  //   .await?
  // );
  let game = T::game();
  let response = fetch_game_record_endpoint(
    game,
    "index",
    Method::GET,
    login_cookie,
    lang,
    &[
      (
        "server".to_string(),
        recognize_server(game, &uid)
          .ok_or(NetworkError::InvalidUidError)?
          .to_string(),
      ),
      ("role_id".to_string(), uid),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>(),
  )
  .await?;
  response
    .json::<JsonWrapper<T>>()
    .await
    .map_err(|_| NetworkError::ParseJsonError)
}

pub async fn get_daily_note<T: DailyNote>(
  uid: String,
  login_cookie: LoginCookie,
  lang: String,
) -> Result<JsonWrapper<T>, NetworkError> {
  println!(
    "{}",
    fetch_game_record_endpoint(
      T::game(),
      "note",
      Method::GET,
      login_cookie.clone(),
      lang.clone(),
      &[
        (
          "server".to_string(),
          recognize_server(T::game(), &uid)
            .ok_or(NetworkError::InvalidUidError)?
            .to_string(),
        ),
        ("role_id".to_string(), uid.clone())
      ]
      .into_iter()
      .collect::<HashMap<_, _>>(),
    )
    .await?
    .text()
    .await?
  );
  let game = T::game();
  let response = fetch_game_record_endpoint(
    game,
    ["dailyNote", "note", "note"][game as usize],
    Method::GET,
    login_cookie,
    lang,
    &vec![
      (
        "server".to_string(),
        recognize_server(game, &uid)
          .ok_or(NetworkError::InvalidUidError)?
          .to_string(),
      ),
      ("role_id".to_string(), uid),
    ]
    .into_iter()
    .collect::<HashMap<String, String>>(),
  )
  .await?;
  response
    .json::<JsonWrapper<T>>()
    .await
    .map_err(|_| NetworkError::ParseJsonError)
}
