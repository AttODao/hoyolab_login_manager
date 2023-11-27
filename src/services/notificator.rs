use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use database::HlmDatabase;
use poise::serenity_prelude::CacheAndHttp;
use scheduler::Scheduled;

pub struct Notificator {
  database: Arc<HlmDatabase>,
  cache_http: Arc<CacheAndHttp>,
}

impl Notificator {
  pub fn new(database: Arc<HlmDatabase>, cache_http: Arc<CacheAndHttp>) -> Self {
    Notificator {
      database,
      cache_http,
    }
  }
}

#[async_trait]
impl Scheduled for Notificator {
  async fn process(&self, interval: Duration) {}
}
