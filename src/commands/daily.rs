use api::{
  daily::{claim_daily, get_daily_info},
  errors::NetworkError,
  models::{
    genshin::daily::{GenshinClaimDaily, GenshinDailyInfo},
    starrail::daily::{StarrailClaimDaily, StarrailDailyInfo},
  },
};
use poise::serenity_prelude::Color;

use crate::{commands::choices::GameChoice, config::CONFIG, errors::CommandError, types::Context};

use super::choices::SetChoice;

#[poise::command(
  slash_command,
  subcommands("set", "info", "claim"),
  subcommand_required
)]
pub async fn daily(_: Context<'_>) -> Result<(), CommandError> {
  Ok(())
}

/// 自動ログインボーナス受け取りを設定する.
#[poise::command(slash_command)]
pub async fn set(
  context: Context<'_>,
  #[description = "有効・無効の設定"] setting: SetChoice,
) -> Result<(), CommandError> {
  let message = context
    .send(|r| {
      r.embed(|e| {
        e.color(Color::LIGHT_GREY)
          .title("ログインボーナス")
          .description("設定中...")
      })
    })
    .await?;
  let database = context.data().database.clone();
  let set_claim_daily = database
    .set_claim_daily(context.author().id.0, setting.to_bool())
    .await;
  message
    .edit(context, |m| {
      m.embed(|e| {
        e.title("ログインボーナス");
        match set_claim_daily {
          Ok(database::results::SettingResult::Set) => e
            .color(Color::DARK_GREEN)
            .description("設定が完了しました."),
          Ok(database::results::SettingResult::CookieNotSet) => e
            .color(Color::DARK_RED)
            .description("Cookieが設定されていません. /cookieで設定してください."),
          Ok(database::results::SettingResult::AccountNotRegistered) => {
            e.color(Color::DARK_RED).description(
              "アカウントが登録されていません. /registerでユーザーIDを登録してください.",
            )
          }
          Err(err) => e
            .color(Color::DARK_RED)
            .description(format!("登録に失敗しました. ({})", err)),
        }
      })
    })
    .await?;
  Ok(())
}

/// ログインボーナス情報を確認する.
#[poise::command(slash_command)]
pub async fn info(
  context: Context<'_>,
  #[description = " ゲームの選択"] game: Option<GameChoice>,
) -> Result<(), CommandError> {
  let message = context
    .send(|r| {
      r.embed(|e| {
        e.color(Color::LIGHT_GREY)
          .title("ログインボーナス")
          .description("確認中...")
      })
    })
    .await?;
  let database = context.data().database.clone();
  let daily_info = match database.get_user(context.author().id.0).await {
    Ok(Some(database_user)) => Ok(Some(match database_user.login_cookie() {
      Some(login_cookie) => Some(
        async {
          let mut daily_info = (None, None);
          if !matches!(game, Some(GameChoice::Starrail)) && database_user.genshin_id.is_some() {
            daily_info.0 = Some(
              get_daily_info::<GenshinDailyInfo>(login_cookie.clone(), CONFIG.lang.clone()).await?,
            );
          }
          if !matches!(game, Some(GameChoice::Genshin)) && database_user.starrail_id.is_some() {
            daily_info.1 = Some(
              get_daily_info::<StarrailDailyInfo>(login_cookie.clone(), CONFIG.lang.clone())
                .await?,
            );
          }
          Ok::<_, NetworkError>((database_user, daily_info))
        }
        .await,
      ),
      None => None,
    })),
    Ok(None) => Ok(None),
    Err(err) => Err(err),
  };
  message
    .edit(context, |m| {
      m.embed(|e| {
        e.title("ログインボーナス");
        match daily_info {
          Ok(Some(Some(Ok(daily_info)))) => {
            if matches!(daily_info.1 .0, None) && matches!(daily_info.1 .1, None) {
              e.color(Color::DARK_RED).description(
                "ユーザーIDが登録されていません. /registerでユーザーIDを登録してください.",
              )
            } else {
              e.color(Color::DARK_GREEN).description(format!(
                "自動ログインボーナス受け取り: {}",
                if daily_info.0.claim_daily {
                  "有効"
                } else {
                  "無効"
                }
              ));
              if let Some(genshin_daily_info) = daily_info.1 .0 {
                e.field(
                  "原神",
                  match genshin_daily_info.data {
                    Some(genshin_daily_info) => format!(
                      "本日のログインボーナス: {}\n今月のログイン日数: {}",
                      if genshin_daily_info.is_sign {
                        "取得済み"
                      } else {
                        "未取得"
                      },
                      genshin_daily_info.total_sign_day
                    ),
                    None => format!("取得に失敗しました. ({})", genshin_daily_info.message),
                  },
                  true,
                );
              };
              if let Some(starrail_daily_info) = daily_info.1 .1 {
                e.field(
                  "スターレイル",
                  match starrail_daily_info.data {
                    Some(starrail_daily_info) => format!(
                      "本日のログインボーナス: {}\n今月のログイン日数: {}",
                      if starrail_daily_info.is_sign {
                        "取得済み"
                      } else {
                        "未取得"
                      },
                      starrail_daily_info.total_sign_day
                    ),
                    None => format!("取得に失敗しました. ({})", starrail_daily_info.message),
                  },
                  true,
                );
              };
              e
            }
          }
          Ok(Some(Some(Err(err)))) => e
            .color(Color::DARK_RED)
            .description(format!("取得に失敗しました. ({})", err)),
          Ok(Some(None)) => e
            .color(Color::DARK_RED)
            .description("Cookieが設定されていません. /cookieで設定してください."),
          Ok(None) => e.color(Color::DARK_RED).description(
            "アカウントが登録されていません. /registerでユーザーIDを登録してください.",
          ),
          Err(err) => e
            .color(Color::DARK_RED)
            .description(format!("取得に失敗しました. ({})", err)),
        }
      })
    })
    .await?;
  Ok(())
}

