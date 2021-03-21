use actix_web::{post, get, web, HttpResponse, Responder };
use crate::models::{User, InternalUser, VerifyUser};
use sqlx::PgPool;
use uuid::Uuid;


#[post("/create")]
async fn create(new_user: web::Json<InternalUser>, db_pool: web::Data<PgPool>,) -> impl Responder {
    let result = InternalUser::create(new_user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("Error trying to create new user"),
    }
}

#[get("/get-account/{id}")]
async fn get(db_pool: web::Data<PgPool>, web::Path(id): web::Path<Uuid>) -> impl Responder {
    let result = User::get(db_pool.get_ref(), id).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("Error trying to fetch user information"),
    }
}

#[post("/login")]
async fn login(db_pool: web::Data<PgPool>, verify_user: web::Json<VerifyUser>) -> impl Responder {
    let result = VerifyUser::login(db_pool.get_ref(), verify_user.into_inner()).await;
    println!("Resutlt: {:?}", result);
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        _ => HttpResponse::BadRequest().body("Error, incorrect login credentials"),
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
    cfg.service(login);
    cfg.service(account_test);
}