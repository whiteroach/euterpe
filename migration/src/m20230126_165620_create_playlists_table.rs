use sea_orm_migration::prelude::*;
use super::m20220101_000001_create_user_table::Users;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Playlists::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Playlists::PlaylistId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Playlists::Title).string().not_null())
                    .col(ColumnDef::new(Playlists::Duration).time().not_null())
                    .foreign_key(sea_query::ForeignKey::create().name("user_id").from(Playlists::Table, Playlists::PlaylistId).to(Users::Table, Users::UserId)) 
                    .to_owned()
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Playlists::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Playlists {
    Table,
    PlaylistId,
    Title,
    Duration,
    UserId,
}
