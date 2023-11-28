use log::error;
use poise::serenity_prelude::Color;

use crate::{errors::CommandError, types::Context};

use super::choices::SetChoice;

#[poise::command(slash_command, subcommands("set", "info"), subcommand_required)]
pub async fn notify(_: Context<'_>) -> Result<(), CommandError> {
  Ok(())
}

/// 通知の有効・無効を設定する.
#[poise::command(slash_command)]
pub async fn set(
  context: Context<'_>,
  #[description = "有効・無効の設定"] setting: SetChoice,
) -> Result<(), CommandError> {
  let message = context
    .send(|r| {
      r.embed(|e| {
        e.color(Color::LIGHT_GREY)
          .title("通知")
          .description("設定中...")
      })
    })
    .await?;
  let database = context.data().database.clone();
  let set_notify = database
    .set_notify(context.author().id.0, setting.to_bool())
    .await;
  message
    .edit(context, |m| {
      m.embed(|e| {
        e.title("通知");
        match set_notify {
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
          Err(err) => {
            error!("[notify set command] Database error: {}", err);
            e.colour(Color::DARK_RED)
              .description(format!("登録に失敗しました. ({})", err))
          }
        }
      })
    })
    .await?;
  Ok(())
}

/// 通知の有効・無効を確認する.
#[poise::command(slash_command)]
pub async fn info(context: Context<'_>) -> Result<(), CommandError> {
  let message = context
    .send(|r| {
      r.embed(|e| {
        e.color(Color::LIGHT_GREY)
          .title("通知")
          .description("確認中...")
      })
    })
    .await?;
  let database = context.data().database.clone();
  let database_user = database.get_user(context.author().id.0).await;
  message
    .edit(context, |m| {
      m.embed(|e| {
        e.title("通知");
        match database_user {
          Ok(database_user) => match database_user {
            Some(database_user) => match database_user.login_cookie() {
              Some(_) => e.color(Color::DARK_GREEN).description(format!(
                "通知: {}",
                if database_user.notify {
                  "有効"
                } else {
                  "無効"
                }
              )),
              None => e
                .color(Color::DARK_RED)
                .description("Cookieが設定されていません. /cookieで設定してください."),
            },
            None => e.color(Color::DARK_RED).description(
              "アカウントが登録されていません. /registerでユーザーIDを登録してください.",
            ),
          },
          Err(err) => {
            error!("[notify info command] Database error: {}", err);
            e.colour(Color::DARK_RED)
              .description(format!("取得に失敗しました. ({})", err))
          }
        }
      })
    })
    .await?;
  Ok(())
}
