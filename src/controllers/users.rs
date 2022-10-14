use actix_web::{HttpRequest, HttpResponse, web::{Path, Data}, get};

use crate::{database::{images, Database}, extractors::UserAuthentication};

#[get("/{user_id}/images")]
pub async fn user_imaes(mut user_id: Path<(String,)>, database: Data<Database>, user: UserAuthentication) -> HttpResponse {
    let mut images = Vec::new();
    if user_id.0 == "@me" {
        images = images::get_user_images(&*database, user.id.clone()).await.unwrap();
    } else {
        images = images::get_user_public_images(&*database, user_id.0.clone()).await.unwrap();
    }

    HttpResponse::Ok().json(images)
}