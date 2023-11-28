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
pub struct GenshinUserStats {}
impl UserStats for GenshinUserStats {}
impl GameIdentification for GenshinUserStats {
  fn game() -> Game {
    Game::Genshin
  }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct GenshinDailyNote {
  pub current_resin: i32,
  pub max_resin: i32,
  #[serde_as(as = "crate::models::deserializers::genshin_to_duration::GenshinToDuration")]
  pub resin_recovery_time: Duration,
  pub finished_task_num: i32,
  pub total_task_num: i32,
  pub is_extra_task_reward_received: bool,
  pub remain_resin_discount_num: i32,
  pub resin_discount_num_limit: i32,
  pub current_expedition_num: i32,
  pub max_expedition_num: i32,
  pub expeditions: Vec<GenshinExpedition>,
  pub current_home_coin: i32,
  pub max_home_coin: i32,
  #[serde_as(as = "crate::models::deserializers::genshin_to_duration::GenshinToDuration")]
  pub home_coin_recovery_time: Duration,
}
impl DailyNote for GenshinDailyNote {}
impl GameIdentification for GenshinDailyNote {
  fn game() -> Game {
    Game::Genshin
  }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct GenshinExpedition {
  pub avatar_side_icon: String,
  #[serde(rename = "status")]
  #[serde_as(as = "crate::models::deserializers::to_ongoing::ToOngoing")]
  pub ongoing: bool,
  #[serde_as(as = "crate::models::deserializers::genshin_to_duration::GenshinToDuration")]
  pub remained_time: Duration,
}
