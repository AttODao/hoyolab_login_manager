use std::time::Duration;

use serde::Deserialize;
use serde_with::DeserializeAs;

pub struct ZzzToDuration;

impl<'de> DeserializeAs<'de, Duration> for ZzzToDuration {
  fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Ok(Duration::from_secs(
      String::deserialize(deserializer)?.parse().unwrap(),
    ))
  }
}
