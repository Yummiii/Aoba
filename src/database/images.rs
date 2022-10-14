use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use super::Database;

#[derive(FromRow, Deserialize)]
pub struct ImageData {
    pub id: i64,
    pub mime_type: String,
    pub content: Vec<u8>
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub user_id: String,
    #[serde(skip_serializing)]
    pub image_id: i64,
    pub public: bool,
    pub anonymous: bool,
    pub public_list: bool
}

pub async fn add_image_data(connection: &Database, image_data: ImageData) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("insert into ImagesData (mime_type, content) values (?, ?)")
        .bind(image_data.mime_type.clone())
        .bind(image_data.content)
        .execute(connection.get_pool())
        .await?;
    Ok(result.last_insert_id() as i64)
}

pub async fn add_image(connection: &Database, image: Image) -> Result<Image, sqlx::Error> {
    let result = sqlx::query("insert into Images (id, user_id, image_id, public, anonymous, public_list) values (?, ?, ?, ?, ?, ?)")
        .bind(&image.id)
        .bind(&image.user_id)
        .bind(&image.image_id)
        .bind(&image.public)
        .bind(&image.anonymous)
        .bind(&image.public_list)
        .execute(connection.get_pool())
        .await?;
    Ok(image)
}

pub async fn get_image(connection: &Database, id: String) -> Result<Image, sqlx::Error> {
    let result: Image = sqlx::query_as("select * from Images where id = ?")
        .bind(id)
        .fetch_one(connection.get_pool())
        .await?;
    Ok(result)
}

pub async fn get_image_data(connection: &Database, id: i64) -> Result<ImageData, sqlx::Error> {
    let result: ImageData = sqlx::query_as("select * from ImagesData where id = ?")
        .bind(id)
        .fetch_one(connection.get_pool())
        .await?;
    Ok(result)
}

pub async fn get_user_images(connection: &Database, id: String)  -> Result<Vec<Image>, sqlx::Error> {
    let result: Vec<Image> = sqlx::query_as("select * from Images where user_id = ?")
        .bind(id)
        .fetch_all(connection.get_pool())
        .await?;
    Ok(result)
}

pub async fn get_user_public_images(connection: &Database, id: String)  -> Result<Vec<Image>, sqlx::Error> {
    let result: Vec<Image> = sqlx::query_as("select * from Images where user_id = ? and public = true and public_list = true")
        .bind(id)
        .fetch_all(connection.get_pool())
        .await?;
    Ok(result)
}