use actix_web::{post, get, web, HttpResponse, Responder };
use crate::models::{InternalUser, LoginRequest};
use sqlx::PgPool;


#[post("/register")]
async fn register(new_user: web::Json<InternalUser>, db_pool: web::Data<PgPool>,) -> impl Responder {
    let result = InternalUser::create(new_user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::BadRequest().body("Error trying to create new user"),
    }
}

#[post("/login")]
async fn login(db_pool: web::Data<PgPool>, verify_user: web::Json<LoginRequest>) -> impl Responder {
    let result = LoginRequest::login(db_pool.get_ref(), verify_user.into_inner()).await;
    match result {
        Ok(res) => HttpResponse::Ok().json(res),
        _ => HttpResponse::BadRequest().body("Error, incorrect login credentials"),
    }
}

// #[post("/logout")]
// async fn logout() {
// 
// }

#[get("/register-signin-test")]
async fn signup_signin_test() -> impl Responder {
    HttpResponse::Ok().body(r#"
        signup signin routes working
    "#
    )
}

pub fn signup_signin_init(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(signup_signin_test);
}
   