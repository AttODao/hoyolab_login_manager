use serde::Deserialize;
use serde_with::DeserializeAs;

pub struct ToCardSign;

impl<'de> DeserializeAs<'de, bool> for ToCardSign {
  fn deserialize_as<D>(deserializer: D) -> Result<bool, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Ok(&String::deserialize(deserializer)? == "CardSignDone")
  }
}
