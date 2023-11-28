use std::{sync::Arc, time::Duration};

use api::{
  daily::claim_daily,
  errors::NetworkError,
  models::{
    genshin::daily::GenshinClaimDaily, login_cookie::LoginCookie,
    starrail::daily::StarrailClaimDaily,
  },
};
use async_trait::async_trait;
use chrono::{Local, NaiveTime};
use database::HlmDatabase;
use futures::TryFutureExt;
use log::{error, info};
use poise::serenity_prelude::{CacheAndHttp, Color, UserId};
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
        .unwrap_or_else(|err| {
          error!("[ClaimDaily] get_claim_daily_users error: {}", err);
          vec![]
        })
      {
        info!("[ClaimDaily] On time");
        let login_cookie = user
          .login_cookie()
          .unwrap_or(LoginCookie::new("".to_string(), "".to_string()));
        let claim_daily = async {
          let mut claim = (None, None);
          if let Some(_) = user.genshin_id {
            claim.0 = Some(
              claim_daily::<GenshinClaimDaily>(login_cookie.clone(), CONFIG.lang.clone()).await?,
            );
          }
          if let Some(_) = user.starrail_id {
            claim.1 = Some(
              claim_daily::<StarrailClaimDaily>(login_cookie.clone(), CONFIG.lang.clone()).await?,
            );
          }
          Ok::<_, NetworkError>(claim)
        }
        .await;
        UserId(user.discord_id as u64)
          .to_user(self.cache_http.clone())
          .and_then(|user| async move {
            user
              .direct_message(self.cache_http.clone(), |m| {
                m.embed(|e| {
                  e.title("自動ログインボーナス受け取り");
                  match claim_daily {
                    Ok(claim_daily) => {
                      e.color(Color::DARK_GREEN);
                      if let Some(genshin_claim_daily) = claim_daily.0 {
                        match genshin_claim_daily.data {
                          Some(_) => {
                            e.description("受け取りが完了しました.");
                          }
                          None => {
                            e.color(Color::DARK_RED).description(format!(
                              "受け取りに失敗しました. ({})",
                              genshin_claim_daily.message
                            ));
                          }
                        }
                      }
                      if let Some(starrail_claim_daily) = claim_daily.1 {
                        match starrail_claim_daily.data {
                          Some(_) => {
                            e.description("受け取りが完了しました.");
                          }
                          None => {
                            e.color(Color::DARK_RED).description(format!(
                              "受け取りに失敗しました. ({})",
                              starrail_claim_daily.message
                            ));
                          }
                        }
                      }
                      e
                    }
                    Err(err) => {
                      error!("[ClaimaDaily] network error: {}", err);
                      e.color(Color::DARK_RED)
                        .description(format!("受け取りに失敗しました. ({})", err))
                    }
                  }
                })
              })
              .await?;
            Ok(())
          })
          .await
          .unwrap_or_else(|err| error!("[ClaimDaily] claim error: {}", err));
      }
    }
  }
}
