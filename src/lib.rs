pub mod commands;
pub mod services;

pub mod config;
pub mod errors;
pub mod types;

#[cfg(test)]
mod tests {
  use std::time::Duration;

  use chrono::NaiveTime;

  #[test]
  fn time_test() {
    let from_hmsm = |h, m, s, milli| NaiveTime::from_hms_milli_opt(h, m, s, milli).unwrap();
    assert_eq!(
      (from_hmsm(3, 0, 0, 0) - from_hmsm(4, 0, 0, 0) + chrono::Duration::days(1))
        .to_std()
        .unwrap(),
      Duration::from_secs(3600 * 23)
    );
  }
}
