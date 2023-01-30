use super::m20230126_165306_create_genres_table::Genres;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Tracks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tracks::TrackId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tracks::Title).string().not_null())
                    .col(ColumnDef::new(Tracks::Duration).time().not_null())
                    .col(ColumnDef::new(Tracks::LinkAudio).string())
                    .col(ColumnDef::new(Tracks::Bpm).integer())
                    .col(ColumnDef::new(Tracks::DeletedAt).timestamp())
                    .col(ColumnDef::new(Tracks::GenreId).integer())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("genre_id")
                            .from(Tracks::Table, Tracks::GenreId)
                            .to(Genres::Table, Genres::GenreId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Tracks::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Tracks {
    Table,
    TrackId,
    Title,
    Duration,
    LinkAudio,
    Bpm,
    DeletedAt,
    GenreId,
}
