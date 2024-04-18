pub use sea_orm_migration::prelude::*;

mod m20240409_085436_create_song_request_table;
mod m20240411_124501_create_access_token_table;
mod m20240417_053155_create_forbidden_artist;
mod m20240418_055748_add_song_request_marks;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240409_085436_create_song_request_table::Migration),
            Box::new(m20240411_124501_create_access_token_table::Migration),
            Box::new(m20240417_053155_create_forbidden_artist::Migration),
            Box::new(m20240418_055748_add_song_request_marks::Migration),
        ]
    }
}
