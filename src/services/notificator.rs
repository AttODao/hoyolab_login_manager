use std::{sync::Arc, time::Duration, vec};

use api::{
  game_record::get_daily_note,
  models::{genshin::game_record::GenshinDailyNote, starrail::game_record::StarrailDailyNote},
};
use async_trait::async_trait;
use database::HlmDatabase;
use log::error;
use poise::serenity_prelude::{CacheAndHttp, Color, UserId};
use scheduler::Scheduled;

use crate::config::CONFIG;

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
  async fn process(&self, interval: Duration) {
    for user in self
      .database
      .clone()
      .get_notify_users()
      .await
      .unwrap_or_else(|err| {
        error!("[Notificator] get_notify_users error: {}", err);
        vec![]
      })
    {
      let discord_user = UserId(user.discord_id as u64)
        .to_user(self.cache_http.clone())
        .await;
      let login_cookie = user.login_cookie();
      if let (Ok(discord_user), Some(login_cookie)) = (discord_user, login_cookie) {
        let discord_user = Arc::new(discord_user);

        //原神
        if let Some(genshin_id) = user.genshin_id {
          if let Ok(daily_notes) = get_daily_note::<GenshinDailyNote>(
            genshin_id,
            login_cookie.clone(),
            CONFIG.lang.clone(),
          )
          .await
          {
            if let Some(daily_notes) = daily_notes.data {
              // 樹脂
              for (notify_mins, notify_time) in CONFIG.genshin_resin_notify_mins.iter().map(|m| {
                (
                  m,
                  daily_notes
                    .resin_recovery_time
                    .saturating_sub(Duration::from_secs(60 * m)),
                )
              }) {
                if notify_time > Duration::ZERO && notify_time <= interval {
                  error!("resin: {}", discord_user.name);
                  let discord_user = discord_user.clone();
                  let cache_http = self.cache_http.clone();
                  tokio::spawn(async move {
                    tokio::time::sleep(notify_time).await;
                    if let Err(err) = discord_user
                      .direct_message(cache_http, |m| {
                        m.embed(|e| {
                          e.title("通知").color(Color::DARK_GREEN).description(
                            if notify_mins > &0 {
                              let notify_mins = notify_mins;
                              format!("【原神】樹脂があふれるまで残り{}分です.", notify_mins)
                            } else {
                              "【原神】樹脂があふれました.".to_string()
                            },
                          )
                        })
                      })
                      .await
                    {
                      error!("[Notificator] notify error: {}", err);
                    }
                  });
                }
              }

              // 洞天宝銭
              for (notify_mins, notify_time) in
                CONFIG.genshin_home_coin_notify_mins.iter().map(|m| {
                  (
                    m,
                    daily_notes
                      .home_coin_recovery_time
                      .saturating_sub(Duration::from_secs(60 * m)),
                  )
                })
              {
                if notify_time > Duration::ZERO && notify_time <= interval {
                  error!("home_coin: {}", discord_user.name);
                  let discord_user = discord_user.clone();
                  let cache_http = self.cache_http.clone();
                  tokio::spawn(async move {
                    tokio::time::sleep(notify_time).await;
                    if let Err(err) = discord_user
                      .direct_message(cache_http, |m| {
                        m.embed(|e| {
                          e.title("通知").color(Color::DARK_GREEN).description(
                            if notify_mins > &0 {
                              let notify_mins = notify_mins;
                              format!("【原神】洞天宝銭があふれるまで残り{}分です.", notify_mins)
                            } else {
                              "【原神】洞天宝銭があふれました.".to_string()
                            },
                          )
                        })
                      })
                      .await
                    {
                      error!("[Notificator] notify error: {}", err);
                    }
                  });
                }
              }

              // 探索派遣
              for expedition in daily_notes.expeditions {
                if expedition.remained_time > Duration::ZERO && expedition.remained_time <= interval
                {
                  error!("expedition: {}", discord_user.name);
                  let discord_user = discord_user.clone();
                  let cache_http = self.cache_http.clone();
                  tokio::spawn(async move {
                    tokio::time::sleep(expedition.remained_time).await;
                    if let Err(err) = discord_user
                      .direct_message(cache_http, |m| {
                        m.embed(|e| {
                          e.title("通知")
                            .color(Color::DARK_GREEN)
                            .description("【原神】探索派遣が完了しました.")
                            .thumbnail(expedition.avatar_side_icon)
                        })
                      })
                      .await
                    {
                      error!("[Notificator] notify error: {}", err);
                    }
                  });
                }
              }
            }
          }
        }

        // スターレイル
        if let Some(starrail_id) = user.starrail_id {
          if let Ok(daily_notes) = get_daily_note::<StarrailDailyNote>(
            starrail_id,
            login_cookie.clone(),
            CONFIG.lang.clone(),
          )
          .await
          {
            if let Some(daily_notes) = daily_notes.data {
              // 開拓力
              for (notify_mins, notify_time) in
                CONFIG.starrail_stamina_notify_mins.iter().map(|m| {
                  (
                    m,
                    daily_notes
                      .stamina_recover_time
                      .saturating_sub(Duration::from_secs(60 * m)),
                  )
                })
              {
                if notify_time > Duration::ZERO && notify_time <= interval {
                  error!("stamina: {}", discord_user.name);
                  let discord_user = discord_user.clone();
                  let cache_http = self.cache_http.clone();
                  tokio::spawn(async move {
                    tokio::time::sleep(notify_time).await;
                    if let Err(err) = discord_user
                      .direct_message(cache_http, |m| {
                        m.embed(|e| {
                          e.title("通知").color(Color::DARK_GREEN).description(
                            if notify_mins > &0 {
                              let notify_mins = notify_mins;
                              format!(
                                "【スターレイル】開拓力があふれるまで残り{}分です.",
                                notify_mins
                              )
                            } else {
                              "【スターレイル】開拓力があふれました.".to_string()
                            },
                          )
                        })
                      })
                      .await
                    {
                      error!("[Notificator] notify error: {}", err);
                    }
                  });
                }
              }

              // 依頼
              for expedition in daily_notes.expeditions {
                if expedition.remaining_time > Duration::ZERO
                  && expedition.remaining_time <= interval
                {
                  error!("expedition: {}", discord_user.name);
                  let discord_user = discord_user.clone();
                  let cache_http = self.cache_http.clone();
                  tokio::spawn(async move {
                    tokio::time::sleep(expedition.remaining_time).await;
                    if let Err(err) = discord_user
                      .direct_message(cache_http, |m| {
                        m.embed(|e| {
                          e.title("通知")
                            .color(Color::DARK_GREEN)
                            .description(format!(
                              "【スターレイル】依頼({})が完了しました.",
                              expedition.name
                            ))
                            .thumbnail(expedition.item_url)
                        })
                      })
                      .await
                    {
                      error!("[Notificator] notify error: {}", err);
                    }
                  });
                }
              }
            }
          }
        }
      }
    }
  }
}
