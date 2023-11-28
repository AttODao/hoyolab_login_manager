use log::error;
use poise::serenity_prelude::Color;

use crate::{errors::CommandError, types::Context};

use super::choices::GameChoice;

/// ゲームのユーザーIDを登録します.
#[poise::command(slash_command)]
pub async fn register(
  context: Context<'_>,
  #[description = "ゲームの選択"] game: GameChoice,
  #[description = "ユーザーID"] id: String,
) -> Result<(), CommandError> {
  let message = context
    .send(|r| {
      r.embed(|e| {
        e.color(Color::LIGHT_GREY)
          .title("ユーザーIDの登録")
          .description("登録中...")
      })
    })
    .await?;
  let set_game_id = context
    .data()
    .database
    .clone()
    .set_game_id(game.to_game(), context.author().id.0, id)
    .await;
  message
    .edit(context, |m| {
      m.embed(|e| {
        e.title("ユーザーIDの登録");
        match set_game_id {
          Ok(database::results::RegisterResult::Registered) => e
            .color(Color::DARK_GREEN)
            .description("登録が完了しました."),
          Ok(database::results::RegisterResult::Updated) => e
            .color(Color::DARK_GREEN)
            .description("ユーザーIDが変更されました."),
          Err(err) => {
            error!("[register command] Database error: {}", err);
            e.color(Color::DARK_RED)
              .description(format!("登録に失敗しました. ({})", err))
          }
        }
      })
    })
    .await?;

  Ok(())
}
