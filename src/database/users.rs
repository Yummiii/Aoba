use super::Database;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, FromRow, query};

#[derive(FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub avatar_id: Option<i64>,
    #[serde(skip_serializing)]
    pub last_token: Option<String>,
}

pub async fn create_user(connection: &Database, user: User) -> Result<User, sqlx::Error> {
    query!("insert into users (id, username, password) values (?, ?, ?)", user.id, user.username, user.password).execute(connection.get_pool()).await?;
    Ok(user)
}

pub async fn add_last_token(connection: &Database, token: String, id: String) -> Result<(), sqlx::Error> {
    query!("update users set last_token = ? where id = ?", token , id).execute(connection.get_pool()).await?;
    Ok(())
}

pub async fn get_by_token(connection: &Database, token: String) -> Result<User, sqlx::Error> {
    let user = query_as!(User, "select * from users where last_token = ?", token)
        .fetch_one(connection.get_pool())
        .await?;
    Ok(user)
}

pub async fn get_by_username(connection: &Database, username: String) -> Result<User, sqlx::Error> {
    let user = query_as!(User, "select * from users where username = ?", username)
        .fetch_one(connection.get_pool())
        .await?;
    Ok(user)
}

pub async fn get_by_id(connection: &Database, id: String) -> Result<User, sqlx::Error> {
    let user = query_as!(User, "select * from users where id = ?", id)
        .fetch_one(connection.get_pool())
        .await?;
    Ok(user)
}

pub async fn set_avatar(connection: &Database, id: String, image_id: i64) -> Result<(), sqlx::Error> {
    query!("update users set avatar_id = ? where id = ?", image_id, id).execute(connection.get_pool()).await?;
    Ok(())
}