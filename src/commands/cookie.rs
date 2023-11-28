use api::models::login_cookie::LoginCookie;
use log::error;
use poise::serenity_prelude::Color;

use crate::{errors::CommandError, types::Context};

/// ログインのためのCookieを設定します.
#[poise::command(slash_command)]
pub async fn cookie(
  context: Context<'_>,
  #[description = "ltoken"] ltoken: String,
  #[description = "ltuid"] ltuid: String,
) -> Result<(), CommandError> {
  let message = context.say("設定中...").await?;
  let database = context.data().database.clone();
  let login_cookie = LoginCookie::new(ltoken, ltuid);
  let user_id = context.author().id.0;
  let set_login_cookie = database.set_login_cookie(user_id, login_cookie).await;
  message
    .edit(context, |m| {
      m.embed(|e| {
        e.title("Cookieの設定");
        match set_login_cookie {
          Ok(database::results::SetCookieResult::Set) => e
            .color(Color::DARK_GREEN)
            .description("設定が完了しました."),
          Ok(database::results::SetCookieResult::AccountNotRegistered) => {
            e.color(Color::DARK_RED).description(
              "アカウントが登録されていません. /registerでユーザーIDを登録してください.",
            )
          }
          Err(err) => {
            error!("[cookie command] Database error: {}", err);
            e.color(Color::DARK_RED)
              .description(format!("登録に失敗しました. ({})", err))
          }
        }
      })
    })
    .await?;
  Ok(())
}
