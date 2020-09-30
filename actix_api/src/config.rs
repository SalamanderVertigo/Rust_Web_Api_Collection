use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(|| HttpResponse::Ok().body("Users")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
    cfg.service(
        web::resource("/stuff")
            .route(web::get().to(|| HttpResponse::Ok().body("Stuff")))
    );
}