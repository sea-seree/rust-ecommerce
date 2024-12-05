use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

// กำหนด Relationship และ Column type
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::cart::Entity")]
    Cart,
    #[sea_orm(has_many = "super::order::Entity")]
    Order,
}