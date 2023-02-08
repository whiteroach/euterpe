use sea_orm_migration::prelude::*;

use crate::{m20230130_153942_create_labels_table::Labels, m20220101_000001_create_users_table::Users};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserLabel::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserLabel::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserLabel::UserId).integer().not_null())
                    .col(ColumnDef::new(UserLabel::LabelId).integer().not_null())
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("user_id")
                            .from(UserLabel::Table, UserLabel::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .foreign_key(
                        sea_query::ForeignKey::create()
                            .name("label_id")
                            .from(UserLabel::Table, UserLabel::LabelId)
                            .to(Labels::Table, Labels::LabelId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserLabel::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum UserLabel {
    Table,
    Id,
    UserId,
    LabelId,
}
