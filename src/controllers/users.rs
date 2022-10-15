use crate::{
    auth::Jwt,
    database::{
        images::{self, Image},
        users::{self, User},
        Database, SqlxErrorExtension,
    },
    macros::{auth, eresp, resp},
};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse,
};
use base64::decode;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct UserDto {
    #[validate(length(
        min = 3,
        max = 255,
        message = "Username must be between 3 and 255 characters"
    ))]
    username: String,
    #[validate(length(min = 8, message = "Password must be greater than 8 characters"))]
    password: String,
}

#[post("/create")]
pub async fn create_user(user: Json<UserDto>, database: Data<Database>) -> HttpResponse {
    let user = user.into_inner();
    if let Err(e) = user.validate() {
        return eresp!(HttpResponse::UnprocessableEntity(), e, "Invalid payload");
    }

    let password = match bcrypt::hash(user.password, 10) {
        Ok(password) => password,
        Err(error) => {
            log::error!("Failed to hash password: {}", error);
            return eresp!(HttpResponse::InternalServerError(); "Failed to hash password");
        }
    };

    let db_request = User {
        id: cuid::cuid().unwrap(),
        username: user.username,
        password,
        last_token: None,
        avatar_id: None,
    };

    match users::create_user(&*database, db_request).await {
        Ok(u) => resp!(HttpResponse::Ok(), u, "User created"),
        Err(err @ sqlx::Error::Database(_)) if err.get_mysql().number() == 1062 => {
            eresp!(HttpResponse::Conflict(); "Username already exists")
        }
        Err(err) => {
            log::error!("Failed to create user: {}", err);
            eresp!(HttpResponse::InternalServerError(); "Failed to create user")
        }
    }
}

#[post("/login")]
pub async fn login_user(
    login: Json<UserDto>,
    database: Data<Database>,
    jwt: Data<Jwt>,
) -> HttpResponse {
    let login = login.into_inner();
    let user = match users::get_by_username(&*database, login.username).await {
        Ok(user) => user,
        Err(_e @ sqlx::Error::RowNotFound) => {
            return eresp!(HttpResponse::Unauthorized(); "Wrong username or password")
        }
        Err(e) => {
            log::error!("Failed to get user: {}", e);
            return eresp!(HttpResponse::InternalServerError());
        }
    };

    match bcrypt::verify(login.password, &user.password) {
        Ok(true) => {
            let token = match jwt.create_jwt(user.id.clone()) {
                Ok(t) => {
                    if let Err(err) = users::add_last_token(&*database, t.clone(), user.id).await {
                        log::error!("Failed to add last token: {}", err);
                        return eresp!(HttpResponse::InternalServerError());
                    }
                    t
                }
                Err(e) => {
                    log::error!("Failed to create token: {}", e);
                    return eresp!(HttpResponse::InternalServerError());
                }
            };

            resp!(HttpResponse::Ok(), json!({ "token": token }))
        }
        Ok(false) => eresp!(HttpResponse::Unauthorized(); "Wrong username or password"),
        Err(e) => {
            log::error!("Failed to verify password: {}", e);
            eresp!(HttpResponse::InternalServerError())
        }
    }
}

#[derive(Deserialize, Validate, Debug)]
pub struct SetAvatarDto {
    #[validate(length(min = 3, max = 255, message = "Mime Type not set"))]
    mime_type: String,
    #[validate(length(min = 1, message = "Content not set"))]
    content: String,
}
#[post("/avatar")]
pub async fn set_avatar(
    req: HttpRequest,
    avatar: Json<SetAvatarDto>,
    database: Data<Database>,
) -> HttpResponse {
    let user = auth!(req);
    let avatar = avatar.into_inner();
    if let Err(e) = avatar.validate() {
        return eresp!(HttpResponse::UnprocessableEntity(), e, "Invalid payload");
    }

    if let Ok(image_content) = decode(avatar.content) {
        let image_id = images::add_image(
            &*database,
            Image {
                id: 0,
                content: image_content,
                mime_type: avatar.mime_type.clone(),
            },
        )
        .await
        .unwrap();
        users::set_avatar(&*database, user.id, image_id)
            .await
            .unwrap();
        resp!(HttpResponse::Ok(); "Avatar updated")
    } else {
        eresp!(HttpResponse::BadGateway(); "Not a valid base64")
    }
}

#[get("/{id}/avatar")]
pub async fn get_avatar(
    req: HttpRequest,
    database: Data<Database>,
    id: Path<String>,
) -> HttpResponse {
    let id = if id.to_string() == "@me" {
        auth!(req).id
    } else {
        id.to_string()
    };
    if let Ok(user) = users::get_by_id(&*database, id).await {
        if let Some(avatar_id) = user.avatar_id {
            let img_data = images::get_image(&*database, avatar_id).await.unwrap();
            HttpResponse::Ok()
                .content_type(img_data.mime_type)
                .body(img_data.content)
        } else {
            eresp!(HttpResponse::NotFound(); "User does not have an avatar")
        }
    } else {
        eresp!(HttpResponse::NotFound(); "User not found")
    }
}
