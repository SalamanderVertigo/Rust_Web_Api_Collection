/*use std::fmt::{Display, Formatter};*/
/*use actix_web::{dev::HttpResponseBuilder, error::ResponseError, get, http::header, http::StatusCode, App, HttpResponse, HttpServer };
use std::convert::From;
use std::fmt::{Display, Formatter};
use sqlx::{error::DatabaseError};
use uuid::ParseError;*/
/*use derive_more::{Display, Error};*/
/*
#[derive(fail, Debug)]
pub enum ServiceError {
    #[fail("wrong credentials")]
    WrongCredentialsError,
    #[fail("jwt token not valid")]
    JWTTokenError,
    #[fail("jwt token creation error")]
    JWTTokenCreationError,
    #[fail("no auth header")]
    NoAuthHeaderError,
    #[fail("invalid auth header")]
    InvalidAuthHeaderError,
    #[fail("no permission")]
    NoPermissionError,
    #[fail(display = "BadRequest: {}", _0)]
    BadRequest(String),
}*/

/*#[derive(Serialize, Debug)]
struct ErrorResponse {
    message: String,
    status: String,
}*/


/*impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error, Please try later"),
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized")
        }
    }
}*/

/*#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}*/


//TODO WORK ON CREATING CUSTOM ERROR TYPES (2 types: visible to users, and not)