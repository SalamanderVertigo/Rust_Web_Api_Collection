#[macro_use]
extern crate log;
use actix_web::{App, HttpServer, Responder, HttpResponse, web, middleware,};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;
use sqlx::PgPool;
use std::env;
use webapi_2::appconfig::config_app;

// default / handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to this Rust Web API powered by ACTIX!
    "#
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::new(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(db_pool.clone())
            .route("/", web::get().to(index))
            .configure(config_app)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in .env file");
            let port = env::var("PORT").expect("PORT is not set in .env file");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await?;

    Ok(())
}