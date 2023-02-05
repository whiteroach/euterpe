use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bands::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bands::BandId)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bands::Name).string().not_null())
                    .col(ColumnDef::new(Bands::Active).boolean().not_null().default(true))
                    .col(ColumnDef::new(Bands::Owned).boolean().default(false).not_null())
                    .col(ColumnDef::new(Bands::DeletedAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bands::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Bands {
    Table,
    BandId,
    Name,
    Active,
    Owned,
    DeletedAt
}
