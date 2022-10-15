use actix_web::{Scope, web};

mod auth;
mod images;
mod users;

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .service(auth::sign_up)
        .service(auth::login)
}

pub fn images_routes() -> Scope {
    web::scope("/images")
        .service(images::upload_image)
        .service(images::get_image)
        .service(images::get_public_image)
}

pub fn users_routes() -> Scope {
    web::scope("/users")
        .service(users::user_images)
        .service(users::set_avatar)
        .service(users::get_avatar)
}