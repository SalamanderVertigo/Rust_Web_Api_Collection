use actix_web::{post, get, web, HttpResponse, Responder };
use crate::models::{User};
use sqlx::PgPool;
use uuid::Uuid;


#[post("/create")]
async fn create(user: web::Json<User>, db_pool: web::Data<PgPool>,) -> impl Responder {
    let result = User::create(user.into_inner(), db_pool.get_ref()).await;
    println!("Result: {:?}", result);
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("Error trying to create new user"),
    }
}

#[get("/get-account/{id}")]
async fn get(db_pool: web::Data<PgPool>, web::Path(id): web::Path<Uuid>) -> impl Responder {
    println!("get-account");
    let result = User::get(db_pool.get_ref(), id).await;
    println!("Result: {:?}", result);
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
    cfg.service(create);
    cfg.service(get);
    cfg.service(account_test);
}