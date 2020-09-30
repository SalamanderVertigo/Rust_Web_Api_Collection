use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
mod users_controllers;
mod config;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

struct AppState {
    app_name: String,
}

#[get("/data")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    format!("Hello {}!,", app_name)
}

struct AppstateWithCounter {
    counter: Mutex<i32>
}

async fn state_with_counter(data: web::Data<AppstateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("request number: {}", counter)
}


// this function could be located in a different module
// fn scoped_config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/test")
//             .route(web::get().to(|| HttpResponse::Ok().body("test")))
//             .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
//     );
// }

// this function could be located in a different module
// fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/users")
//             .route(web::get().to(|| HttpResponse::Ok().body("Users")))
//             .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
//     );
//     cfg.service(
//         web::resource("/stuff")
//             .route(web::get().to(|| HttpResponse::Ok().body("Stuff")))
//     );
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppstateWithCounter {
        counter: Mutex::new(0),
    });


    HttpServer::new(move || {
        App::new()
            .configure(config::config)
            .data(AppState {
                app_name: String::from("Actix-web-api"),
            })
            .app_data(counter.clone())
            .service(hello)
            .service(echo)
            .service(index)
            .service(users_controllers::get_user)
            .service(web::scope("/api").configure(users_controllers::user_routes)) // found in another file, create custome routes this way too
            .route("/hey", web::get().to(manual_hello))
            .route("/app_state_counter", web::get().to(state_with_counter))
            // .service(web::resource("/uses_container/")
            //     .route(web::get().to(users_controllers::get_user))
                // .route(web::post().to(post_handler))
                // .route(web::delete().to(delete_handler)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}