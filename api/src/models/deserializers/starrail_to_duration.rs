use std::time::Duration;

use serde::Deserialize;
use serde_with::DeserializeAs;

pub struct StarrailToDuration;

impl<'de> DeserializeAs<'de, Duration> for StarrailToDuration {
  fn deserialize_as<D>(deserializer: D) -> Result<Duration, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Ok(Duration::from_secs(u64::deserialize(deserializer)?))
  }
}
