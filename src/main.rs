use std::{sync::Arc, time::Duration};

use database::HlmDatabase;
use hlm::{
  commands,
  config::CONFIG,
  errors::CommandError,
  services::{daily_claimer::DailyClaimer, notificator::Notificator},
  types::Data,
};
use log::{info, warn};
use poise::{
  samples::register_globally, serenity_prelude::GatewayIntents, Framework, FrameworkOptions,
};
use scheduler::Scheduler;

extern crate hoyolab_login_manager as hlm;

async fn on_error(error: poise::FrameworkError<'_, Data, CommandError>) {
  match error {
    poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
    poise::FrameworkError::Command { error, ctx, .. } => {
      println!("Error in command `{}`: {:?}", ctx.command().name, error,);
    }
    error => {
      if let Err(e) = poise::builtins::on_error(error).await {
        println!("Error while handling error: {}", e)
      }
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));

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
      on_error: |error| Box::pin(on_error(error)),
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
  let scheduler = Scheduler::from_scheduleds(vec![
    Box::new(DailyClaimer::new(
      database.clone(),
      cache_http.clone(),
      CONFIG.claim_daily_time,
    )),
    Box::new(Notificator::new(database.clone(), cache_http.clone())),
  ]);
  info!("Starting scheduler");
  scheduler.run(Duration::from_secs(60 * CONFIG.scheduler_interval_mins));

  info!("Starting bot");
  let mut client = framework.client();
  let shard_manager = client.shard_manager.clone();

  tokio::spawn(async move {
    #[cfg(unix)]
    {
      use tokio::signal::unix as signal;

      let [mut s1, mut s2, mut s3] = [
        signal::signal(signal::SignalKind::hangup()).unwrap(),
        signal::signal(signal::SignalKind::interrupt()).unwrap(),
        signal::signal(signal::SignalKind::terminate()).unwrap(),
      ];

      tokio::select!(
          v = s1.recv() => v.unwrap(),
          v = s2.recv() => v.unwrap(),
          v = s3.recv() => v.unwrap(),
      );
    }
    #[cfg(windows)]
    {
      let (mut s1, mut s2) = (
        tokio::signal::windows::ctrl_c().unwrap(),
        tokio::signal::windows::ctrl_break().unwrap(),
      );

      tokio::select!(
          v = s1.recv() => v.unwrap(),
          v = s2.recv() => v.unwrap(),
      );
    }

    warn!("Recieved control C and shutting down.");
    shard_manager.lock().await.shutdown_all().await;
  });

  client.start_autosharded().await.map_err(Into::into)
}
