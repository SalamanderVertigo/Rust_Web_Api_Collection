use actix_web::web;

use crate::handlers::{account};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .service(
                web::resource("")
                    .route(web::get().to(account::get_user))
                    .route(web::put().to(account::update_account))
            )
    );   
}