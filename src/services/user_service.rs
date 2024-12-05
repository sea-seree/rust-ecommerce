use crate::models::user::ActiveModel;
use crate::schemas::user_schema::RegisterRequest;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait};

pub async fn register_user(data: RegisterRequest, db: &DatabaseConnection) -> Result<(), String> {
    let new_user = ActiveModel {
        name: Set(data.name),
        email: Set(data.email),
        password_hash: Set(hash_password(data.password).unwrap()),
        ..Default::default()
    };

    new_user.insert(db).await.map_err(|e| e.to_string())
}
