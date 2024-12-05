use actix_web::{web, Scope};

pub fn user_routes() -> Scope {
    web::scope("/users")
        .route("/register", web::post().to(user::register_user))
        .route("/login", web::post().to(user::login_user))
}