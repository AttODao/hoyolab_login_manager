use serde::Deserialize;

use crate::{
  models::{
    common::daily::{ClaimDaily, DailyInfo},
    game_identification::GameIdentification,
  },
  types::Game,
};

#[derive(Debug, Deserialize)]
pub struct StarrailDailyInfo {
  total_sign_day: i32,
  is_sign: bool,
  sign_cnt_missed: i32,
}
impl DailyInfo for StarrailDailyInfo {}
impl GameIdentification for StarrailDailyInfo {
  fn game() -> Game {
    Game::Starrail
  }
}

#[derive(Debug, Deserialize)]
pub struct StarrailClaimDaily {}
impl ClaimDaily for StarrailClaimDaily {}
impl GameIdentification for StarrailClaimDaily {
  fn game() -> Game {
    Game::Starrail
  }
}
