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
pub struct GenshinUserStats {
  pub role: GenshinRole,
  pub avatars: Vec<GenshinAvatar>,
  pub stats: GenshinStats,
  pub city_explorations: Vec<GenshinCityExploration>,
  pub world_explorations: Vec<GenshinWorldExploration>,
  pub homes: Vec<GenshinHome>,
  pub query_tool_link: String,
  pub query_tool_image: String,
}
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct GenshinRole {
  pub AvatarUrl: String,
  pub nickname: String,
  pub region: String,
  pub level: i32,
  pub game_head_icon: String,
}
#[derive(Debug, Deserialize)]
pub struct GenshinAvatar {
  pub id: i32,
  pub image: String,
  pub element: String,
  pub fetter: i32,
  pub level: i32,
  pub rarity: i32,
  pub actived_constellation_num: i32,
  pub card_image: String,
  pub is_chosen: bool,
}
#[derive(Debug, Deserialize)]
pub struct GenshinStats {
  pub active_day_number: i32,
  pub achievement_number: i32,
  pub anemoculus_number: i32,
  pub geoculus_number: i32,
  pub avatar_number: i32,
  pub way_point_number: i32,
  pub domain_number: i32,
  pub spiral_abyss: String,
  pub precious_chest_number: i32,
  pub luxurious_chest_number: i32,
  pub exquisite_chest_number: i32,
  pub common_chest_number: i32,
  pub electroculus_number: i32,
  pub magic_chest_number: i32,
  pub dendroculus_number: i32,
  pub hydroculus_number: i32,
  pub field_ext_map: GenshinFieldExtMap,
}
#[derive(Debug, Deserialize)]
pub struct GenshinFieldExtMap {
  pub electroculus_number: GenshinFieldExtMapLink,
  pub exquisite_chest_number: GenshinFieldExtMapLink,
  pub way_point_number: GenshinFieldExtMapLink,
  pub common_chest_number: GenshinFieldExtMapLink,
  pub spiral_abyss: GenshinFieldExtMapLink,
  pub precious_chest_number: GenshinFieldExtMapLink,
  pub hydroculus_number: GenshinFieldExtMapLink,
  pub dendroculus_number: GenshinFieldExtMapLink,
  pub geoculus_number: GenshinFieldExtMapLink,
  pub anemoculus_number: GenshinFieldExtMapLink,
  pub magic_chest_number: GenshinFieldExtMapLink,
  pub avatar_number: GenshinFieldExtMapLink,
  pub domain_number: GenshinFieldExtMapLink,
  pub luxurious_chest_number: GenshinFieldExtMapLink,
}
#[derive(Debug, Deserialize)]
pub struct GenshinFieldExtMapLink {
  pub link: String,
  pub backup_link: String,
}
#[derive(Debug, Deserialize)]
pub struct GenshinCityExploration {}
#[derive(Debug, Deserialize)]
pub struct GenshinWorldExploration {
  pub level: i32,
  pub exploration_percentage: i32,
  pub icon: String,
  pub name: String,
  pub r#type: String,
  pub offerings: Vec<GenshinOfferings>,
  pub id: i32,
  pub parent_id: i32,
  pub map_url: String,
  pub strategy_url: String,
  pub background_image: String,
  pub inner_icon: String,
  pub cover: String,
  pub area_exploration_list: Vec<GenshinExploration>,
  pub boss_list: Vec<GenshinBoss>,
  pub is_hot: bool,
  pub index_active: bool,
  pub detail_active: bool,
}
#[derive(Debug, Deserialize)]
pub struct GenshinOfferings {
  pub name: String,
  pub level: i32,
  pub icon: String,
}
#[derive(Debug, Deserialize)]
pub struct GenshinExploration {
  pub name: String,
  pub exploration_percentage: i32,
}
#[derive(Debug, Deserialize)]
pub struct GenshinBoss {
  pub name: String,
  pub kill_num: i32,
}
#[derive(Debug, Deserialize)]
pub struct GenshinHome {
  pub level: i32,
  pub visit_num: i32,
  pub comfort_num: i32,
  pub item_num: i32,
  pub name: String,
  pub icon: String,
  pub comfort_level_name: String,
  pub comfort_level_icon: String,
}

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

impl DailyNote for GenshinDailyNote {}
impl GameIdentification for GenshinDailyNote {
  fn game() -> Game {
    Game::Genshin
  }
}
