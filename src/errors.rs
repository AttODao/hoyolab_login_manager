use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
  #[error("Serenity: {0}")]
  SerenityError(#[from] poise::serenity_prelude::Error),
}
