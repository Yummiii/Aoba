use crate::{
    database::{users, Database, images::{self, Image, ImageData}},
    extractors::UserAuthentication,
};
use actix_web::{post, web::{Data, Json, Path}, HttpResponse, get};
use base64::decode;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct ImageUpDto {
    #[validate(length(min = 1, message = "kd a image piranha?"))]
    content: String,
    #[validate(length(min = 1, message = "coloque o mime type vadia puta"))]
    mime_type: String,
    public: Option<bool>,
    anonymous: Option<bool>,
    public_list: Option<bool>,
}

#[post("/upload")]
pub async fn upload_image(image: Json<ImageUpDto>, database: Data<Database>, user: UserAuthentication) -> HttpResponse {
    if let Ok(img_content) = decode(image.content.clone()) {
        let image_id = images::add_image_data(&*database, ImageData {
            id: 0,
            mime_type: image.mime_type.clone(),
            content: img_content
        }).await.unwrap();

        let saved_data = images::add_image(&*database, Image {
            id: cuid::cuid().unwrap(),
            image_id,
            user_id: user.id,
            anonymous: image.anonymous.unwrap_or_default(),
            public_list: image.public_list.unwrap_or_default(),
            public: image.public.unwrap_or_default()
        }).await.unwrap();

        HttpResponse::Ok().json(saved_data)
    } else {
        HttpResponse::BadRequest().json(serde_json::json!({
            "error": "not a valid base64"
        }))
    }
}

#[get("/{id}")]
pub async fn get_image(id: Path<(String,)>, database: Data<Database>, user: UserAuthentication) -> HttpResponse {
    match images::get_image(&*database, id.0.clone()).await {
        Ok(img) => {
            if img.user_id == user.id {
                let img_data = images::get_image_data(&*database, img.image_id).await.unwrap();
                HttpResponse::Ok()
                    .content_type(img_data.mime_type)
                    .body(img_data.content)
            } else {
                HttpResponse::Unauthorized().finish()
            }
        },
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::NotFound().finish()
        }
    }
}

#[get("/public/{id}")]
pub async fn get_public_image(id: Path<(String,)>, database: Data<Database>) -> HttpResponse {
    match images::get_image(&*database, id.0.clone()).await {
        Ok(img) => {
            if img.public {
                let img_data = images::get_image_data(&*database, img.image_id).await.unwrap();
                HttpResponse::Ok()
                    .content_type(img_data.mime_type)
                    .body(img_data.content)
            } else {
                HttpResponse::Unauthorized().finish()
            }
        },
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::NotFound().finish()
        }
    }
}