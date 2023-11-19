use serde::de::DeserializeOwned;

use crate::models::game_identification::GameIdentification;

pub trait UserStats: GameIdentification + DeserializeOwned {}

pub trait DailyNote: GameIdentification + DeserializeOwned {}
