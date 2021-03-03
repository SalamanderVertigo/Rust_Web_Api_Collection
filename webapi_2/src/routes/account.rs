use actix_web::{post, get, web, HttpResponse, Responder };
use crate::models::{User};
use sqlx::PgPool;

#[post("/create")]
async fn create(user: web::Json<User>, db_pool: web::Data<PgPool>,) -> impl Responder {
    println!("Create new user Controller, {:?}", user);
    let result = User::create(user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("Error trying to create new user"),
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
    cfg.service(create);
    cfg.service(account_test);
}