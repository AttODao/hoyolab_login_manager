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
  pub current_stamina: i32,
  pub max_stamina: i32,
  #[serde_as(as = "crate::models::deserializers::starrail_to_duration::StarrailToDuration")]
  pub stamina_recover_time: Duration,
  pub accepted_epedition_num: i32,
  pub total_expedition_num: i32,
  pub expeditions: Vec<StarrailExpedition>,
  pub current_train_score: i32,
  pub max_train_score: i32,
  pub current_rogue_score: i32,
  pub max_rogue_score: i32,
  pub weekly_cocoon_cnt: i32,
  pub weekly_cocoon_limit: i32,
  pub current_reserve_stamina: i32,
  pub is_reserve_stamina_full: bool,
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
  pub avatars: Vec<String>,
  #[serde(rename = "status")]
  #[serde_as(as = "crate::models::deserializers::to_ongoing::ToOngoing")]
  pub ongoing: bool,
  #[serde_as(as = "crate::models::deserializers::starrail_to_duration::StarrailToDuration")]
  pub remaining_time: Duration,
  pub name: String,
  pub item_url: String,
}
