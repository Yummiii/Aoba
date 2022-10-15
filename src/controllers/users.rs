use actix_web::{HttpRequest, HttpResponse, web::{Path, Data, Json}, get, post};
use base64::decode;
use serde::Deserialize;
use validator::Validate;

use crate::{database::{images::{self, ImageData}, Database, users}, extractors::UserAuthentication};

#[get("/{user_id}/images")]
pub async fn user_images(user_id: Path<(String,)>, database: Data<Database>, user: UserAuthentication) -> HttpResponse {
    let mut images = Vec::new();
    if user_id.0 == "@me" {
        images = images::get_user_images(&*database, user.id.clone()).await.unwrap();
    } else {
        images = images::get_user_public_images(&*database, user_id.0.clone()).await.unwrap();
    }

    HttpResponse::Ok().json(images)
}

#[derive(Deserialize, Validate, Debug)]
pub struct AvatarUpDto {
    #[validate(length(min = 1, message = "kd o avatar piranha?"))]
    content: String,
    #[validate(length(min = 1, message = "coloque o mime type vadia puta"))]
    mime_type: String
}

#[post("/avatar")]
pub async fn set_avatar(image: Json<AvatarUpDto>, database: Data<Database>, user: UserAuthentication) -> HttpResponse {
    if let Ok(img_content) = decode(image.content.clone()) {
        let image_id = images::add_image_data(&*database, ImageData {
            id: 0,
            mime_type: image.mime_type.clone(),
            content: img_content
        }).await.unwrap();
        users::set_avatar(&*database, user.id.clone(), image_id).await.unwrap();
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::BadRequest().json(serde_json::json!({
            "error": "not a valid base64"
        }))
    }
}


#[get("/{user_id}/avatar")]
pub async fn get_avatar(mut user_id: Path<(String,)>, database: Data<Database>) -> HttpResponse {
    if let Ok(user) = users::get_by_id(&*database, user_id.0.clone()).await {
        if let Some(avatar_id) = user.avatar_id {
            let img_data = images::get_image_data(&*database, avatar_id).await.unwrap();
            HttpResponse::Ok()
                .content_type(img_data.mime_type)
                .body(img_data.content)
        } else {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "user does not have an avatar"
            }))
        }
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "error": "user not found"
        }))
    }
}