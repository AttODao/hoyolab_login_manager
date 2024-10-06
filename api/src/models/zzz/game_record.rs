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
pub struct ZzzUserStats {
  pub stats: ZzzStats,
  pub avatar_list: Vec<ZzzAvatar>,
  pub cur_head_icon_url: String,
  pub buddy_list: Vec<ZzzBuddy>,
  pub cat_notes_list: Vec<ZzzCatNotes>,
  pub award_state: String,
}
#[derive(Debug, Deserialize)]
pub struct ZzzStats {
  pub active_days: i32,
  pub avatar_num: i32,
  pub world_level_name: String,
  pub cur_period_zone_layer_count: i32,
  pub buddy_num: i32,
  pub commemorative_coins_list: Vec<ZzzStatsCommemorativeCoins>,
  pub achievement_count: i32,
}
#[derive(Debug, Deserialize)]
pub struct ZzzStatsCommemorativeCoins {
  pub num: i32,
  pub name: String,
  pub sort: i32,
}
#[derive(Debug, Deserialize)]
pub struct ZzzAvatar {
  pub id: i32,
  pub level: i32,
  pub name_mi18n: String,
  pub full_name_mi18n: String,
  pub element_type: i32,
  pub camp_name_mi18n: String,
  pub avatar_profession: i32,
  pub rarity: String,
  pub group_icon_path: String,
  pub hollow_icon_path: String,
  pub rank: i32,
  pub is_choosen: bool,
}
#[derive(Debug, Deserialize)]
pub struct ZzzBuddy {
  pub id: i32,
  pub name: String,
  pub rarity: String,
  pub level: i32,
  pub star: i32,
}
#[derive(Debug, Deserialize)]
pub struct ZzzCatNotes {
  pub name: String,
  pub icon: String,
  pub num: i32,
  pub total: i32,
  pub is_lock: bool,
  pub id: i32,
}

impl UserStats for ZzzUserStats {}
impl GameIdentification for ZzzUserStats {
  fn game() -> Game {
    Game::Zzz
  }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ZzzDailyNote {
  pub energy: ZzzEnergy,
  pub vitality: ZzzVitality,
  pub vhs_sale: ZzzVhsSale,
  #[serde_as(as = "crate::models::deserializers::to_card_sign::ToCardSign")]
  pub card_sign: bool,
  pub bounty_commission: ZzzBountyCommission,
  pub survey_points: ZzzSurveyPoints,
  #[serde_as(as = "crate::models::deserializers::zzz_to_duration::ZzzToDuration")]
  pub abyss_refresh: Duration,
}
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ZzzEnergy {
  pub progress: ZzzEnergyProgress,
  #[serde_as(as = "crate::models::deserializers::zzz_to_duration::ZzzToDuration")]
  pub restore: Duration,
  pub day_type: i32,
  pub hour: i32,
  pub minute: i32,
}
#[derive(Debug, Deserialize)]
pub struct ZzzEnergyProgress {
  pub max: i32,
  pub current: i32,
}
#[derive(Debug, Deserialize)]
pub struct ZzzVitality {
  pub max: i32,
  pub current: i32,
}
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct ZzzVhsSale {
  #[serde_as(as = "crate::models::deserializers::to_sale_state::ToSaleState")]
  pub sale_state: bool,
}
#[derive(Debug, Deserialize)]
pub struct ZzzBountyCommission {
  pub num: i32,
  pub total: i32,
}
#[derive(Debug, Deserialize)]
pub struct ZzzSurveyPoints {
  pub num: i32,
  pub total: i32,
  pub is_max_level: bool,
}

impl DailyNote for ZzzDailyNote {}
impl GameIdentification for ZzzDailyNote {
  fn game() -> Game {
    Game::Zzz
  }
}
