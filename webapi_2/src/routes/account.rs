use actix_web::{get, web, HttpResponse, Responder };
use crate::models::{User};
use sqlx::PgPool;
use uuid::Uuid;


#[get("/get-account/{id}")]
async fn get(db_pool: web::Data<PgPool>, web::Path(id): web::Path<Uuid>) -> impl Responder {
    let result = User::get(db_pool.get_ref(), id).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("Error trying to fetch user information"),
    }
}

#[get("/account-test")]
async fn account_test() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Account Controllers Test
    "#
    )
}

pub fn account_init(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
    cfg.service(account_test);
}