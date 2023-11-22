use hlm::{commands, config::CONFIG};
use poise::{
  samples::register_globally, serenity_prelude::GatewayIntents, Framework, FrameworkOptions,
};

extern crate hoyolab_login_manager as hlm;

#[tokio::main]
async fn main() {
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
        Ok(())
      })
    });
  framework.run().await.unwrap();
}
