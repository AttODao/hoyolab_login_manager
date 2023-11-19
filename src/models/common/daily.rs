use serde::de::DeserializeOwned;

use crate::models::game_identification::GameIdentification;

pub trait DailyInfo: GameIdentification + DeserializeOwned {}
pub trait ClaimDaily: GameIdentification + DeserializeOwned {}
