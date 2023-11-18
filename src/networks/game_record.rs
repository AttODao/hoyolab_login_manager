use std::collections::HashMap;

use reqwest::{Method, Response};

use crate::{
  errors::NetworkError,
  models::{
    account_info::AccountInfo,
    genshin::game_record::{DailyNote, UserStats},
    json_wrapper::JsonWrapper,
  },
  types::Game,
  utils::recognize_server,
};

use super::network::fetch_endpoint;

const OS_GAME_RECORD_URL: &str = "https://bbs-api-os.hoyoverse.com/game_record/";

async fn fetch_game_record_endpoint(
  endpoint: &str,
  method: Method,
  account_info: AccountInfo,
  lang: String,
  query: &HashMap<String, String>,
) -> Result<Response, NetworkError> {
  fetch_endpoint(
    OS_GAME_RECORD_URL.to_string() + endpoint,
    method,
    account_info,
    lang,
    None,
    Some(query),
    None,
  )
  .await
}

pub async fn get_user_stats(
  uid: String,
  account_info: AccountInfo,
  lang: String,
) -> Result<JsonWrapper<UserStats>, NetworkError> {
  // println!(
  //   "{}",
  //   fetch_game_record_endpoint(
  //     "genshin/api/index",
  //     Method::GET,
  //     account_info.clone(),
  //     lang.clone(),
  //     &vec![
  //       (
  //         "server".to_string(),
  //         recognize_server(Game::Genshin, &uid)
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
  let response = fetch_game_record_endpoint(
    "genshin/api/index",
    Method::GET,
    account_info,
    lang,
    &vec![
      (
        "server".to_string(),
        recognize_server(Game::Genshin, &uid)
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
    .json::<JsonWrapper<UserStats>>()
    .await
    .map_err(|_| NetworkError::ParseJsonError)
}

pub async fn get_daily_note(
  uid: String,
  account_info: AccountInfo,
  lang: String,
) -> Result<JsonWrapper<DailyNote>, NetworkError> {
  // println!(
  //   "{}",
  //   fetch_game_record_endpoint(
  //     "genshin/api/dailyNote",
  //     Method::GET,
  //     account_info.clone(),
  //     lang.clone(),
  //     &vec![
  //       (
  //         "server".to_string(),
  //         recognize_server(Game::Genshin, &uid)
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
  let response = fetch_game_record_endpoint(
    "genshin/api/dailyNote",
    Method::GET,
    account_info,
    lang,
    &vec![
      (
        "server".to_string(),
        recognize_server(Game::Genshin, &uid)
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
    .json::<JsonWrapper<DailyNote>>()
    .await
    .map_err(|_| NetworkError::ParseJsonError)
}
