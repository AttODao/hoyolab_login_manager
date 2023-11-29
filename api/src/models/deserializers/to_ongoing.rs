use serde::Deserialize;
use serde_with::DeserializeAs;

pub struct ToOngoing;

impl<'de> DeserializeAs<'de, bool> for ToOngoing {
  fn deserialize_as<D>(deserializer: D) -> Result<bool, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Ok(&String::deserialize(deserializer)? == "Ongoing")
  }
}
