use serde::Deserialize;

use crate::{
  models::{common::daily::{DailyInfo, ClaimDaily}, game_identification::GameIdentification},
  types::Game,
};

#[derive(Debug, Deserialize)]
pub struct GenshinDailyInfo {
  pub is_sign: bool,
  pub total_sign_day: i32,
}
impl DailyInfo for GenshinDailyInfo {}
impl GameIdentification for GenshinDailyInfo {
  fn game() -> Game {
    Game::Genshin
  }
}

#[derive(Debug, Deserialize)]
pub struct GenshinClaimDaily {}
impl ClaimDaily for GenshinClaimDaily {}
impl GameIdentification for GenshinClaimDaily {
  fn game() -> Game {
    Game::Genshin
  }
}
