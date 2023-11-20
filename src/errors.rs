use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
  #[error("{0}")]
  SerenityError(#[from] poise::serenity_prelude::Error),
}

#[derive(Error, Debug)]
pub enum NetworkError {
  #[error("generate ds error")]
  GenerateDsError,
  #[error("http request error({0})")]
  HttpRequestError(#[from] reqwest::Error),
  #[error("invalid method")]
  InvalidMethodError,
  #[error("parse json error")]
  ParseJsonError,
  #[error("invalid user id")]
  InvalidUidError,
}
