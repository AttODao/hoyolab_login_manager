use std::sync::Arc;

use database::database::HlmDatabase;
use futures::FutureExt;
use hlm::{commands, config::CONFIG, types::Data};
use poise::{
  samples::register_globally, serenity_prelude::GatewayIntents, Framework, FrameworkOptions,
};
use tokio::signal;

extern crate hoyolab_login_manager as hlm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let database = Arc::new(HlmDatabase::connect(&CONFIG.database_url).await?);

  let framework = Framework::builder()
    .options(FrameworkOptions {
      commands: vec![commands::ping::ping()],
      ..Default::default()
    })
    .token(&CONFIG.discord_token)
    .intents(GatewayIntents::non_privileged())
    .setup(|context, _ready, framework| {
      Box::pin(async move {
        register_globally(context, &framework.options().commands).await?;
        Ok(Data {
          database: database.clone(),
        })
      })
    })
    .build()
    .await?;

  futures::select! {
    res = framework.start().fuse() => Ok(res?),
    ctrlc = signal::ctrl_c().fuse() => {
      if ctrlc.is_ok() {
        println!("ctrl_c");
      }
      Ok(ctrlc?)
    }
  }
}
