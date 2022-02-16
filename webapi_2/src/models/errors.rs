use actix_web::{error, get, http::header, http::StatusCode, App, HttpResponse, HttpResponseBuilder};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum MyError {
    #[display(fmt = "Invalid Token")]
    InvalidToken,
    #[display(fmt = "Expired Signature")]
    ExpiredSignature,
    #[display(fmt = "Invalid User")]
    InvalidUser,
    #[display(fmt = "Authentication Denied")]
    AuthenticationDenied,
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            MyError::InvalidToken => StatusCode::UNAUTHORIZED,
            MyError::InvalidUser => StatusCode::UNAUTHORIZED,
            MyError::ExpiredSignature => StatusCode::UNAUTHORIZED,
            MyError::AuthenticationDenied => StatusCode::UNAUTHORIZED
        }
    }
}