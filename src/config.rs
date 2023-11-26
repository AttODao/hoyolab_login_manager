use config::ConfigError;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub hoge: String,
  pub discord_token: String,
  pub database_url: String,
  pub lang: String,
}
impl Config {
  pub fn from_file() -> Result<Self, ConfigError> {
    let cfg = config::Config::builder()
      .add_source(config::File::with_name("config"))
      .build()?;
    cfg.try_deserialize()
  }
}

pub static CONFIG: Lazy<Config> =
  Lazy::new(|| Config::from_file().expect("Failed to load config file"));
