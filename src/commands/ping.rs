use poise::Context;

use crate::errors::CommandError;

#[poise::command(slash_command)]
pub async fn ping(context: Context<'_, (), CommandError>) -> Result<(), CommandError> {
  context.reply("Pong!").await?;
  Ok(())
}
