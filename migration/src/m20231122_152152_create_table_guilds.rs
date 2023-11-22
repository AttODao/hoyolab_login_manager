use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Guilds::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Guilds::GuildId)
              .big_unsigned()
              .not_null()
              .primary_key(),
          )
          .col(ColumnDef::new(Guilds::ChannelId).big_unsigned().not_null())
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Guilds::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Guilds {
  Table,
  GuildId,
  ChannelId,
}
