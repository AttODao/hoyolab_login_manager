use std::time::Duration;

use serde::Deserialize;
use serde_with::serde_as;

use crate::{
  models::{
    common::game_record::{DailyNote, UserStats},
    game_identification::GameIdentification,
  },
  types::Game,
};

#[derive(Debug, Deserialize)]
pub struct StarrailUserStats {}
impl UserStats for StarrailUserStats {}
impl GameIdentification for StarrailUserStats {
  fn game() -> Game {
    Game::Starrail
  }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct StarrailDailyNote {
  current_stamina: i32,
  max_stamina: i32,
  #[serde_as(as = "crate::models::deserializers::to_duration::ToDuration")]
  stamina_recover_time: Duration,
  accepted_epedition_num: i32,
  total_expedition_num: i32,
  expeditions: Vec<StarrailExpedition>,
  current_train_score: i32,
  max_train_score: i32,
  current_rogue_score: i32,
  max_rogue_score: i32,
  weekly_cocoon_cnt: i32,
  weekly_cocoon_limit: i32,
  current_reserve_stamina: i32,
  is_reserve_stamina_full: bool,
}
impl DailyNote for StarrailDailyNote {}
impl GameIdentification for StarrailDailyNote {
  fn game() -> Game {
    Game::Starrail
  }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct StarrailExpedition {
  avatars: Vec<String>,
  status: String,
  #[serde_as(as = "crate::models::deserializers::to_duration::ToDuration")]
  remaining_time: Duration,
  name: String,
  item_url: String,
}
