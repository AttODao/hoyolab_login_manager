use std::sync::Arc;

use database::HlmDatabase;

use crate::errors::CommandError;

pub type Context<'a> = poise::Context<'a, Data, CommandError>;

pub struct Data {
  pub database: Arc<HlmDatabase>,
}
