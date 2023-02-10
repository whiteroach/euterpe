use sea_orm_migration::prelude::*;

use crate::{m20220101_000001_create_users_table::Users, m20230126_164652_create_pictures_table::Pictures};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserPicture::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserPicture::UserId).integer().not_null())
                    .col(ColumnDef::new(UserPicture::PictureId).integer().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("user_id")
                            .from(UserPicture::Table, UserPicture::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("picture_id")
                            .from(UserPicture::Table, UserPicture::PictureId)
                            .to(Pictures::Table, Pictures::PictureId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPicture::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum UserPicture {
    Table,
    UserId,
    PictureId,
}
