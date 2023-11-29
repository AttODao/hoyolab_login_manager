pub mod entities;

mod database;
pub mod results;

pub use database::HlmDatabase;

#[cfg(test)]
mod tests {
  use api::models::login_cookie::LoginCookie;

  use crate::database::HlmDatabase;

  #[tokio::test]
  async fn test_insert() {
    let db = HlmDatabase::connect("postgres://postgres:Atsu05012878@localhost/hlm")
      .await
      .unwrap();
    db.close().await.unwrap();
  }

  #[tokio::test]
  async fn test_notify_users() {
    let db = HlmDatabase::connect("postgres://postgres:Atsu05012878@localhost/hlm")
      .await
      .unwrap();
    db.set_game_id(
      api::types::Game::Genshin,
      123456789,
      "840787996".to_string(),
    )
    .await
    .unwrap();
    db.set_login_cookie(
      123456789,
      LoginCookie::new("hoge".to_string(), "huga".to_string()),
    )
    .await
    .unwrap();
    db.set_notify(123456789, true).await.unwrap();
    let users = db.get_notify_users().await.unwrap();
    println!("{:?}", users[0]);
    db.close().await.unwrap();
  }
}
