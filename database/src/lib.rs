pub mod entities;

pub mod database;
pub mod results;

#[cfg(test)]
mod tests {
  use crate::database::HlmDatabase;

  #[tokio::test]
  async fn test_insert() {
    let db = HlmDatabase::connect("postgres://postgres:Atsu05012878@localhost/hlm".to_string())
      .await
      .unwrap();
    db.close().await.unwrap();
  }
}
