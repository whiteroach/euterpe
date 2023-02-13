use sea_orm_migration::prelude::*;

use crate::{
    m20230126_164652_create_pictures_table::Pictures, m20230206_151400_create_albums_table::Albums,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AlbumPicture::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AlbumPicture::AlbumId).integer().not_null())
                    .col(ColumnDef::new(AlbumPicture::PictureId).integer().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("album_id")
                            .from(AlbumPicture::Table, AlbumPicture::AlbumId)
                            .to(Albums::Table, Albums::AlbumId),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("picture_id")
                            .from(AlbumPicture::Table, AlbumPicture::PictureId)
                            .to(Pictures::Table, Pictures::PictureId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AlbumPicture::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum AlbumPicture {
    Table,
    AlbumId,
    PictureId,
}
