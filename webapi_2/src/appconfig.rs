use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;
use crate::routes::{account_init};
use crate::auth;

// handle token checking in here, migrate routes which requrie auth function to auth init mod

pub fn config_app(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(auth::bearer_token_check);
    cfg.service(
        web::scope("/account")
        .wrap(auth)
        .configure(account_init)
    );   
}