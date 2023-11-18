use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserStats {}

#[derive(Debug, Deserialize)]
pub struct DailyNote {
  current_resin: i32,
  max_resin: i32,
  resin_recovery_time: String,
  finished_task_num: i32,
  total_task_num: i32,
  is_extra_task_reward_received: bool,
  remain_resin_discount_num: i32,
  resin_discount_num_limit: i32,
  current_expedition_num: i32,
  max_expedition_num: i32,
  expeditions: Vec<Expedition>,
  current_home_coin: i32,
  max_home_coin: i32,
  home_coin_recovery_time: String,
}

#[derive(Debug, Deserialize)]
pub struct Expedition {
  avatar_side_icon: String,
  status: String,
  remained_time: String,
}
