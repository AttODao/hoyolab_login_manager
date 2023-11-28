use std::{sync::Arc, time::Duration};

use database::HlmDatabase;
use futures::FutureExt;
use hlm::{commands, config::CONFIG, services::daily_claimer::DailyClaimer, types::Data};
use log::info;
use poise::{
  samples::register_globally, serenity_prelude::GatewayIntents, Framework, FrameworkOptions,
};
use scheduler::Scheduler;
use tokio::signal;

extern crate hoyolab_login_manager as hlm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));

  info!("Initialize database");
  let database = Arc::new(HlmDatabase::connect(&CONFIG.database_url).await?);
  let mdatabase = database.clone();

  let framework = Framework::builder()
    .options(FrameworkOptions {
      commands: vec![
        commands::ping::ping(),
        commands::cookie::cookie(),
        commands::daily::daily(),
        commands::notify::notify(),
        commands::register::register(),
      ],
      ..Default::default()
    })
    .token(&CONFIG.discord_token)
    .intents(GatewayIntents::non_privileged())
    .setup(|context, _ready, framework| {
      Box::pin(async move {
        register_globally(context, &framework.options().commands).await?;
        Ok(Data {
          database: mdatabase,
        })
      })
    })
    .build()
    .await?;

  let cache_http = framework.client().cache_and_http.clone();
  let scheduler = Scheduler::from_scheduleds(vec![Box::new(DailyClaimer::new(
    database.clone(),
    cache_http.clone(),
    CONFIG.claim_daily_time,
  ))]);
  info!("Starting scheduler");
  scheduler.run(Duration::from_secs(60 * CONFIG.scheduler_interval_mins));

  info!("Starting bot");
  futures::select! {
    res = framework.start().fuse() => Ok(res?),
    ctrlc = signal::ctrl_c().fuse() => {
      if ctrlc.is_ok() {
        info!("ctrl_c");
      }
      Ok(ctrlc?)
    }
  }
}
