use sea_orm_migration::prelude::*;

use crate::{m20220101_000001_create_users_table::Users, m20230204_232031_create_bands_table::Bands};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BandUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BandUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BandUser::BandId).integer().not_null())
                    .col(ColumnDef::new(BandUser::UserId).integer().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("user_id")
                            .from(BandUser::Table, BandUser::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("band_id")
                            .from(BandUser::Table, BandUser::BandId)
                            .to(Bands::Table, Bands::BandId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BandUser::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum BandUser {
    Table,
    Id,
    BandId,
    UserId,
}
