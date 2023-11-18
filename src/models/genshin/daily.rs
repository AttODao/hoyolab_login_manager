use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DailyInfo {
  pub is_sign: bool,
  pub total_sign_day: i32,
}

#[derive(Debug, Deserialize)]
pub struct ClaimDaily {}
