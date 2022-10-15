use actix_web::{Scope, web};

mod users;

pub fn users_routes() -> Scope {
    web::scope("/users")
        .service(users::create_user)
        .service(users::login_user)
        .service(users::set_avatar)
        .service(users::get_avatar)
}