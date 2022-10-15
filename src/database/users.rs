use serde::Serialize;
use sqlx::FromRow;

use super::{Database, SqlxErrorExtension};

pub enum UsersErrors {
    DuplicateUsername,
    Generic(sqlx::Error),
}

#[derive(FromRow)]
pub struct Users {
    pub id: String,
    pub avatar_id: Option<i64>,
    pub username: String,
    pub password: String,
    pub last_token: Option<String>,
}

#[derive(Serialize)]
pub struct UsersDtoResult {
    pub id: String,
    pub username: String,
}

pub async fn クリエート(connection: &Database, user: Users) -> Result<UsersDtoResult, UsersErrors> {
    let result = sqlx::query("insert into Users (id, username, password, last_token) values (?, ?, ?, ?)")
        .bind(user.id.clone())
        .bind(user.username.clone())
        .bind(user.password)
        .bind(user.last_token)
        .execute(connection.get_pool())
        .await;

    match result {
        Ok(_) => Ok(UsersDtoResult {
            id: user.id,
            username: user.username,
        }),
        Err(err @ sqlx::Error::Database(_)) if err.get_mysql().number() == 1062 => {
            Err(UsersErrors::DuplicateUsername)
        }
        Err(err) => Err(UsersErrors::Generic(err)),
    }
}

#[derive(Debug)]
pub enum UsersGetByUsernameErrors {
    NotFound,
    Generic(sqlx::Error),
}

pub async fn get_by_username(
    connection: &Database,
    username: String,
) -> Result<Users, UsersGetByUsernameErrors> {
    let result: Users = sqlx::query_as("select * from Users where username = ?")
        .bind(username)
        .fetch_one(connection.get_pool())
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => UsersGetByUsernameErrors::NotFound,
            err => UsersGetByUsernameErrors::Generic(err),
        })?;

    Ok(result)
}

pub async fn get_by_id(
    connection: &Database,
    id: String,
) -> Result<Users, UsersGetByUsernameErrors> {
    let result: Users = sqlx::query_as("select * from Users where id = ?")
        .bind(id)
        .fetch_one(connection.get_pool())
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => UsersGetByUsernameErrors::NotFound,
            err => UsersGetByUsernameErrors::Generic(err),
        })?;

    Ok(result)
}

pub async fn add_last_token(
    connection: &Database,
    last_token: String,
    id: String,
) -> Result<(), sqlx::Error> {
    sqlx::query("update Users set last_token = ? where id = ?")
        .bind(last_token)
        .bind(id)
        .execute(connection.get_pool())
        .await?;
    Ok(())
}

pub async fn get_by_token(connection: &Database, token: String) -> Result<Users, sqlx::Error> {
    let user: Users = sqlx::query_as("select * from Users where last_token = ?")
        .bind(token)
        .fetch_one(connection.get_pool())
        .await?;

    Ok(user)
}

pub async fn set_avatar(connection: &Database, id: String, avatar_id: i64) -> Result<(), sqlx::Error>{
    sqlx::query("update Users set avatar_id = ? where id = ?")
        .bind(avatar_id)
        .bind(id)
        .execute(connection.get_pool())
        .await?;
    Ok(())
}