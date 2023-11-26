use std::{sync::Arc, time::Duration};

use api::{
  daily::claim_daily,
  models::{
    genshin::daily::GenshinClaimDaily, login_cookie::LoginCookie,
    starrail::daily::StarrailClaimDaily,
  },
};
use async_trait::async_trait;
use chrono::{Local, NaiveTime};
use database::database::HlmDatabase;
use scheduler::Scheduled;

use crate::{config::CONFIG, types::Context};

pub struct DailyClaimer<'a> {
  database: Arc<HlmDatabase>,
  context: Context<'a>,
  claim_time: NaiveTime,
}

impl<'a> DailyClaimer<'a> {
  pub fn new(database: Arc<HlmDatabase>, context: Context<'a>, claim_time: NaiveTime) -> Self {
    DailyClaimer {
      database,
      context,
      claim_time,
    }
  }
}

#[async_trait]
impl<'a> Scheduled for DailyClaimer<'a> {
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
        let login_cookie = LoginCookie::new(
          user.ltoken.unwrap_or("".to_string()),
          user.ltuid.unwrap_or("".to_string()),
        );
        let genshin_message = if let Some(_) = user.genshin_id {
          match claim_daily::<GenshinClaimDaily>(login_cookie.clone(), CONFIG.lang.clone()).await {
            Ok(json) => json.message,
            Err(err) => err.to_string(),
          }
        } else {
          "idが登録されていません.".to_string()
        };
        let starrail_message = if let Some(_) = user.starrail_id {
          match claim_daily::<StarrailClaimDaily>(login_cookie.clone(), CONFIG.lang.clone()).await {
            Ok(json) => json.message,
            Err(err) => err.to_string(),
          }
        } else {
          "idが登録されていません.".to_string()
        };
      }
    }
  }
}
