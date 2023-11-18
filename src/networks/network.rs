use std::{collections::HashMap, convert::TryInto};

use reqwest::{header::HeaderMap, Client, Method, Response};

use crate::{errors::NetworkError, models::account_info::AccountInfo, utils::generate_ds};

const OS_DS_SALT: &str = "6s25p5ox5y14umn1p61aqyyvbvvl3lrt";
// const CN_DS_SALT: &str = "xV8v4Qu54lUKrEYFZkJhB8cuOh9Asafs";

const OS_RPC_APP_VERSION: &str = "1.5.0";
const OS_RPC_CLIENT_TYPE: &str = "5";
// const CN_RPC_APP_VERSION: &str = "2.11.1";
// const CN_RPC_CLIENT_TYPE: &str = "5";

pub async fn fetch_endpoint(
  url: String,
  method: Method,
  account_info: AccountInfo,
  lang: String,
  json: Option<&HashMap<String, String>>,
  query: Option<&HashMap<String, String>>,
  headers: Option<HeaderMap>,
) -> Result<Response, NetworkError> {
  let mut header_map = HashMap::new();
  header_map.insert("Cookie".to_string(), account_info.get_cookie());
  header_map.insert(
    "ds".to_string(),
    generate_ds(OS_DS_SALT).ok_or(NetworkError::GenerateDsError)?,
  );
  header_map.insert(
    "x-rpc-app_version".to_string(),
    OS_RPC_APP_VERSION.to_string(),
  );
  header_map.insert(
    "x-rpc-client_type".to_string(),
    OS_RPC_CLIENT_TYPE.to_string(),
  );
  header_map.insert("x-rpc-language".to_string(), lang);
  let client = Client::new();
  let mut builder = match method {
    Method::GET => client.get(url),
    Method::POST => client.post(url),
    _ => return Err(NetworkError::InvalidMethodError),
  };
  if let Some(json) = json {
    builder = builder.json(json);
  }
  if let Some(query) = query {
    builder = builder.query(query);
  }
  if let Some(headers) = headers {
    builder = builder.headers(headers);
  }
  Ok(
    builder
      .headers((&header_map).try_into().expect("header parse error"))
      .send()
      .await?,
  )
}
