pub mod models;

mod api;
pub mod daily;
pub mod errors;
pub mod game_record;
pub mod types;
pub mod utils;

#[cfg(test)]
mod tests {
  use crate::{
    daily::{claim_daily, get_daily_info},
    game_record::{get_daily_note, get_user_stats},
    models::{
      genshin::game_record::GenshinDailyNote,
      login_cookie::LoginCookie,
      starrail::game_record::StarrailDailyNote,
      zzz::{
        daily::{ZzzClaimDaily, ZzzDailyInfo},
        game_record::{ZzzDailyNote, ZzzUserStats},
      },
    },
  };

  #[tokio::test]
  async fn game_record_test() {
    let login_cookie = LoginCookie::new(
      "Ft3meFr0ZKxUnAM1MKNkhNjQfLDIunpTeJZ5vi2H".to_string(),
      "166032325".to_string(),
    );
    // println!(
    //   "{:?}",
    //   get_daily_note::<GenshinDailyNote>(
    //     "840787996".to_string(),
    //     login_cookie.clone(),
    //     "ja-jp".to_string()
    //   )
    //   .await
    //   .unwrap()
    // );
    // println!(
    //   "{:?}",
    //   get_daily_note::<StarrailDailyNote>(
    //     "800746144".to_string(),
    //     login_cookie,
    //     "ja-jp".to_string()
    //   )
    //   .await
    //   .unwrap()
    // );
    println!(
      "{:?}",
      claim_daily::<ZzzClaimDaily>(login_cookie.clone(), "ja-jp".to_string())
        .await
        .unwrap()
    );
    println!(
      "{:?}",
      get_daily_note::<ZzzDailyNote>(
        "1309455489".to_string(),
        login_cookie.clone(),
        "ja-jp".to_string()
      )
      .await
      .unwrap()
    );
  }
}
