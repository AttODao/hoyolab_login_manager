pub use sea_orm_migration::prelude::*;

mod m20231122_152119_create_table_users;
mod m20231122_152152_create_table_guilds;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
      Box::new(m20231122_152119_create_table_users::Migration),
      Box::new(m20231122_152152_create_table_guilds::Migration),
    ]
  }
}
