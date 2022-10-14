use actix_web::{get, middleware::Logger, web::{Data, JsonConfig}, App, HttpResponse, HttpServer, Responder, error::InternalError};
use config::Config;
use database::Database;
use dotenv::dotenv;
use jwt::Jwt;

mod config;
mod controllers;
mod database;
mod jwt;
mod extractors;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let config = Config::get_config();
    let database = Database::new(&config).await;
    database.migrate().await;

    let database_data = Data::new(database);
    let jwt = Data::new(Jwt::new(&config));

    HttpServer::new(move || {
        App::new()
            .app_data(JsonConfig::default().error_handler(|err, _req| {
                InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .json(serde_json::json!({
                            "error": err.to_string()
                        })),
                )
                .into()
            }))
            .app_data(database_data.clone())
            .app_data(jwt.clone())
            .wrap(Logger::default())
            .service(controllers::auth_routes())
            .service(controllers::users_routes())
            .service(controllers::images_routes())
    })
    .bind(("0.0.0.0", config.server_port))?
    .run()
    .await
}
