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
pub struct StarrailUserStats {
  pub stats: StarrailStats,
  pub avatar_list: Vec<StarrailAvatar>,
  pub cur_head_icon_url: String,
  pub phone_background_image_url: String,
}
#[derive(Debug, Deserialize)]
pub struct StarrailStats {
  pub active_days: i32,
  pub avatar_num: i32,
  pub achievement_num: i32,
  pub chest_num: i32,
  pub abyss_process: String,
  pub field_ext_map: StarrailFieldExtMap,
  pub dream_paster_num: i32,
}
#[derive(Debug, Deserialize)]
pub struct StarrailFieldExtMap {
  pub dream_paster_num: StarrailFieldExtMapLink,
  pub active_days: StarrailFieldExtMapLink,
  pub avatar_num: StarrailFieldExtMapLink,
  pub achievement_num: StarrailFieldExtMapLink,
  pub chest_num: StarrailFieldExtMapLink,
}
#[derive(Debug, Deserialize)]
pub struct StarrailFieldExtMapLink {
  pub link: String,
  pub backup_link: String,
}
#[derive(Debug, Deserialize)]
pub struct StarrailAvatar {
  pub id: i32,
  pub level: i32,
  pub name: String,
  pub element: String,
  pub icon: String,
  pub rarity: i32,
  pub rank: i32,
  pub is_chosen: bool,
}

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

impl DailyNote for StarrailDailyNote {}
impl GameIdentification for StarrailDailyNote {
  fn game() -> Game {
    Game::Starrail
  }
}
