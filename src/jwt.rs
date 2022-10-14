use jsonwebtoken::{EncodingKey, DecodingKey, Validation, Header};
use serde::{Serialize, Deserialize};
use crate::config::Config;

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub id: String,
    exp: usize,
}

pub struct Jwt {
    secret: String,
}

impl Jwt {
    pub fn new(config: &Config) -> Self {
        Self {
            secret: config.jwt_secret.clone(),
        }
    }

    pub fn create_jwt(&self, id: String) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = JwtClaims {
            id,
            exp: 2147483647,
        };
        jsonwebtoken::encode(
            &Header::default(),
            &claims,
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
