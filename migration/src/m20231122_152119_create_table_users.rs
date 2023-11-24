use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(Users::Table)
          .if_not_exists()
          .col(
            ColumnDef::new(Users::DiscordId)
              .big_integer()
              .not_null()
              .primary_key(),
          )
          .col(ColumnDef::new(Users::GenshinId).char_len(9))
          .col(ColumnDef::new(Users::StarrailId).char_len(9))
          .col(ColumnDef::new(Users::Ltoken).char_len(40))
          .col(ColumnDef::new(Users::Ltuid).char_len(9))
          .col(
            ColumnDef::new(Users::ClaimDaily)
              .boolean()
              .not_null()
              .default(false),
          )
          .col(
            ColumnDef::new(Users::Notify)
              .boolean()
              .not_null()
              .default(false),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Users::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum Users {
  Table,
  DiscordId,
  GenshinId,
  StarrailId,
  Ltoken,
  Ltuid,
  ClaimDaily,
  Notify,
}
