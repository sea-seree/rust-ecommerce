use actix_web::{web, HttpResponse};
use crate::schemas::user_schema::{UserResponse,LoginRequest, RegisterRequest};
// use crate::schemas::user_schema::{UserResponse, LoginRequest, RegisterRequest};
use crate::services::user_service;

pub async fn register_user(data: web::Json<RegisterRequest>) -> HttpResponse {
        if let Err(errors) = data.validate() {
            return HttpResponse::BadRequest().json(errors);
        }

        match user_service::register_user(data.into_inner()).await {
            Ok(user) => HttpResponse::Ok().json(UserResponse {
                id: user.id,
                username: user.username,
                email: user.email,
                created_at: user.created_at.to_rfc3339(),
            }),
            Err(err) => HttpResponse::InternalServerError().json({"error"; err.to_string()}),
        }
}


pub async fn login_user(data: web::Json<LoginRequest>) -> HttpResponse {
        if let Err(errors) = data.validate() {
            return HttpResponse::BadRequest().json(errors);
        }
    match user_service::authenticate_user(data.into_inner()).await {
        Ok(token) => HttpResponse::Ok().json({"token"; token}),
        Err(_) => HttpResponse::Unauthorized().json({"error"; "Invalid credentials"}),
    }
}