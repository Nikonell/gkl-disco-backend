//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "song_request")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: String,
    pub yandex_id: i32,
    pub song_name: String,
    pub artist_names: String,
    pub say_hello: bool,
    pub hello_from: Option<String>,
    pub hello_to: Option<String>,
    pub hello_text: Option<String>,
    pub explicit_correct: bool,
    pub artist_correct: bool,
    pub expert_mark: Option<bool>
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
