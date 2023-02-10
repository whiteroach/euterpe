use sea_orm_migration::prelude::*;

use crate::{m20230206_151400_create_albums_table::Albums, m20230130_132523_create_tracks_table::Tracks};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .create_table(
                Table::create()
                    .table(AlbumTrack::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AlbumTrack::AlbumId).integer().not_null())
                    .col(ColumnDef::new(AlbumTrack::TrackId).integer().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("album_id")
                            .from(AlbumTrack::Table, AlbumTrack::AlbumId)
                            .to(Albums::Table, Albums::AlbumId),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("track_id")
                            .from(AlbumTrack::Table, AlbumTrack::TrackId)
                            .to(Tracks::Table, Tracks::TrackId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        

        manager
            .drop_table(Table::drop().table(AlbumTrack::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum AlbumTrack {
    Table,
    AlbumId,
    TrackId,
}
