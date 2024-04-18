use std::sync::Arc;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use crate::domain::entities::forbidden_artist::{Entity as ForbiddenArtistEntity, Model, Column};

#[derive(Clone)]
pub struct ForbiddenArtistRepository {
    connection: Arc<DatabaseConnection>
}

impl ForbiddenArtistRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        ForbiddenArtistEntity::find().all(self.connection.as_ref()).await
    }

    pub async fn find_first(&self, name: &String) -> Result<Option<Model>, DbErr> {
        ForbiddenArtistEntity::find()
            .filter(Column::Name.contains(name).or(Column::Summary.contains(name)))
            .one(self.connection.as_ref()).await
    }
}
