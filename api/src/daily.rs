use std::{collections::HashMap, hash::Hash};

use reqwest::{
  header::{HeaderMap, HeaderName, HeaderValue},
  Method, Response,
};

use crate::{
  api::fetch_endpoint,
  errors::NetworkError,
  models::{
    common::daily::{ClaimDaily, DailyInfo},
    json_wrapper::JsonWrapper,
    login_cookie::LoginCookie,
  },
  types::Game,
};

const OS_DAILY_URL: [&str; 3] = [
  "https://sg-hk4e-api.hoyolab.com/event/sol/",
  "https://sg-public-api.hoyolab.com/event/luna/os/",
  "https://sg-public-api.hoyolab.com/event/luna/zzz/os/",
];
const OS_ACT_ID: [&str; 3] = ["e202102251931481", "e202303301540311", "e202406031448091"];

async fn fetch_daily_endpoint(
  game: Game,
  endpoint: &str,
  method: Method,
  login_cookie: LoginCookie,
  lang: String,
) -> Result<Response, NetworkError> {
  let query = &vec![
    ("lang".to_string(), lang.clone()),
    ("act_id".to_string(), OS_ACT_ID[game as usize].to_string()),
  ]
  .into_iter()
  .collect::<HashMap<String, String>>();

  fetch_endpoint(
    OS_DAILY_URL[game as usize].to_string() + endpoint,
    method,
    login_cookie,
    lang,
    None,
    Some(query),
    match game {
      Game::Zzz => Some(
        (&[("x-rpc-signgame".to_string(), "zzz".to_string())]
          .into_iter()
          .collect::<HashMap<_, _>>())
          .try_into()
          .unwrap(),
      ),
      _ => None,
    },
  )
  .await
}

pub async fn get_daily_info<T: DailyInfo>(
  login_cookie: LoginCookie,
  lang: String,
) -> Result<JsonWrapper<T>, NetworkError> {
  // println!(
  //   "{}",
  //   fetch_daily_endpoint(
  //     game,
  //     "info",
  //     Method::GET,
  //     account_info.clone(),
  //     lang
  //   )
  //   .await?
  //   .text()
  //   .await?
  // );
  let game = T::game();
  let response = fetch_daily_endpoint(game, "info", Method::GET, login_cookie, lang).await?;
  response
    .json::<JsonWrapper<T>>()
    .await
    .map_err(|_| NetworkError::ParseJsonError)
}

pub async fn claim_daily<T: ClaimDaily>(
  login_cookie: LoginCookie,
  lang: String,
) -> Result<JsonWrapper<T>, NetworkError> {
  // println!(
  //   "{}",
  //   fetch_daily_endpoint(
  //     game,
  //     "sign",
  //     Method::POST,
  //     account_info.clone(),
  //     lang.clone()
  //   )
  //   .await?
  //   .text()
  //   .await?
  // );
  let game = T::game();
  let response = fetch_daily_endpoint(game, "sign", Method::POST, login_cookie, lang).await?;
  response
    .json::<JsonWrapper<T>>()
    .await
    .map_err(|_| NetworkError::ParseJsonError)
}
