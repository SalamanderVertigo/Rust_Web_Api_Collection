use actix_web::{App, HttpServer, Responder, HttpResponse};

use webapi_2::appconfig::config_app;

// default / handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to this Rust Web API powered by ACTIX!
    "#
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config_app)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}