use sea_orm_migration::prelude::*;

use crate::{
    m20230130_153942_create_labels_table::Labels, m20230204_232031_create_bands_table::Bands,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BandLabel::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(BandLabel::BandId).integer().not_null())
                    .col(ColumnDef::new(BandLabel::LabelId).integer().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("band_id")
                            .from(BandLabel::Table, BandLabel::BandId)
                            .to(Bands::Table, Bands::BandId),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("label_id")
                            .from(BandLabel::Table, BandLabel::LabelId)
                            .to(Labels::Table, Labels::LabelId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BandLabel::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum BandLabel {
    Table,
    BandId,
    LabelId,
}
