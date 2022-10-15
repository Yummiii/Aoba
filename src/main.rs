use actix_web::{middleware::Logger, web::{Data, JsonConfig}, App, HttpServer, HttpResponse, error::InternalError};
use auth::Jwt;
use configs::Configs;
use database::Database;
use dotenv::dotenv;
use macros::eresp;
use serde_json::Value;

mod auth;
mod configs;
mod controllers;
mod database;
mod macros;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let configs = Configs::get_config();
    let database = Database::new(&configs).await;
    database.migrate().await;

    let database_data = Data::new(database);
    let jwt = Data::new(Jwt::new(&configs));
    
    HttpServer::new(move || {
        App::new()
            .app_data(JsonConfig::default().error_handler(|err, _req| {
                InternalError::from_response("", eresp!(HttpResponse::BadRequest(), None::<Value>, err.to_string())).into()
            }))
            .app_data(database_data.clone())
            .app_data(jwt.clone())
            .wrap(Logger::default())
            .service(controllers::users_routes())
    })
    .bind(("0.0.0.0", configs.server_port))?
    .run()
    .await
}
