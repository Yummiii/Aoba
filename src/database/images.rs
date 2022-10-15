use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};

use super::Database;

#[derive(FromRow, Deserialize)]
pub struct Image {
    pub id: i64,
    pub mime_type: String,
    pub content: Vec<u8>,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ImageInfo {
    pub id: String,
    pub user_id: String,
    #[serde(skip_serializing)]
    pub image_id: i64,
    pub public: bool,
    pub public_list: bool,
}

pub async fn add_image(connection: &Database, image: Image) -> Result<i64, sqlx::Error> {
    let result = query!(
        "insert into images (mime_type, content) values (?, ?)",
        image.mime_type,
        image.content
    )
    .execute(connection.get_pool())
    .await?;
    Ok(result.last_insert_id() as i64)
}

pub async fn get_image(connection: &Database, id: i64) -> Result<Image, sqlx::Error> {
    let img = query_as!(Image, "select * from images where id = ?", id)
        .fetch_one(connection.get_pool())
        .await?;
    Ok(img)
}