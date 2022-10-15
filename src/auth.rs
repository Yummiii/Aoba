use crate::{
    configs::Configs,
    database::{users, Database},
};
use actix_web::{web::Data, HttpRequest};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub id: String,
    exp: usize,
}
pub struct Jwt {
    secret: String,
}
impl Jwt {
    pub fn new(config: &Configs) -> Self {
        Self {
            secret: config.jwt_secret.clone(),
        }
    }

    pub fn create_jwt(&self, id: String) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(
            &Header::default(),
            &JwtClaims {
                id,
                exp: 2147483647,
            },
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }

    pub fn verify_jwt(&self, token: &str) -> Result<JwtClaims, jsonwebtoken::errors::Error> {
        let claims = jsonwebtoken::decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(claims.claims)
    }
}

#[derive(Deserialize)]
pub struct UserAuthentication {
    pub id: String,
}

pub async fn auth(req: HttpRequest) -> Result<UserAuthentication, String> {
    if let Some(token) = req.headers().get("Authorization") {
        if let Ok(token) = token.to_str() {
            let token = token.replace("Bearer ", "").trim().to_owned();
            let jwt = req.app_data::<Data<Jwt>>().unwrap();
            if let Ok(claims) = jwt.verify_jwt(&token) {
                let connection = req.app_data::<Data<Database>>().unwrap().clone();
                if let Ok(user_db) = users::get_by_token(&*connection, token).await {
                    if user_db.id == claims.id {
                        Ok(UserAuthentication { id: claims.id })
                    } else {
                        Err("Invalid token".into())
                    }
                } else {
                    Err("Error to get token in database".into())
                }
            } else {
                Err("Jwt verification error".into())
            }
        } else {
            Err("Token in invalid format".into())
        }
    } else {
        Err("Missing Authorization header".into())
    }
}
