use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                .table(SongRequest::Table)
                .add_column(ColumnDef::new(SongRequest::ExplicitCorrect).boolean().not_null())
                .to_owned()
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                .table(SongRequest::Table)
                .add_column(ColumnDef::new(SongRequest::ArtistCorrect).boolean().not_null())
                .to_owned()
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                .table(SongRequest::Table)
                .add_column(ColumnDef::new(SongRequest::ExpertMark).boolean())
                .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                .drop_column(SongRequest::ExplicitCorrect)
                .to_owned()
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                .drop_column(SongRequest::ArtistCorrect)
                .to_owned()
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                .drop_column(SongRequest::ExpertMark)
                .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum SongRequest {
    Table,
    ExplicitCorrect,
    ArtistCorrect,
    ExpertMark
}