use sea_orm_migration::prelude::*;

use crate::{m20230130_153942_create_labels_table::Labels, m20230126_165306_create_genres_table::Genres, m20220101_000001_create_users_table::Users};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Albums::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Albums::AlbumId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Albums::Title).string().not_null())
                    .col(ColumnDef::new(Albums::Type).string())
                    .col(ColumnDef::new(Albums::Duration).time())
                    .col(ColumnDef::new(Albums::Info).json())
                    .col(ColumnDef::new(Albums::Year).date())
                    .col(ColumnDef::new(Albums::DeletedAt).timestamp())
                    .col(ColumnDef::new(Albums::LabelId).integer())
                    .col(ColumnDef::new(Albums::GenreId).integer())
                    .col(ColumnDef::new(Albums::ArtistId).integer())
                    .foreign_key(sea_query::ForeignKey::create()
                        .name("label_id")
                        .from(Albums::Table, Albums::LabelId)
                        .to(Labels::Table, Labels::LabelId)
                    )
                    .foreign_key(sea_query::ForeignKey::create()
                        .name("genre_id")
                        .from(Albums::Table, Albums::GenreId)
                        .to(Genres::Table, Genres::GenreId)
                    )
                    .foreign_key(sea_query::ForeignKey::create()
                        .name("artist_id")
                        .from(Albums::Table, Albums::ArtistId)
                        .to(Users::Table, Users::UserId)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Albums::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Albums {
    Table,
    AlbumId,
    Title,
    Duration,
    Type,
    Info,
    Year,
    DeletedAt,
    LabelId,
    GenreId,
    ArtistId,
}
