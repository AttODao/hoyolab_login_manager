use std::collections::HashMap;

use reqwest::{Method, Response};

use crate::{
  errors::NetworkError,
  models::{
    account_info::AccountInfo,
    genshin::daily::{ClaimDaily, DailyInfo},
    json_wrapper::JsonWrapper,
  },
};

use super::network::fetch_endpoint;

const OS_DAILY_URL: &str = "https://sg-hk4e-api.hoyolab.com/event/sol/";
const OS_ACT_ID: &str = "e202102251931481";

async fn fetch_daily_endpoint(
  endpoint: &str,
  method: Method,
  account_info: AccountInfo,
  lang: String,
) -> Result<Response, NetworkError> {
  let query = &vec![
    ("lang".to_string(), lang.clone()),
    ("act_id".to_string(), OS_ACT_ID.to_string()),
  ]
  .into_iter()
  .collect::<HashMap<String, String>>();

  fetch_endpoint(
    OS_DAILY_URL.to_string() + endpoint,
    method,
    account_info,
    lang,
    None,
    Some(query),
    None,
  )
  .await
}

pub async fn get_daily_info(
  account_info: AccountInfo,
  lang: String,
) -> Result<JsonWrapper<DailyInfo>, NetworkError> {
  // println!(
  //   "{}",
  //   fetch_daily_endpoint(
  //     "info",
  //     Method::GET,
  //     account_info.clone(),
  //     lang
  //   )
  //   .await?
  //   .text()
  //   .await?
  // );
  let response = fetch_daily_endpoint("info", Method::GET, account_info, lang).await?;
  response
    .json::<JsonWrapper<DailyInfo>>()
    .await
    .map_err(|_| NetworkError::ParseJsonError)
}

pub async fn claim_daily(
  account_info: AccountInfo,
  lang: String,
) -> Result<JsonWrapper<ClaimDaily>, NetworkError> {
  println!(
    "{}",
    fetch_daily_endpoint("sign", Method::POST, account_info.clone(), lang.clone())
      .await?
      .text()
      .await?
  );
  let response = fetch_daily_endpoint("sign", Method::POST, account_info, lang).await?;
  response
    .json::<JsonWrapper<ClaimDaily>>()
    .await
    .map_err(|_| NetworkError::ParseJsonError)
}
