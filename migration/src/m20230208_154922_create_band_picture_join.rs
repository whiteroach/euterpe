use sea_orm_migration::prelude::*;

use crate::{m20230204_232031_create_bands_table::Bands, m20230126_164652_create_pictures_table::Pictures};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BandPicture::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BandPicture::BandId).integer().not_null())
                    .col(ColumnDef::new(BandPicture::PictureId).integer().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("band_id")
                            .from(BandPicture::Table, BandPicture::BandId)
                            .to(Bands::Table, Bands::BandId),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("picture_id")
                            .from(BandPicture::Table, BandPicture::PictureId)
                            .to(Pictures::Table, Pictures::PictureId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BandPicture::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum BandPicture {
    Table,
    BandId,
    PictureId,
}
