use actix_web::{web, get, HttpResponse, Result};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
}

#[get("/{user_name}")]
pub async fn get_user(web::Path(user_name): web::Path<String>) -> Result<String> {
    Ok(format!("Welcome {:?}", user_name))
}