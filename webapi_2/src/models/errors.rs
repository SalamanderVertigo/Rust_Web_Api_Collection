use actix_web::{ error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::{Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display,  Deserialize, Serialize)]
pub enum ServiceError {}

#[derive(Debug, Display,  Deserialize, Serialize)]
pub enum AuthError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError(String),

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Error: Unauthorized, {}", _0)]
    InvalidToken(String),

    ExpiredSignature(String),

    #[display(fmt = "Error UnAuthorized")]
    InvalidUser,

    #[display(fmt = "timeout")]
    Timeout,

    #[display(fmt = "Error UnAuthorized")]
    AuthenticationDenied,
}

// #[derive(Debug, Display,  Deserialize, Serialize)]
// pub struct AuthErrorResponse {
//     status: StatusCode,
//     message: &'static str
// }


impl ResponseError for AuthError {
    // fn status_code(&self) -> StatusCode {
    //     match *self {
    //         AuthError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
    //         AuthError::BadClientData => StatusCode::BAD_REQUEST,
    //         AuthError::Timeout => StatusCode::GATEWAY_TIMEOUT,
    //         AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
    //         AuthError::InvalidUser => StatusCode::UNAUTHORIZED,
    //         AuthError::ExpiredSignature => StatusCode::UNAUTHORIZED,
    //         AuthError::AuthenticationDenied => StatusCode::UNAUTHORIZED,
            
    //     }
    // }

    fn error_response(&self) -> HttpResponse {
        match *self {
            AuthError::InternalServerError(ref message) => HttpResponse::InternalServerError().json(message),
            AuthError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            AuthError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            AuthError::ExpiredSignature(ref message) => HttpResponse::Unauthorized().json(message),
            AuthError::InvalidToken(ref message) => HttpResponse::Unauthorized().json(message),
            AuthError::InvalidUser => HttpResponse::Unauthorized().json("Unauthorized: Invalid User"),
            AuthError::Timeout => HttpResponse::Unauthorized().json("TimeOut"),
            AuthError::AuthenticationDenied => HttpResponse::Unauthorized().json("Authentication Denied")
        }
    }
}