use sea_orm_migration::prelude::*;

use crate::{m20230126_165620_create_playlists_table::Playlists, m20230130_132523_create_tracks_table::Tracks};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PlaylistTrack::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PlaylistTrack::PlaylistId).integer().not_null())
                    .col(ColumnDef::new(PlaylistTrack::TrackId).integer().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("playlist_id")
                            .from(PlaylistTrack::Table, PlaylistTrack::PlaylistId)
                            .to(Playlists::Table, Playlists::PlaylistId),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("track_id")
                            .from(PlaylistTrack::Table, PlaylistTrack::TrackId)
                            .to(Tracks::Table, Tracks::TrackId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PlaylistTrack::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum PlaylistTrack {
    Table,
    PlaylistId,
    TrackId,
}