use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ForbiddenArtist::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ForbiddenArtist::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ForbiddenArtist::Name).string().not_null())
                    .col(ColumnDef::new(ForbiddenArtist::Summary).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ForbiddenArtist::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ForbiddenArtist {
    Table,
    Id,
    Name,
    Summary
}
