use hlm::models::{
    account_info::AccountInfo,
    genshin::{daily::GenshinClaimDaily, game_record::GenshinUserStats}, starrail::{daily::StarrailDailyInfo, game_record::StarrailDailyNote},
  };

extern crate hoyolab_login_manager as hlm;

#[tokio::main]
async fn main() {
  println!(
    "{:?}",
    hlm::networks::game_record::get_daily_note::<StarrailDailyNote>(
      "800746144".to_string(),
      AccountInfo::new(
        "Ft3meFr0ZKxUnAM1MKNkhNjQfLDIunpTeJZ5vi2H".to_string(),
        "166032325".to_string()
      ),
      "ja-jp".to_string()
    )
    .await
  );
  println!(
    "{:?}",
    hlm::networks::daily::claim_daily::<GenshinClaimDaily>(
      AccountInfo::new(
        "Ft3meFr0ZKxUnAM1MKNkhNjQfLDIunpTeJZ5vi2H".to_string(),
        "166032325".to_string()
      ),
      "ja-jp".to_string()
    )
    .await
  );
  println!(
    "{:?}",
    hlm::networks::daily::get_daily_info::<StarrailDailyInfo>(
      AccountInfo::new(
        "Ft3meFr0ZKxUnAM1MKNkhNjQfLDIunpTeJZ5vi2H".to_string(),
        "166032325".to_string()
      ),
      "ja-jp".to_string()
    )
    .await
  );
}
