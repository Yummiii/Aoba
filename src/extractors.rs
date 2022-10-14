use std::{fmt::Display, pin::Pin};
use actix_web::{
    http::StatusCode, web::Data, FromRequest, HttpResponse, HttpResponseBuilder, ResponseError,
};
use futures::Future;
use serde::Deserialize;
use crate::{
    database::{
        users, Database,
    },
    jwt::{Jwt, JwtClaims},
};

#[derive(Debug)]
pub struct ExtractorError<'a> {
    status: StatusCode,
    message: &'a str,
}

impl<'a> Display for ExtractorError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> ResponseError for ExtractorError<'a> {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponseBuilder::new(self.status).json(serde_json::json!({ "message": self.message }))
    }
}

impl<'a> ExtractorError<'a> {
    fn new(status: StatusCode, message: &'a str) -> Self {
        Self { status, message }
    }
}

#[derive(Deserialize)]
pub struct UserAuthentication {
    pub id: String,
}

impl FromRequest for UserAuthentication {
    type Error = ExtractorError<'static>;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let sync_result = || {
            let header = req.headers().get("Authorization").ok_or_else(|| {
                ExtractorError::new(StatusCode::UNAUTHORIZED, "Missing authorization header")
            })?;

            let token = header
                .to_str()
                .map_err(|_| ExtractorError::new(StatusCode::UNAUTHORIZED, "Invalid token format"))?
                .replace("Bearer ", "");

            let jwt = req.app_data::<Data<Jwt>>().unwrap();

            let claims = jwt.verify_jwt(&token).map_err(|err| {
                log::error!("Jwt verification error: {}", err);
                ExtractorError::new(StatusCode::UNAUTHORIZED, "Jwt verification error")
            })?;

            Ok((token, claims)) as Result<(String, JwtClaims), ExtractorError>
        };

        let token_result = sync_result();
        let connection = req.app_data::<Data<Database>>().unwrap().clone();

        Box::pin(async move {
            let (token, claims) = token_result?;
            let user_token_response =
                users::get_by_token(&*connection, token)
                    .await
                    .map_err(|err| {
                        log::error!("Error to get token in database, {}", err);
                        ExtractorError::new(
                            StatusCode::UNAUTHORIZED,
                            "Error to get token in database",
                        )
                    })?;

            if user_token_response.id != claims.id {
                return Err(ExtractorError::new(
                    StatusCode::UNAUTHORIZED,
                    "Invalid token",
                ));
            }

            let authentication = UserAuthentication {
                id: user_token_response.id,
            };

            Ok(authentication)
        })
    }
}