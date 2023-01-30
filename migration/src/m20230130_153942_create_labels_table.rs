use sea_orm_migration::prelude::*;

use crate::m20230126_165306_create_genres_table::Genres;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Labels::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Labels::LabelId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Labels::Name).string().not_null())
                    .col(ColumnDef::new(Labels::Info).json())
                    .col(ColumnDef::new(Labels::Owned).boolean().default(false).not_null())
                    .col(ColumnDef::new(Labels::Year).date())
                    .col(ColumnDef::new(Labels::GenreId).integer())
                    .foreign_key(sea_query::ForeignKey::create()
                        .name("genre_id")
                        .from(Labels::Table, Labels::GenreId)
                        .to(Genres::Table, Genres::GenreId)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Labels::Table).to_owned())
            .await
    }
}


#[derive(Iden)]
enum Labels {
    Table,
    LabelId,
    Name,
    Info,
    Owned,
    Year,
    GenreId
}
