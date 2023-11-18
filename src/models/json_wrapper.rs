use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JsonWrapper<T> {
  pub retcode: i32,
  pub message: String,
  pub data: Option<T>,
}
