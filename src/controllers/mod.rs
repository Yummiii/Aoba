use actix_web::{Scope, web};

mod users;
mod images;

pub fn users_routes() -> Scope {
    web::scope("/users")
        .service(users::create_user)
        .service(users::login_user)
        .service(users::set_avatar)
        .service(users::get_avatar)
}

pub fn images_routes() -> Scope {
    web::scope("/images")
        .service(images::add_image)
        .service(images::get_image)        
}