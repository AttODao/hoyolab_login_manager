#![allow(overflowing_literals)]
use std::time::Duration;

use api::{models::login_cookie::LoginCookie, types::Game};
use migration::{Migrator, MigratorTrait};
use sea_orm::{
  ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DbConn, DbErr, EntityTrait, QueryFilter,
  Set,
};

use crate::{
  entities::users,
  results::{RegisterResult, SetResult},
};

pub struct HlmDatabase {
  pub conn: DbConn,
}

impl HlmDatabase {
  pub async fn connect(url: String) -> Result<Self, DbErr> {
    let mut opt = ConnectOptions::new(url);
    opt
      .max_connections(100)
      .min_connections(5)
      .connect_timeout(Duration::from_secs(8))
      .acquire_timeout(Duration::from_secs(8))
      .idle_timeout(Duration::from_secs(8))
      .max_lifetime(Duration::from_secs(8))
      .sqlx_logging(true)
      .sqlx_logging_level(log::LevelFilter::Info);
    let conn = Database::connect(opt).await?;

    Migrator::up(&conn, None).await?;

    Ok(HlmDatabase { conn })
  }

  pub async fn close(self) -> Result<(), DbErr> {
    Migrator::down(&self.conn, None).await?;
    self.conn.close().await?;
    Ok(())
  }

  pub async fn get_user(&self, discord_id: u64) -> Result<Option<users::Model>, DbErr> {
    users::Entity::find_by_id(discord_id as i64)
      .one(&self.conn)
      .await
  }

  pub async fn get_claim_daily_users(&self) -> Result<Vec<users::Model>, DbErr> {
    users::Entity::find()
      .filter(users::Column::ClaimDaily.eq(true))
      .all(&self.conn)
      .await
  }

  pub async fn get_notify_users(&self) -> Result<Vec<users::Model>, DbErr> {
    users::Entity::find()
      .filter(users::Column::Notify.eq(true))
      .all(&self.conn)
      .await
  }

  pub async fn set_game_id(
    &self,
    game: Game,
    discord_id: u64,
    game_id: String,
  ) -> Result<RegisterResult, DbErr> {
    if let Some(user) = users::Entity::find_by_id(discord_id as i64)
      .one(&self.conn)
      .await?
    {
      let not_set = match game {
        Game::Genshin => &user.genshin_id,
        Game::Starrail => &user.starrail_id,
      } == &None;
      let mut user: users::ActiveModel = user.into();
      match game {
        Game::Genshin => user.genshin_id = Set(Some(game_id)),
        Game::Starrail => user.starrail_id = Set(Some(game_id)),
      }
      user.update(&self.conn).await?;
      Ok(if not_set {
        RegisterResult::Registered
      } else {
        RegisterResult::Updated
      })
    } else {
      let mut user = users::ActiveModel {
        discord_id: Set(discord_id as i64),
        ..Default::default()
      };
      match game {
        Game::Genshin => user.genshin_id = Set(Some(game_id)),
        Game::Starrail => user.starrail_id = Set(Some(game_id)),
      }
      users::Entity::insert(user).exec(&self.conn).await?;
      Ok(RegisterResult::Registered)
    }
  }

  pub async fn set_login_cookie(
    &self,
    discord_id: u64,
    login_cookie: LoginCookie,
  ) -> Result<SetResult, DbErr> {
    if let Some(user) = users::Entity::find_by_id(discord_id as i64)
      .one(&self.conn)
      .await?
    {
      let mut user: users::ActiveModel = user.into();
      user.ltoken = Set(Some(login_cookie.ltoken));
      user.ltuid = Set(Some(login_cookie.ltuid));
      user.update(&self.conn).await?;
      Ok(SetResult::Set)
    } else {
      Ok(SetResult::AccountNotRegistered)
    }
  }

  pub async fn set_claim_daily(
    &self,
    discord_id: u64,
    claim_daily: bool,
  ) -> Result<SetResult, DbErr> {
    if let Some(user) = users::Entity::find_by_id(discord_id as i64)
      .one(&self.conn)
      .await?
    {
      let mut user: users::ActiveModel = user.into();
      user.claim_daily = Set(claim_daily);
      user.update(&self.conn).await?;
      Ok(SetResult::Set)
    } else {
      Ok(SetResult::AccountNotRegistered)
    }
  }

  pub async fn set_notify(&self, discord_id: u64, notify: bool) -> Result<SetResult, DbErr> {
    if let Some(user) = users::Entity::find_by_id(discord_id as i64)
      .one(&self.conn)
      .await?
    {
      let mut user: users::ActiveModel = user.into();
      user.notify = Set(notify);
      user.update(&self.conn).await?;
      Ok(SetResult::Set)
    } else {
      Ok(SetResult::AccountNotRegistered)
    }
  }
}