/// ログインボーナスを受け取る.
#[poise::command(slash_command)]
pub async fn claim(context: Context<'_>) -> Result<(), CommandError> {
  let message = context
    .send(|r| {
      r.embed(|e| {
        e.color(Color::LIGHT_GREY)
          .title("ログインボーナス")
          .description("受け取り中...")
      })
    })
    .await?;
  let database = context.data().database.clone();
  let claim_daily = match database.get_user(context.author().id.0).await {
    Ok(Some(database_user)) => Ok(Some(match database_user.login_cookie() {
      Some(login_cookie) => Some(
        async {
          let mut claim = (None, None);
          if matches!(database_user.genshin_id, Some(_)) {
            claim.0 = Some(
              claim_daily::<GenshinClaimDaily>(login_cookie.clone(), CONFIG.lang.clone()).await?,
            );
          }
          if matches!(database_user.starrail_id, Some(_)) {
            claim.1 = Some(
              claim_daily::<StarrailClaimDaily>(login_cookie.clone(), CONFIG.lang.clone()).await?,
            );
          }
          Ok::<_, NetworkError>(claim)
        }
        .await,
      ),
      None => None,
    })),
    Ok(None) => Ok(None),
    Err(err) => Err(err),
  };
  message
    .edit(context, |m| {
      m.embed(|e| {
        e.title("ログインボーナス");
        match claim_daily {
          Ok(Some(Some(Ok(claim_daily)))) => {
            e.color(Color::DARK_GREEN);
            if let Some(genshin_claim_daily) = claim_daily.0 {
              e.field(
                "原神",
                match genshin_claim_daily.data {
                  Some(genshin_claim_daily) => "受け取りが完了しました.".to_string(),
                  None => format!("受け取りに失敗しました. ({})", genshin_claim_daily.message),
                },
                true,
              );
            }
            if let Some(starrail_claim_daily) = claim_daily.1 {
              e.field(
                "スターレイル",
                match starrail_claim_daily.data {
                  Some(starrail_claim_daily) => "受け取りが完了しました.".to_string(),
                  None => format!("受け取りに失敗しました. ({})", starrail_claim_daily.message),
                },
                true,
              );
            }
            e
          }
          Ok(Some(Some(Err(err)))) => e
            .color(Color::DARK_RED)
            .description(format!("取得に失敗しました. ({})", err)),
          Ok(Some(None)) => e
            .color(Color::DARK_RED)
            .description("Cookieが設定されていません. /cookieで設定してください."),
          Ok(None) => e.color(Color::DARK_RED).description(
            "アカウントが登録されていません. /registerでユーザーIDを登録してください.",
          ),
          Err(err) => e
            .color(Color::DARK_RED)
            .description(format!("受け取りに失敗しました. ({})", err)),
        }
      })
    })
    .await?;
  Ok(())
}
