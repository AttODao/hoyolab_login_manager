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
  current_resin: i32,
  max_resin: i32,
  #[serde_as(as = "crate::models::deserializers::genshin_to_duration::GenshinToDuration")]
  resin_recovery_time: Duration,
  finished_task_num: i32,
  total_task_num: i32,
  is_extra_task_reward_received: bool,
  remain_resin_discount_num: i32,
  resin_discount_num_limit: i32,
  current_expedition_num: i32,
  max_expedition_num: i32,
  expeditions: Vec<GenshinExpedition>,
  current_home_coin: i32,
  max_home_coin: i32,
  #[serde_as(as = "crate::models::deserializers::genshin_to_duration::GenshinToDuration")]
  home_coin_recovery_time: Duration,
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
  avatar_side_icon: String,
  status: String,
  #[serde_as(as = "crate::models::deserializers::genshin_to_duration::GenshinToDuration")]
  remained_time: Duration,
}
