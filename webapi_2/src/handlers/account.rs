use actix_web::{Error, HttpResponse, Responder};

use crate::models::{User};

pub async fn get_user() -> impl Responder {
    User {
        id: 1,
        first_name: "Jon",
        last_name: "Gucciardi",
        user_name: "Jgucciardi218"
    }
}

pub async fn update_account() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}
