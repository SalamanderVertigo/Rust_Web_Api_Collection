use actix_web::{web, get, Result };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct User {
    user_id: u32,
    user_name: String,
}

pub struct FormData {
    user_info: String
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            
    );
}

#[get("/details/{user_name}/{user_id}")]
pub async fn get_user_details(info: web::Path<User>) -> Result<String> {
    Ok(format!("Hello {}, user_id {}", info.user_name, info.user_id))
}

#[get("/{user_name}")]
pub async fn get_user(web::Path(user_name): web::Path<String>) -> Result<String> {
    Ok(format!("Welcome {:?}", user_name))
}

#[get("/user/query")]
pub async fn get_query(info: web::Query<User>) -> Result<String> {
    println!("Query Request Handler");
    Ok(format!("Welcome {}!", info.user_name))
}

// use postman and send json object type USER in body
#[get("/user/json")]
pub async fn get_json(info: web::Json<User>) -> Result<String> {
    println!("Json Request Handler");
    Ok(format!("Hello {}!", info.user_name))
}