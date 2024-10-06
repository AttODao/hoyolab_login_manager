use serde::Deserialize;

use crate::{
  models::{
    common::daily::{ClaimDaily, DailyInfo},
    game_identification::GameIdentification,
  },
  types::Game,
};

#[derive(Debug, Deserialize)]
pub struct ZzzDailyInfo {
  pub to_tal_sign_day: i32,
  pub is_sign: bool,
  pub sign_cnt_missed: i32,
}
impl DailyInfo for ZzzDailyInfo {}
impl GameIdentification for ZzzDailyInfo {
  fn game() -> Game {
    Game::Zzz
  }
}

#[derive(Debug, Deserialize)]
pub struct ZzzClaimDaily {}
impl ClaimDaily for ZzzClaimDaily {}
impl GameIdentification for ZzzClaimDaily {
  fn game() -> Game {
    Game::Zzz
  }
}
