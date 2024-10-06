//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use api::models::login_cookie::LoginCookie;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub discord_id: i64,
  pub genshin_id: Option<String>,
  pub starrail_id: Option<String>,
  pub zzz_id: Option<String>,
  pub ltoken: Option<String>,
  pub ltuid: Option<String>,
  pub claim_daily: bool,
  pub notify: bool,
}

impl Model {
  pub fn login_cookie(&self) -> Option<LoginCookie> {
    Some(LoginCookie {
      ltoken: self.ltoken.clone()?,
      ltuid: self.ltuid.clone()?,
    })
  }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
