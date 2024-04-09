use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SongRequest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SongRequest::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SongRequest::CreatedAt).date_time().not_null().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(SongRequest::YandexId).integer().not_null())
                    .col(ColumnDef::new(SongRequest::SongName).string().not_null())
                    .col(ColumnDef::new(SongRequest::ArtistNames).string().not_null())
                    .col(ColumnDef::new(SongRequest::SayHello).boolean().not_null())
                    .col(ColumnDef::new(SongRequest::HelloFrom).string())
                    .col(ColumnDef::new(SongRequest::HelloTo).string())
                    .col(ColumnDef::new(SongRequest::HelloText).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SongRequest::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum SongRequest {
    Table,
    Id,
    CreatedAt,
    YandexId,
    SongName,
    ArtistNames,
    SayHello,
    HelloFrom,
    HelloTo,
    HelloText
}
