use std::{sync::Arc, time::Duration};

use api::{
  daily::claim_daily,
  models::{genshin::daily::GenshinClaimDaily, starrail::daily::StarrailClaimDaily},
};
use async_trait::async_trait;
use chrono::{Local, NaiveTime};
use database::HlmDatabase;
use poise::serenity_prelude::{CacheAndHttp, UserId};
use scheduler::Scheduled;

use crate::config::CONFIG;

pub struct DailyClaimer {
  database: Arc<HlmDatabase>,
  cache_http: Arc<CacheAndHttp>,
  claim_time: NaiveTime,
}

impl DailyClaimer {
  pub fn new(
    database: Arc<HlmDatabase>,
    cache_http: Arc<CacheAndHttp>,
    claim_time: NaiveTime,
  ) -> Self {
    DailyClaimer {
      database,
      cache_http,
      claim_time,
    }
  }
}

#[async_trait]
impl Scheduled for DailyClaimer {
  async fn process(&self, interval: Duration) {
    let now = Local::now().time();
    let to = (self.claim_time - now
      + if self.claim_time > now {
        chrono::Duration::zero()
      } else {
        chrono::Duration::days(1)
      })
    .to_std()
    .unwrap();
    if to < interval {
      tokio::time::sleep(to).await;
      for user in self
        .database
        .get_claim_daily_users()
        .await
        .unwrap_or(vec![])
      {
        if let Some(login_cookie) = user.login_cookie() {
          let genshin_message = if let Some(_) = user.genshin_id {
            match claim_daily::<GenshinClaimDaily>(login_cookie.clone(), CONFIG.lang.clone()).await
            {
              Ok(json) => json.message,
              Err(err) => err.to_string(),
            }
          } else {
            "idが登録されていません.".to_string()
          };
          let starrail_message = if let Some(_) = user.starrail_id {
            match claim_daily::<StarrailClaimDaily>(login_cookie.clone(), CONFIG.lang.clone()).await
            {
              Ok(json) => json.message,
              Err(err) => err.to_string(),
            }
          } else {
            "idが登録されていません.".to_string()
          };
          if let Ok(user) = UserId(user.discord_id as u64)
            .to_user(self.cache_http.clone())
            .await
          {
            user
              .direct_message(self.cache_http.clone(), |m| {
                m.content(&format!(
                  "ログインボーナス受け取り\n原神: {}\nスターレイル: {}",
                  genshin_message, starrail_message
                ))
              })
              .await;
          }
        }
      }
    }
  }
}
