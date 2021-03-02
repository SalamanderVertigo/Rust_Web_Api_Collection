use actix_web::{Responder, Error, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize };
use futures::future::{ready, Ready};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub user_name: &'static str
}

impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
        )
    }
}
