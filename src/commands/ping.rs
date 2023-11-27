use crate::{errors::CommandError, types::Context};

/// Reply "Pong!"
#[poise::command(slash_command)]
pub async fn ping(context: Context<'_>) -> Result<(), CommandError> {
  context
    .reply(format!("Pong! (user id: {})", context.author().id.0))
    .await?;
  context
    .author()
    .direct_message(context, |m| {
      m.content("test").tts(true).embed(|e| {
        e.title("This is an embed")
          .description("With a description")
      })
    })
    .await?;
  Ok(())
}
