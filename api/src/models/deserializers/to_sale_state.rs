use serde::Deserialize;
use serde_with::DeserializeAs;

pub struct ToSaleState;

impl<'de> DeserializeAs<'de, bool> for ToSaleState {
  fn deserialize_as<D>(deserializer: D) -> Result<bool, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    Ok(&String::deserialize(deserializer)? == "SaleStateDoing")
  }
}
