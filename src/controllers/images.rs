use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, get,
};
use base64::decode;
use serde::Deserialize;
use validator::Validate;

use crate::{
    database::{
        images::{self, Image, ImageInfo},
        Database,
    },
    macros::{auth, eresp, resp},
};

#[derive(Deserialize, Validate, Debug)]
pub struct ImageUpDto {
    #[validate(length(min = 3, max = 255, message = "Mime Type not set"))]
    mime_type: String,
    #[validate(length(min = 1, message = "Content not set"))]
    content: String,
    public: Option<bool>,
    public_list: Option<bool>,
}

#[post("/upload")]
pub async fn add_image(
    req: HttpRequest,
    img: Json<ImageUpDto>,
    database: Data<Database>,
) -> HttpResponse {
    let user = auth!(req);
    let img = img.into_inner();
    if let Err(e) = img.validate() {
        return eresp!(HttpResponse::UnprocessableEntity(), e, "Invalid payload");
    }

    if let Ok(image_content) = decode(img.content) {
        let image_id = images::add_image(
            &*database,
            Image {
                id: 0,
                content: image_content,
                mime_type: img.mime_type.clone(),
            },
        )
        .await
        .unwrap();

        let info = images::add_image_info(
            &*database,
            ImageInfo {
                id: cuid::cuid().unwrap(),
                image_id,
                public: img.public.unwrap_or_default(),
                public_list: img.public_list.unwrap_or_default(),
                user_id: user.id,
            },
        )
        .await
        .unwrap();

        resp!(HttpResponse::Ok(), info, "Image uploaded")
    } else {
        eresp!(HttpResponse::BadGateway(); "Not a valid base64")
    }
}

#[get("/{id}")]
pub async fn get_image(req: HttpRequest, database: Data<Database>, id: Path<String>) -> HttpResponse {
    match images::get_image_info(&*database, id.to_string()).await {
        Ok(img) => {
            if img.public || auth!(req).id == img.user_id {
                let img_data = images::get_image(&*database, img.image_id).await.unwrap();
                HttpResponse::Ok()
                    .content_type(img_data.mime_type)
                    .body(img_data.content)
            }  else {
                eresp!(HttpResponse::Unauthorized(); "NaN coleguinha")
            }

        },
        Err(e) => {
            println!("{:?}", e);
            eresp!(HttpResponse::NotFound(); "Image not found")
        }
    }
}
